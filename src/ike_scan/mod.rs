use std::process::{Command, Output};

use ::config::Config;
use ::CONFIG;
use ::report::scan_report::ScanReport;

#[cfg(target_os = "windows")]
pub const IKE_SCAN_BIN: &'static str = "ike-scan.exe";
#[cfg(not(target_os = "windows"))]
pub const IKE_SCAN_BIN: &'static str = "./ike-scan";

const SUCCESS_MSG: &'static str = "[✓] Reached endpoint";
const FAILURE_MSG: &'static str = "[!] Unable to reach endpoint";

pub fn run(report: &mut ScanReport) {
    if let Ok(output) = exec_ike_scan(&CONFIG) {
        if CONFIG.verbose {
            println!("ike-scan executed. Processing result...");
        }
        match analyze_output(&output) {
            Ok(msg) => {
                println!("{}", msg);
                report.endpoint_reachable()
            },
            Err(msg) => println!("{}", msg)
        }
    } else {
        println!("ike-scan failed");
    }

    if let Ok(output) = exec_ike_scan(&Config::with_nat_t()) {
        if CONFIG.verbose {
            println!("ike-scan executed with NAT-T. Processing result...");
        }
        match analyze_output(&output) {
            Ok(msg) => {
                println!("{} using NAT-T", msg);
                report.endpoint_reachable_using_nat_t()
            },
            Err(msg) => println!("{} using NAT-T", msg)
        }
    } else {
        println!("ike-scan with NAT-T failed");
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
