# Health status and beacon

This app is intended to be run regularly (e.g. every 30 seconds) during nominal operations.  It queries the operating system for basic system health information, including uptime, memory usage, processor load, and disk usage.  Future updates will add additional health information.  

## Command line arguments

The following command line arguments are implements:
* -c, --config: Passed through to the KubOS service framework in order to allow custom configuration files to be used.
* -h, --help: prints the usage help screen
* -s, --save: Saves the health information in the telemetry service database under the OBC subsystem with parameters:
  * uptime:  Time in seconds since the process started
  * mem_usage: Current memory usage as a percentage of total capacity
  * cpu_usage: Current processor load (non-idle cycles) as a percentage of total cycles (includes all cores)
  * disk_root_usage: Use percentage of the disk supporting the / (root) directory
  * disk_home_usage: Use percentage of the disk supporting the /home/system user directory
  * disk_sd_usage: Use percentage of the external micro-SD card
  * disk_upgrade_usage: Use percentage of the disk hosting /upgrade
