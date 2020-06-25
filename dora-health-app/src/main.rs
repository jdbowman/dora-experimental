
use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;
use log::*;
use std::time::Duration;
use std::io;
mod system_info;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() -> Result<(), Error> {

	// Setup logging
    logging_setup!("dora-health-app", log::LevelFilter::Info)?;

    // Set up the recognized command line arguments
    let args: Vec<String> = ::std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    // First, the KubOS required arguments
    opts.optflagopt(
        "c",
        "config",
        "System config file which should be used",
        "CONFIG",
    );

    // Now our custom app arguments:
    opts.optflag("h", "help", "Print this help menu");
    opts.optflagopt("s", "cmd_string", "Subcommand", "CMD_STR");
    opts.optflagopt("t", "cmd_sleep", "Safe-mode sleep time", "CMD_INT");

    // Parse the command args
    let matches = match opts.parse(args) {
        Ok(r) => r,
        Err(f) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Could not parse command line"))?
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(())
    }

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
    	let cpu_use = system_info::cpu_usage(1).unwrap_or_else( |_| {
                        println!("Failed to get CPU usage");
                        0.0 } );
    println!("CPU usage: {:.2}%", cpu_use );

	// Get memory usage 
	// (we'll do it ourselves rather than query the KubOS monitor service)
	let meminfo = system_info::mem_info().unwrap_or_else( |_| {
                    println!("Failed to get memory info");
                    system_info::Meminfo{total:0, free:0, available:0, use_percent:0.0} } );
    println!("Memory usage: {:.2}%", meminfo.use_percent);

	// Get disk usage
    let disks = system_info::disk_usage_all().unwrap_or_else ( |_| {
                    println!("Failed to get disk usage");
                    Vec::<system_info::Diskinfo>::new() } );

    let boot = system_info::disk_usage_by_mount("/boot").unwrap_or_else( |_| {
                    println!("Failed to get /boot disk usage info");
                    system_info::Diskinfo{
                        filesystem:"".to_string(), 
                        total:0, 
                        used:0, 
                        available:0, 
                        use_percent:0.0, 
                        mounted_on:"".to_string()} } );

    println!("/boot: {}", boot);

    let sda3 = system_info::disk_usage_by_filesystem("/dev/sda3").unwrap_or_else( |_| {
                    println!("Failed to get /boot disk usage info");
                    system_info::Diskinfo{
                        filesystem:"".to_string(), 
                        total:0, 
                        used:0, 
                        available:0, 
                        use_percent:0.0, 
                        mounted_on:"".to_string()} } );

    println!("/dev/sda3: {}", sda3);

    info!("Storing health status");

    // Collect telemetry
    let monitor_service = ServiceConfig::new("monitor-service")?;
    let telemetry_service = ServiceConfig::new("telemetry-service")?;

    // Get the amount of memory currently available on the OBC
    let request = "{memInfo{available}}";
    let response = match query(&monitor_service, request, Some(Duration::from_secs(1))) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Monitor service query failed: {}", err);
            bail!("Monitor service query failed: {}", err);
        }
    };

    let memory = response.get("memInfo").and_then(|msg| msg.get("available"));

    // Save the amount to the telemetry database
    if let Some(mem) = memory {
        let request = format!(
            r#"
            mutation {{
                insert(subsystem: "OBC", parameter: "available_mem", value: "{}") {{
                    success,
                    errors
                }}
            }}
        "#,
            mem
        );

        match query(&telemetry_service, &request, Some(Duration::from_secs(1))) {
            Ok(msg) => {
                let success = msg
                    .get("insert")
                    .and_then(|data| data.get("success").and_then(|val| val.as_bool()));

                if success == Some(true) {
                    info!("Current memory value saved to database");
                } else {
                    match msg.get("errors") {
                        Some(errors) => {
                            error!("Failed to save value to database: {}", errors);
                            bail!("Failed to save value to database: {}", errors);
                        }
                        None => {
                            error!("Failed to save value to database");
                            bail!("Failed to save value to database");
                        }
                    };
                }
            }
            Err(err) => {
                error!("Telemetry service mutation failed: {}", err);
                bail!("Telemetry service mutation failed: {}", err);
            }
        }
    }
    

    Ok(())
}
