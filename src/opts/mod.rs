extern crate clap;
use self::clap::{Arg, App};
use ::config::Config;

pub fn get_opts() -> Config {
    let matches = App::new("dowser - ike-scan convenience facade")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("eHealth Experts GmbH https://www.ehealthexperts.de")
                          .about("Convenience wrapper tool that utilizes ike-scan and interprets the output to analyze VPN connectivity from a certain environment.")
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .help("Activate verbose log output")
                               )
                          .arg(Arg::with_name("source_port")
                               .short("s")
                               .long("sport")
                               .value_name("ike-scan source port")
                               .default_value("4242")
                               .help("Local source port to establish connection from (only non-NAT-T case)")
                               .takes_value(true)
                               )
                          .arg(Arg::with_name("endpoint")
                               .value_name("VPN endpoint IP")
                               .default_value("146.185.113.4")
                               .help("IP of the VPN endpoint to attempt to connect to")
                               .takes_value(true)
                               )
                          .get_matches();
    let mut cfg_bldr = Config::builder();
    if let Some(endpoint) = matches.value_of("endpoint") {
        cfg_bldr.vpn_endpoint_ip(endpoint);
    }
    if let Some(port_arg) = matches.value_of("source_port") {
        cfg_bldr.source_port(port_arg);
    }
    if matches.is_present("verbose") {
        cfg_bldr.verbose();
    }
    cfg_bldr.build()
}
