extern crate chrono;

use std::fs::File;
use std::io::Write;

use self::chrono::{DateTime, Local};
use askama::Template;

use super::scan_host_interface::ScanHostInterface;

#[derive(Default)]
pub struct ScanReport {
    endpoint_reachable: bool,
    endpoint_reachable_using_nat_t: bool,
    scan_host_ifaces: Vec<ScanHostInterface>,
}

#[derive(Template)]
#[template(path = "report.txt.j2")]
struct ScanReportTemplate<'a> {
    now: &'a str,
    endpoint_reachable: bool,
    endpoint_reachable_using_nat_t: bool,
}

impl ScanReport {
    pub fn new() -> ScanReport {
        let result = ScanReport {
            ..Default::default()
        };
        result
    }

    pub fn endpoint_reachable(&mut self) {
        self.endpoint_reachable = true
    }

    pub fn endpoint_reachable_using_nat_t(&mut self) {
        self.endpoint_reachable_using_nat_t = true
    }

    pub fn add_interface(&mut self, iface: ScanHostInterface) {
        self.scan_host_ifaces.push(iface)
    }

    pub fn write_file(self) {
        let mut file = File::create("report.txt").expect("Unable to create file");
        let now: DateTime<Local> = Local::now();
        let template = ScanReportTemplate {
            now: &now.to_rfc2822(),
            endpoint_reachable: self.endpoint_reachable,
            endpoint_reachable_using_nat_t: self.endpoint_reachable_using_nat_t,
        };
        match template.render() {
            Ok(output) => file
                .write_all(&output.into_bytes()[..])
                .expect("Unable to write data"),
            Err(msg) => eprintln!("Failed to write scan report: {}", msg),
        }
    }
}
