
use std::thread;
use std::time::Duration;
use std::{io, fs};
use std::io::Read;
use std::process::Command;
use std::ops::Sub;
use std::fmt;

// Copied from systemstat crate
fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_string(&mut s))
        .map(|_| s)
}


fn not_numeric(c: char) -> bool { 
	!c.is_numeric() 
}

// Result of an up_time call. Values are in seconds.
#[derive(Debug)]
pub struct Uptime {
	pub up: f32,
	pub idle: f32
}

// Return an Uptime structure based on contents of /proc/uptime
pub fn up_time() -> io::Result<Uptime> {
    read_file("/proc/uptime").and_then(|data| {
    	
		let vec: Vec<&str> = data.trim().split_whitespace().collect();
		if vec.len() < 2 {
			Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect number of fields in /proc/uptime"))
		} else {
				
			match (	vec[0].replace(not_numeric, "").parse::<f32>(),
					vec[1].replace(not_numeric, "").parse::<f32>() ) {

				(Ok(up), Ok(idle)) => Ok( Uptime{up: up, idle: idle} ),

				_W => Err(io::Error::new(io::ErrorKind::InvalidData,"Could not parse fields in /proc/uptime"))
			}
		} 			
    })
}


// Result of a cpu_time call.  Values should be in hundredths of a second (TBD).
#[derive(Debug)]
pub struct CPUtime {
	pub user: i32,
	pub nice: i32,
	pub system: i32,
	pub idle: i32,
	pub iowait: i32,
	pub irq: i32,
	pub softirq: i32
}


impl Sub for CPUtime {
	type Output = CPUtime;

	fn sub(self, other: CPUtime) -> CPUtime {
		CPUtime { 
			user: self.user - other.user,
			nice: self.nice - other.nice,
			system: self.system - other.system,
			idle: self.idle - other.idle,
			iowait: self.iowait - other.iowait,
			irq: self.irq - other.irq,
			softirq: self.softirq - other.softirq
		}
	}
}



pub fn cpu_time() -> io::Result<CPUtime> {
    read_file("/proc/stat").and_then(|data| {

    	data.lines()
    		.find(|line| line.starts_with("cpu "))
    		.ok_or(io::Error::new(io::ErrorKind::InvalidData, "Could not find cpu in /proc/stat"))
    		.and_then(|line| {
    			
    			let vec: Vec<&str> = line.trim().split_whitespace().collect();
    			if vec.len() < 8 {
    				Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect number of fields in cpu line of /proc/stat"))
    			} else {

	    			match (	vec[1].replace(not_numeric, "").parse::<i32>(),
	    					vec[2].replace(not_numeric, "").parse::<i32>(),
	    					vec[3].replace(not_numeric, "").parse::<i32>(),
	    					vec[4].replace(not_numeric, "").parse::<i32>(),
	    					vec[5].replace(not_numeric, "").parse::<i32>(),
	    					vec[6].replace(not_numeric, "").parse::<i32>(),
	    					vec[7].replace(not_numeric, "").parse::<i32>() ) {

	    				(Ok(user), Ok(nice), Ok(system), Ok(idle), Ok(iowait), Ok(irq), Ok(softirq)) => 
	    					Ok( CPUtime {user:user, nice:nice, system:system, idle:idle, iowait:iowait, irq:irq, softirq:softirq}),

	    				_ => Err(io::Error::new(io::ErrorKind::InvalidData, "Could not parse fields in /proc/stat"))
	    			}
    			}
    			
    		})
    })
}



/// Returns the CPU usage level as a percentage (f32).  
/// 
/// Specify the interval in seconds over which to perform the analysis. 
pub fn cpu_usage(dur_seconds: u64) -> io::Result<f32> {

	cpu_time().and_then(|time1| {

		thread::sleep(Duration::from_secs(dur_seconds));
		cpu_time().and_then(|time2| {
			let diff: CPUtime = time2 - time1;
			let total: i32 = diff.user + diff.nice + diff.system + diff.idle;
			Ok(100.0 * (1.0 - diff.idle as f32 / total as f32))
		})
	})
}




// Result of a mem_info call.  Values are in kB.
#[derive(Debug)]
pub struct Meminfo {
	pub total: i32,
	pub free: i32,
	pub available: i32,
	pub use_percent: f32
}

// Returns the total, free, and available memory in kB.
pub fn mem_info() -> io::Result<Meminfo> {
    read_file("/proc/meminfo").and_then(|data| {

		match (	data.lines().find(|line| line.starts_with("MemTotal:")), 
				data.lines().find(|line| line.starts_with("MemFree:")), 
				data.lines().find(|line| line.starts_with("MemAvailable:")) ) {

			(Some(total), Some(free), Some(avail)) => {

				match (	total.replace(not_numeric, "").parse::<i32>(), 
						free.replace(not_numeric, "").parse::<i32>(), 
						avail.replace(not_numeric, "").parse::<i32>() ) {

					(Ok(t), Ok(f), Ok(a)) => Ok(Meminfo {total: t, free: f, available: a, use_percent: 100.0*(1.0 - a as f32 / t as f32)}), 
					
					_ => Err(io::Error::new(io::ErrorKind::InvalidData, "Could not parse fields in /proc/meminfo"))
				}
			},

			_ => Err(io::Error::new(io::ErrorKind::InvalidData, "Could not find fields in /proc/meminfo"))
		}
	})
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Diskinfo {
	pub filesystem: String,
	pub total: i32,
	pub used: i32,
	pub available: i32,
	pub use_percent: f32,
	pub mounted_on: String
}


impl fmt::Display for Diskinfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "(fs: {}, total: {}, used: {}, avail: {}, use %: {:.2}, mounted on: {})", 
			self.filesystem, 
			self.total, 
			self.used, 
			self.available, 
			self.use_percent, 
			self.mounted_on )
	}
}






pub fn disk_usage_all() -> io::Result< Vec<Diskinfo> > {


	match Command::new("/bin/df").arg("-k").output() {

		Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Could not execute df command")), 

		Ok(output) => {

			let mut result: Vec<Diskinfo> = Vec::new();
			let stdout = std::str::from_utf8(&output.stdout).unwrap_or("");
			
			for line in stdout.lines().skip(1) {

				let vec: Vec<&str> = line.trim().split_whitespace().collect();

				if vec.len() != 6 {
					
					//sprintln!("Wrong number of fields in df output");
					return Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong number fields in df output"));

				} else {
				
					match (	vec[1].replace(not_numeric, "").parse::<i32>(),
							vec[2].replace(not_numeric, "").parse::<i32>(),
							vec[3].replace(not_numeric, "").parse::<i32>(),
							vec[4].replace(not_numeric, "").parse::<f32>() ) {

						(Ok(blocks), Ok(used), Ok(avail), Ok(use_perc)) => {
							result.push( Diskinfo {
								filesystem: vec[0].to_string(), 
								total: blocks, 
								used: used, 
								available: avail, 
								use_percent: use_perc, 
								mounted_on: vec[5].to_string() } );
						},

						_ => {
							//println!("Failed to parse a line");
							return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse a line"));
						}
					}
				} 
					
			}

			return Ok(result);
		}
	}
}



pub fn disk_usage_by_mount(ptr: &str) -> io::Result<Diskinfo> {
	disk_usage_all()
		.and_then( |disks| disks.iter().find(|&d| d.mounted_on == ptr)
			.ok_or( io::Error::new(io::ErrorKind::InvalidData, "Could not find mount entry") )
			.and_then(|d| Ok(d.clone())))
}		



// Returns the disk info associated with the first instance of the filesystem in the df usage table
pub fn disk_usage_by_filesystem(ptr: &str) -> io::Result<Diskinfo> {
	disk_usage_all()
		.and_then( |disks| disks.iter().find(|&d| d.filesystem == ptr)
			.ok_or( io::Error::new(io::ErrorKind::InvalidData, "Could not find filesystem entry") )
			.and_then(|d| Ok(d.clone())))
}		




pub fn find_mount(disks: &Vec<Diskinfo>, ptr: &str) -> io::Result<Diskinfo> {
	disks.iter().find(|&d| d.mounted_on == ptr)
		.ok_or( io::Error::new(io::ErrorKind::InvalidData, "Could not find mount entry") )
		.and_then(|d| Ok(d.clone()))
}	


// Returns the disk info associated with the first instance of the filesystem in the disk list
pub fn find_filesystem(disks: &Vec<Diskinfo>, ptr: &str) -> io::Result<Diskinfo> {
	disks.iter().find(|&d| d.filesystem == ptr)
		.ok_or( io::Error::new(io::ErrorKind::InvalidData, "Could not find filesystem entry") )
		.and_then(|d| Ok(d.clone()))
}	