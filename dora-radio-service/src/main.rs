// Return type for this service.
type ServiceResult<T> = Result<T, Error>;

use comms_service::*;
use failure::*;
use kubos_service::Logger;
use log::*;
use serial;
use serial::prelude::*;
use std::cell::RefCell;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const BUS: &str = "/dev/ttyS2";
const TIMEOUT: Duration = Duration::from_millis(100);
// Maximum number of bytes to attempt to read at one time
const MAX_READ: usize = 48;


// Initialize the serial bus connection for reading and writing from/to the "radio"
pub fn serial_init() -> ServiceResult<Arc<Mutex<RefCell<serial::SystemPort>>>> {

    // Define our serial settings
    let settings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };

    // Open a connection to the serial port
    let mut port = serial::open(BUS)?;

    // Save our settings
    port.configure(&settings)?;
    port.set_timeout(TIMEOUT)?;

    // Wrap the port in a mutex so that multiple threads can access it
    let conn = Arc::new(Mutex::new(RefCell::new(port)));

    Ok(conn)
}



// The write function that the comms service will use to write messages to the "radio"
//
// This function may be called from either a message handler thread or from a downlink endpoint
pub fn write(conn: &Arc<Mutex<RefCell<serial::SystemPort>>>, msg: &[u8]) -> ServiceResult<()> {
    let conn = match conn.lock() {
        Ok(val) => val,
        Err(e) => bail!("Failed to take mutex: {:?}", e),
    };
    let mut conn = conn.try_borrow_mut()?;

    conn.write(msg).and_then(|num| {
        debug!("Wrote {} bytes to radio", num);
        Ok(())
    })?;

    Ok(())
}



// The read function that the comms service read thread will call to wait for messages from the
// "radio"
//
// Returns once a message has been received
pub fn read(conn: &Arc<Mutex<RefCell<serial::SystemPort>>>) -> ServiceResult<Vec<u8>> {
    loop {
        // Note: These brackets force the program to release the serial port's mutex so that any
        // threads waiting on it in order to perform a write may do so
        {
            // Take ownership of the serial port
            let conn = match conn.lock() {
                Ok(val) => val,
                Err(e) => {
                    error!("Failed to take mutex: {:?}", e);
                    panic!();
                }
            };
            let mut conn = conn.try_borrow_mut()?;

            // Loop until either a full message has been received or a non-timeout error has occured
            let mut packet = vec![];
            loop {
                let mut buffer: Vec<u8> = vec![0; MAX_READ];
                match conn.read(buffer.as_mut_slice()) {
                    Ok(num) => {
                        buffer.resize(num, 0);
                        packet.append(&mut buffer);

                        debug!("Read {} bytes from radio", packet.len());

                        if num < MAX_READ {
                            return Ok(packet);
                        }
                    }
                    Err(ref err) => match err.kind() {
                        ::std::io::ErrorKind::TimedOut => {
                            if packet.len() > 0 {
                                return Ok(packet);
                            } else {
                                break;
                            }
                        }
                        other => bail!("Radio read failed: {:?}", other),
                    },
                };
            }
        }

        // Sleep for a moment so that other threads have the chance to grab the serial port mutex
        thread::sleep(Duration::from_millis(10));
    }
}





fn main() -> ServiceResult<()> {

    // Initialize logging for the service
    Logger::init("dora-radio-service").unwrap();

    // Get the main service configuration from the system's config.toml file
    let service_config = kubos_system::Config::new("dora-radio-service")?;

    // Pull out our communication settings
    let config = CommsConfig::new(service_config)?;

    // Initialize the serial port
    let conn = serial_init()?;

    // In this instance, reading and writing are done over the same connection,
    // so we'll just clone the UART port connection
    let read_conn = conn.clone();
    let write_conn = conn;

    // Tie everything together in our final control block
    let control = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        read_conn,
        write_conn,
        config,
    )?;

    // Set up our communications telemetry structure
    let telemetry = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start the comms service thread
    CommsService::start::<Arc<Mutex<RefCell<serial::SystemPort>>>, SpacePacket>(control, &telemetry)?;

    // TODO: Start the GraphQL service
    loop {}


}