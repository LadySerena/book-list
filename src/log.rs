use std::fs::File;
use std::io;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, TcpStream};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub struct Logger {
    attributes: LogAttributes,
}
// TODO add level support
// TODO implement actual writing

#[derive(Serialize)]
struct Message {
    attributes: LogAttributes,
    body: String,
    timestamp: String,
}

impl Logger {
    pub fn new(attributes: LogAttributes) -> Self {
        Logger {
            attributes: attributes.clone(),
        }
    }

    pub fn write(self, body: &str) {
        let m = Message {
            attributes: self.attributes.clone(),
            body,
            timestamp: Utc::now().to_rfc3339(),
        };
        println!("{}", serde_json::to_string(&m).unwrap())
    }
}

#[derive(Serialize, Clone)]
pub struct LogAttributes {
    hostname: String,
    ip_address: IpAddr,
}

impl LogAttributes {
    pub(crate) fn new() -> Result<Self, io::Error> {
        let hostname = if cfg!(target_os = "linux") {
            get_hostname_linux()?
        } else {
            "unknown".to_string()
        };

        let ip_address = get_ipv4_address()?;

        Ok(LogAttributes {
            hostname,
            ip_address,
        })
    }
}

#[cfg(target_os = "linux")]
fn get_hostname_linux() -> io::Result<String> {
    let mut file = File::open("/proc/sys/kernel/hostname")?;
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s.trim().to_string()),
        Err(e) => Err(e),
    }
}


fn get_ipv4_address() -> io::Result<IpAddr> {
    let stream = TcpStream::connect("8.8.8.8:443")?;
    let local = stream.local_addr()?;
    Ok(local.ip())
}