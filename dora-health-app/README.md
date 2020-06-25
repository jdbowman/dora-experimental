# Health status and beacon

This app is intended to be run regularly (e.g. every 30 seconds) during nominal operations.  It queries the operating system for basic system health information, including uptime, memory usage, processor load, and disk usage.  This information is acquired through queries to the /proc filesystem and execuation of "df".  Future updates will add additional health information.  The app is implemented in Rust using the KubOS app framework.

## Command line arguments

The following command line arguments are implements:
* -c, --config: Passed through to the KubOS service framework in order to allow custom configuration files to be used.
* -h, --help: prints the usage help screen
* -s, --save: Saves the health information in the telemetry service database under the OBC subsystem with parameters:
  * uptime:  Time in seconds since the processor started
  * mem_usage: Current memory usage as a percentage of total capacity
  * cpu_usage: Current processor load (non-idle cycles) as a percentage of total cycles (includes all cores)
  * disk_root_usage: Use percentage of the disk supporting the / (root) directory
  * disk_home_usage: Use percentage of the disk supporting the /home/system user directory
  * disk_sd_usage: Use percentage of the external micro-SD card
  * disk_upgrade_usage: Use percentage of the disk hosting /upgrade
* -t, --transmit: [Not yet implemented]  Sends a CCSDS packet containing the health information through the KubOS communications layer for transmission

## Registering the app

This app is intended to be run through the KubOS app service so that it can be used by the scheduler.  Information about registering an app can be found at: https://docs.kubos.com/1.21.0/tutorials/app-register.html.  To register this app with the app service, follow these steps:

1. Make a folder on the target device and copy the app executable and its manifest.toml file to the folder.
1. Send the following GraphQL to the app service (port 8000):
```
mutation { 
  register(path: "/home/vagrant/dora-experimental/0.1") { 
    success, 
    errors, 
    entry { app { name executable } } 
  } 
} 
```
In the above example, we have assumed that the app executable and manifest.toml file have been copied to the folder /home/vagrant/dora-experimental/0.1.

## Running the app

This app will primarily be run automatically using the KubOS scheduler service.  See the `tasks` folder in this repository for a description of the scheduler.  This app is included in the `idle` mode task list.

For testing or running the app outside of the scheduler service, send the app service the following:
```
mutation { 
  startApp(name: “dora-health-app", args: “--save”) { 
    success, 
    pid 
  } 
} 
```

## Finding registered apps

To get a list of all registered apps, send the app service the following:
```
{
  registeredApps {
    active
    app {
      name
      version
    }
  }
}
```
Or to check on this app specficially, send:
```
{
  registeredApps(name: "dora-health-app") {
    active
    app {
      name
      version
    }
  }
}
```
