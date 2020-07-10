use comms_service::CommsTelemetry;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{Read, Write};
use base64::{encode, decode};
use std::process::Command;
use log::*;

#[derive(Clone)]
pub struct Subsystem {
    telem: Arc<Mutex<CommsTelemetry>>,
}

impl Subsystem {

    pub fn new(telem: Arc<Mutex<CommsTelemetry>>) -> Subsystem {
        Subsystem { telem }
    }


    // FALLBACK CONTINGENCY:  In the event of an anomaly where only the radio service 
    // is running, we want to have basic capability to interact with the operating 
    // system without needing to communicate with other services.  The following three 
    // functions provide this minimum functionality:
    // 
    // - run_command
    // - downlink_file
    // - uplink_file
    //
    // In the future, we may also want to include a keep_alive command that must be
    // received from the ground once per ~week or else the computer is made to 
    // reboot by the radio service.
    //
    //
    // run_command
    //
    // Runs a system command specified by a path and optional command line arguments.
    // The path may be absolute or may be a command name that can be found through the 
    // user PATH environment for the root user.  The option stdout and stderr arguments
    // can be used to pipe the output to specified files paths, otherwise, the output
    // will be returned in the resulting JSON file from the service.
    pub fn run_command(&self,   path: Option<String>, 
                                args: Option<Vec<String>>,
                                stdout: Option<String>,
                                stderr: Option<String>) -> Result<String, String> {

        // Build the command 
        let p = path.ok_or("No command path specified".to_owned())?;
        let mut command = Command::new(p);

        // Pipe the stdout output to a file if one is provided
        stdout.map(|s| { 
            File::create(s)
                .and_then(|f| Ok(command.stdout(f)))
                .or_else(|_| Err(info!("Failed to open stdout file")))
        });

        // Pipe the stderr output to a file if one is provided         
        stderr.map(|s| { 
            File::create(s)
                .and_then(|f| Ok(command.stderr(f)))
                .or_else(|_| Err(info!("Failed to open stderr file")))
        });

        // Set the command arguments if any are provided
        args.map(|a| command.args(a));

        // Execute the command
        command.output()
            .map(|s| format!("stdout: {}\n\nstderr: {}", 
                String::from_utf8_lossy(&s.stdout),
                String::from_utf8_lossy(&s.stderr)))
            .map_err(|_| "Could not execute command".to_owned())       
    }

    // download_file
    //
    // Download a file from the computer running the radio service.  The file can 
    // be optionally encoded in base64 before it is inserted into the returned
    // JSON response as a string (this is useful for binary files).
    pub fn download_file(&self, path: Option<String>, enc: Option<bool>) -> Result<String, String> {
        
        // Open the whole file or exit with error
        let p = path.ok_or("No file path specified".to_owned())?;
        let mut f = File::open(p).map_err(|_| "Failed to open file".to_owned())?;

        // Read the entire contents of the file or exit with error
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).map_err(|_| "Failed to read file".to_owned())?;

        // Encode to base64 if binary flag is set
        let mut do_encode = false;
        enc.and_then(|b| Some(do_encode = b));

        match do_encode {
            true => Ok(encode(buffer)),
            false => Ok(String::from_utf8_lossy(&buffer).to_string()) 
        }
    }


    // upload_file
    //
    // Upload a file to the computer running the radio service.  The file data
    // is included in the GraphQL query and will be written in the file path 
    // specified.  Optionally, the data can be decoded from base64 before it is
    // written to the file (this is useful for binary files).
    pub fn upload_file(&self, path: Option<String>, dec: Option<bool>, data: Option<String>) -> Result<String, String> {
        
        // Decode the uploaded data if necessary
        let d = data.ok_or("No data for file".to_owned())?;
        let data_vec = if dec.unwrap_or(false) { 
            decode(d).map_err(|_| "Failed to decode data".to_owned())?
        } else {
            d.as_bytes().to_vec()
        };

        // Open the new file for writing
        let p = path.ok_or("No file path specified".to_owned())?;
        let mut f = File::create(p).map_err(|_| "Failed to create new file".to_owned())?;

        // Write data to the file
        f.write_all(&data_vec)
            .and_then(|_| Ok("Wrote data to file".to_owned()))
            .map_err(|_| "Failed to write data to file".to_owned())            
    }



    // The following functions are copied from the KubOS radio service tutorial

    pub fn failed_packets_up(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.failed_packets_up),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn failed_packets_down(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.failed_packets_down),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn packets_up(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.packets_up),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn packets_down(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.packets_down),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn errors(&self) -> Result<Vec<String>, String> {
        match self.telem.lock() {
            Ok(data) => {
                Ok(data.errors.to_owned())
            }
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }
}