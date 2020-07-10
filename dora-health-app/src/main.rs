
use failure::Error;
use getopts::Options;
use kubos_app::*;
use log::*;
use std::io;
use std::net::UdpSocket;
use std::time::{Duration};
use chrono::{Datelike, Timelike, Utc};
mod system_info;


fn save_parameter(serv: &ServiceConfig, param: &str, val: &String) -> Result<(), Error> {
    let request = format!(
        r#"
            mutation {{
                insert(subsystem: "OBC", parameter: "{}", value: "{}") {{
                    success,
                    errors
                }}
            }}
        "#, 
        param, val );

    match query(serv, &request, Some(Duration::from_secs(1))) {
        Ok(msg) => {
            println!("{}", msg);
            let success = msg
                .get("insert")
                .and_then(|data| data.get("success").and_then(|val| val.as_bool()));

            if success == Some(true) {
                debug!("Parameter [{}] saved to database", param);
                return Ok(())
            } else {
                match msg.get("errors") {
                    Some(errors) => {
                        error!("Failed to save value to database: {}", errors);
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, 
                            format!("Failed to save value to database: {}", errors)))?;
                    },

                    None => return Err(io::Error::new(io::ErrorKind::InvalidInput, 
                            "Failed to save value to database errors"))?
                };
            }
        },

        Err(err) => {
            error!("Telemetry service mutation failed: {}", err);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, 
                            format!("Telemetry service mutation failed: {}", err)))?
        }
    }
}



fn transmit_beacon(data: &str, service: &str) -> io::Result<usize> {

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    //let service = format!("127.0.0.1:{}", downlink_endpoint_udp);

    println!("Transmit beacon through service: {}", service);
    socket.send_to(data.as_bytes(), service)
}





fn main() -> Result<(), Error> {

	// Set up logging
    logging_setup!("dora-health-app", log::LevelFilter::Info)?;

    // Set up the recognized command line arguments
    let args: Vec<String> = ::std::env::args().collect();
    let mut opts = Options::new();

    opts.optflagopt("c", "config", 
        "Configuration file for connecting to KubOS services.  Default is /etc/kubos-config.toml",
        "CONFIG");
    opts.optflag("h", "help", "Print this help menu");
    opts.optflagopt("s", "save", 
        "Save the health information to the KubOS telemetry service database",
        "SAVE");
    opts.optflagopt("t", "transmit", 
        "Transmit health through KubOS communications system",
        "TRANSMIT");

    let matches = match opts.parse(&args) {
        Ok(r) => r,
        Err(_) => {
            error!("Abort.  Could not parse command line");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Could not parse command line"))?
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", &args[0]);
        print!("{}", opts.usage(&brief));
        return Ok(())
    }

    let save = if matches.opt_present("s") {true} else {false};
    let transmit = if matches.opt_present("t") {true} else {false};

	// ---------------------------------------------------------
	// Health status information
	// ---------------------------------------------------------

    info!("Getting health status info");

	// Get up time
	let uptime = system_info::up_time().unwrap_or_else( |_| {
                    println!("Failed to get uptime"); 
                    system_info::Uptime{up:0.0, idle:0.0} } );
	println!("Up time: {}", uptime.up);

	// Get CPU usage 
	let cpu_use_percent = system_info::cpu_usage(1).unwrap_or_else( |_| {
                            println!("Failed to get CPU usage");
                            0.0 } );
    println!("CPU usage: {:.2}%", cpu_use_percent );

	// Get memory usage 
	// (we'll do it ourselves rather than query the KubOS monitor service)
	let meminfo = system_info::mem_info().unwrap_or_else( |_| {
                    println!("Failed to get memory info");
                    system_info::Meminfo{total:0, free:0, available:0, use_percent:0.0} } );
    println!("Memory usage: {:.2}%", meminfo.use_percent);

	// Get disk usage
    let disks = system_info::disk_usage_all().unwrap_or_else ( |_| {
                    warn!("Failed to get disk usage");
                    Vec::<system_info::Diskinfo>::new() } );

    let root = system_info::find_mount(&disks, "/").unwrap_or_else( |_| {
                    warn!("Failed to get /boot disk usage info");
                    system_info::Diskinfo{
                        filesystem:"".to_string(), 
                        total:0, 
                        used:0, 
                        available:0, 
                        use_percent:0.0, 
                        mounted_on:"".to_string()} } );

    println!("/: {}", root);

    let upgrade = system_info::find_filesystem(&disks, "/dev/sda3").unwrap_or_else( |_| {
                    warn!("Failed to get /boot disk usage info");
                    system_info::Diskinfo{
                        filesystem:"".to_string(), 
                        total:0, 
                        used:0, 
                        available:0, 
                        use_percent:0.0, 
                        mounted_on:"".to_string()} } );

    println!("/dev/sda3: {}", upgrade);


    let sd = system_info::find_filesystem(&disks, "/dev/sda1").unwrap_or_else( |_| {
                    warn!("Failed to get /boot disk usage info");
                    system_info::Diskinfo{
                        filesystem:"".to_string(), 
                        total:0, 
                        used:0, 
                        available:0, 
                        use_percent:0.0, 
                        mounted_on:"".to_string()} } );

    println!("/dev/sda1: {}", sd);

    let home = system_info::find_filesystem(&disks, "/dev/sda3").unwrap_or_else( |_| {
                    warn!("Failed to get /boot disk usage info");
                    system_info::Diskinfo{
                        filesystem:"".to_string(), 
                        total:0, 
                        used:0, 
                        available:0, 
                        use_percent:0.0, 
                        mounted_on:"".to_string()} } );

    println!("/dev/sda3: {}", home);
  

    // Save the amount to the telemetry database
    if save {

        info!("Storing health status");

        match ServiceConfig::new("telemetry-service") {

            Ok(t) => {
        
                match ( save_parameter(&t, "uptime", &uptime.up.to_string()),
                        save_parameter(&t, "mem_usage", &meminfo.use_percent.to_string()),
                        save_parameter(&t, "cpu_usage", &cpu_use_percent.to_string()),
                        save_parameter(&t, "disk_root_usage", &root.use_percent.to_string()),
                        save_parameter(&t, "disk_home_usage", &home.use_percent.to_string()),
                        save_parameter(&t, "disk_sd_usage", &sd.use_percent.to_string()),
                        save_parameter(&t, "disk_upgrade_usage", &upgrade.use_percent.to_string()) ) {

                    ( Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_) ) => { },
             
                    _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to save telemetry"))?

                }
            },

            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to find telemetry database service"))?
        }
    }

    if transmit {

        match ServiceConfig::("dora-radio-service") {

            Ok(s) = > {

                // Get the current time for the timestamp in the beacon
                let now = Utc::now();

                // Form the beacon data (will be wrapped in CCSDS space packet by the radio service)
                let byte_string = format!("{},{:012.1},{:05.1},{:05.1},{:05.1},{:05.1},{:05.1},{:05.1}", 
                    now.format("%Y-%m-%dT%H:%M:%S"),
                    uptime.up,
                    meminfo.use_percent,
                    cpu_use_percent,
                    root.use_percent,
                    home.use_percent,
                    sd.use_percent,
                    upgrade.use_percent );

                info!("Transmitting health beacon: {}", byte_string);

                let raw = s.raw();
                let ip = raw["addr"]["ip"].as_str().unwrap();
                let port = 8161;
                let service = format!("{}:{}", ip, port);

                match transmit_beacon(&*byte_string, service) {

                    Ok(len) => { 
                        println!("Bytes transmitted: {}", len);
                    },

                    Err(err) => {
                        error!("Failed to send health beacon to radio: {}", err);
                        Err(io::Error::new(io::ErrorKind::NotConnected, 
                            format!("Failed to send health beacon to radio: {}", err)))?
                    }
                }
            },

            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to find radio service"))?
        }
        
    }

    Ok(())
}
