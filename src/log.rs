use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, TcpStream};
#[cfg(all(target_family = "unix"))]
#[cfg(not(target_os = "linux"))]
use std::process::Command;
#[cfg(all(target_family = "unix"))]
#[cfg(not(target_os = "linux"))]
use std::string::FromUtf8Error;
use std::time::SystemTime;
use std::{fmt, io};

use chrono::Utc;
use opentelemetry_semantic_conventions::resource::*;
use serde::Serialize;
use serde_json::map::Map;

#[derive(Clone)]
pub struct Logger {
    attributes: LogAttributes,
}

#[derive(Copy, Clone)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    pub fn severity_number(level: LogLevel) -> u8 {
        match level {
            LogLevel::Trace => 1,
            LogLevel::Debug => 5,
            LogLevel::Info => 9,
            LogLevel::Warn => 13,
            LogLevel::Error => 17,
            LogLevel::Fatal => 24,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Message {
    attributes: Map<String, serde_json::Value>,
    body: String,
    timestamp: u64,
    severity_text: String,
    severity_number: u8,
    resource: Map<String, serde_json::Value>,
}

impl Logger {
    pub fn new(attributes: LogAttributes) -> Self {
        Logger { attributes }
    }

    pub fn trace(&self, body: &str) {
        self.write(body.to_string(), LogLevel::Trace)
    }

    pub fn debug(&self, body: &str) {
        self.write(body.to_string(), LogLevel::Debug)
    }

    pub fn info(&self, body: &str) {
        self.write(body.to_string(), LogLevel::Info)
    }

    pub fn warn(&self, body: &str) {
        self.write(body.to_string(), LogLevel::Warn)
    }

    pub fn error(&self, body: &str) {
        self.write(body.to_string(), LogLevel::Error)
    }

    pub fn fatal(&self, body: &str) {
        self.write(body.to_string(), LogLevel::Fatal)
    }

    fn write(&self, body: String, level: LogLevel) {
        let now = SystemTime::now();

        let epoch = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(now) => now.as_nanos() as u64,
            Err(e) => panic!("{}", e.to_string()),
        };

        let mut attributes = Map::new();

        attributes.insert(
            "human_timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339()),
        );
        attributes.insert(
            "ipaddress".to_string(),
            serde_json::Value::String(self.attributes.ip_address.to_string()),
        );

        let mut resource = Map::new();

        resource.insert(
            HOST_NAME.to_string(),
            serde_json::Value::String(self.attributes.hostname.clone()),
        );

        let severity_number = LogLevel::severity_number(level);

        let m = Message {
            attributes,
            body,
            timestamp: epoch,
            severity_text: level.to_string(),
            severity_number,
            resource,
        };

        let serialized_message = serde_json::to_string(&m).unwrap();

        if severity_number <= 9 {
            println!("{}", serialized_message)
        } else {
            eprintln!("{}", serialized_message)
        }
    }
}

#[derive(Serialize, Clone)]
pub struct LogAttributes {
    hostname: String,
    ip_address: IpAddr,
}

impl LogAttributes {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let hostname = get_hostname()?;

        let ip_address = get_ipv4_address()?;

        Ok(LogAttributes {
            hostname,
            ip_address,
        })
    }
}

#[cfg(target_os = "linux")]
fn get_hostname() -> io::Result<String> {
    let mut file = File::open("/proc/sys/kernel/hostname")?;
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s.trim().to_string()),
        Err(e) => Err(e),
    }
}
#[cfg(all(target_family = "unix"))]
#[cfg(not(target_os = "linux"))]
fn get_hostname() -> Result<String, FromUtf8Error> {
    let mut input = Command::new("hostname");
    let output = input
        .output()
        .expect("could not get output from running `hostname`");
    Ok(String::from_utf8(output.stdout)
        .expect("invalid characters from stdout running `hostname`")
        .trim()
        .to_string())
}

fn get_ipv4_address() -> io::Result<IpAddr> {
    let stream = TcpStream::connect("8.8.8.8:443")?;
    let local = stream.local_addr()?;
    Ok(local.ip())
}
