#[macro_use]
extern crate lazy_static;

use std::path::Path;
use std::process::{Command, Output};

pub mod config;
pub mod opts;
use ::config::Config;

#[cfg(target_os = "windows")]
const IKE_SCAN_BIN: &'static str = "ike-scan.exe";
#[cfg(not(target_os = "windows"))]
const IKE_SCAN_BIN: &'static str = "./ike-scan";

const SUCCESS_MSG: &'static str = "[✓] Reached endpoint";
const FAILURE_MSG: &'static str = "[!] Unable to reach endpoint";

lazy_static! {
    pub static ref CONFIG: Config = ::opts::get_opts();
}

fn verify_env_reqs() {
    if !Path::new(IKE_SCAN_BIN).exists() {
        panic!("Unable to find binary dependency");
    }

    if CONFIG.verbose {
        println!("Environment ready");
    }
}

fn exec_ike_scan(conf: &Config) -> Result<Output, String> {
    let mut cmd = Command::new(IKE_SCAN_BIN);
    if conf.use_nat_t {
        cmd.arg("--nat-t");
    } else {
        cmd.arg("--sport").arg(&conf.source_port.to_string());
    }
    cmd.arg(&conf.vpn_endpoint_ip);
    let output = cmd
                .output()
                .expect(&format!("Failed to connect using ike-scan with parameters --sport {} to endpoint {}",
                        &conf.source_port.to_string(), &conf.vpn_endpoint_ip));

    Ok(output)
}

fn analyze_output(output: &Output) -> Result<String, String> {
    if !output.status.success() {
        println!("ike-scan was a failure - exit code {:?}", output.status.code().expect("Cannot get exit code of ike-scan execution (process must have been killed by signal)"));
        println!("ike-scan stderr output: {:?}", String::from_utf8_lossy(&output.stderr));
        panic!("Aborting analysis due to failed ike-scan run");
    }

    let string_to_analyze = String::from_utf8_lossy(&output.stdout);

    let ret;
    if string_to_analyze.contains("1 returned notify") {
        ret = Ok(SUCCESS_MSG.to_owned());
    } else {
        ret = Err(FAILURE_MSG.to_owned())
    }
    if CONFIG.verbose {
        println!("Response was:\n{:?}", &string_to_analyze);
    }
    ret
}

fn main() {
    if CONFIG.verbose {
        println!("IP of VPN endpoint: {}", CONFIG.vpn_endpoint_ip);
        println!("Using source port: {}", CONFIG.source_port);
    }
    verify_env_reqs();
    if let Ok(output) = exec_ike_scan(&CONFIG) {
        if CONFIG.verbose {
            println!("ike-scan executed. Processing result...");
        }
        match analyze_output(&output) {
            Ok(msg) => println!("{}", msg),
            Err(msg) => println!("{}", msg)
        }
    } else {
        println!("ike-scan failed");
    };

    if let Ok(output) = exec_ike_scan(&Config::with_nat_t()) {
        if CONFIG.verbose {
            println!("ike-scan executed with NAT_T. Processing result...");
        }
        match analyze_output(&output) {
            Ok(msg) => println!("{} using NAT-T", msg),
            Err(msg) => println!("{} using NAT-T", msg)
        }
    } else {
        println!("ike-scan with NAT-T failed");
    }
}
