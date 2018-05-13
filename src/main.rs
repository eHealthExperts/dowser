#[macro_use]
extern crate askama;
#[macro_use]
extern crate lazy_static;

use std::path::Path;

pub mod config;
pub mod env_scan;
pub mod ike_scan;
pub mod opts;
pub mod report;
use ::config::Config;
use ::report::scan_report::ScanReport;


lazy_static! {
    pub static ref CONFIG: Config = ::opts::get_opts();
}

fn verify_env_reqs() {
    if !Path::new(ike_scan::IKE_SCAN_BIN).exists() {
        panic!("Unable to find binary dependency");
    }

    if CONFIG.verbose {
        println!("Environment ready");
    }
}



fn main() {
    if CONFIG.verbose {
        println!("IP of VPN endpoint: {}", CONFIG.vpn_endpoint_ip);
        println!("Using source port: {}", CONFIG.source_port);
    }

    verify_env_reqs();

    let mut report = ScanReport::new();
    ike_scan::run(&mut report);
    env_scan::run(&mut report);
    report.write_file();
}
