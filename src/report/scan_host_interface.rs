use std::net::IpAddr;

#[derive(Default)]
pub struct ScanHostInterface {
    gateway: Option<IpAddr>,
    netmask: Option<IpAddr>,
    ip: Option<IpAddr>,
    dns1: Option<IpAddr>,
    dns2: Option<IpAddr>,
    domain: Option<String>,
}

pub struct ScanHostInterfaceBuilder {
    iface: ScanHostInterface,
}

impl ScanHostInterface {
    pub fn builder() -> ScanHostInterfaceBuilder {
        ScanHostInterfaceBuilder::new()
    }
}

impl ScanHostInterfaceBuilder {
    fn new() -> ScanHostInterfaceBuilder {
        let iface = ScanHostInterface {
            ..Default::default()
        };
        ScanHostInterfaceBuilder { iface }
    }

    pub fn gateway(&mut self, ip: IpAddr) -> &mut ScanHostInterfaceBuilder {
        self.iface.gateway = Some(ip);
        self
    }

    pub fn netmask(&mut self, ip: IpAddr) -> &mut ScanHostInterfaceBuilder {
        self.iface.netmask = Some(ip);
        self
    }

    pub fn ip(&mut self, ip: IpAddr) -> &mut ScanHostInterfaceBuilder {
        self.iface.ip = Some(ip);
        self
    }

    pub fn dns1(&mut self, ip: IpAddr) -> &mut ScanHostInterfaceBuilder {
        self.iface.dns1 = Some(ip);
        self
    }

    pub fn dns2(&mut self, ip: IpAddr) -> &mut ScanHostInterfaceBuilder {
        self.iface.dns2 = Some(ip);
        self
    }

    pub fn domain(&mut self, domain: String) -> &mut ScanHostInterfaceBuilder {
        self.iface.domain = Some(domain);
        self
    }

    pub fn build(self) -> ScanHostInterface {
        self.iface
    }
}
