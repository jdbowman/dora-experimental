# Task list descriptions for CONOPS modes

This folder contains the JSON files that contain the task list descriptions for each of the operational modes for the spacecraft.  Operational modes are implemented in KubOS using the scheduler service.  More about using the scheduler can be found at: https://docs.kubos.com/1.21.0/tutorials/schedule-app.html.  Details of the task specifications are given at: https://docs.kubos.com/1.21.0/ecosystem/services/scheduler.html#schedule-specification.

## Operational Modes

Our experimental DORA setup has two CONOPS modes: 
1. safe:  Conducts no operations
1. idle:  Runs the dora-health-app once every 30 seconds to collect basic information about the system and transmit it and/or save it to the telemetry database.

Each mode is registered with the scheduler service (port 8010) by sending it the following GraphQL: 
```
mutation { 
  createMode(name: "idle") { 
    success
    errors 
  } 
} 
```
After registering a mode, the next step is to associate a task list with the mode.  Do this by sending the scheduler service the path to task list descriptor file and the mode to associate it with:
```
mutation { 
  importTaskList( 
    name: "idle-mode", 
    path: "/home/vagrant/dora-experimental/tasks/dora-idle-mode.json", 
    mode: "idle"  ) 
    { 
      success 
      errors 
    } 
} 
```
The last step is to active the mode by sending the scheduler service the following:
```
mutation { 
  activateMode(
    name: "idle") {
      success 
      errors 
  } 
} 
```



