use smol_str::SmolStr;
use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpAddrValue(pub IpAddr);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainValue(pub String);

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Eq)]
pub struct IpNetValue {
    addr: IpAddr,
    prefix_len: u8,
}

impl IpNetValue {
    pub fn new(addr: IpAddr, prefix_len: u8) -> Option<Self> {
        // Validate prefix length based on address family (v4: <=32, v6: <=128)
        let max_prefix = match addr {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        } as u8;
        if prefix_len > max_prefix {
            return None;
        }
        Some(Self { addr, prefix_len })
    }
}
impl Display for IpNetValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix_len)
    }
}
// Comparison impl moved to orion_exp adapters.

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DomainT(pub SmolStr);
impl Display for DomainT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct UrlValue(pub SmolStr);
impl Display for UrlValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct EmailT(pub SmolStr);
impl Display for EmailT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    // ========== IpNetValue tests ==========

    #[test]
    fn test_ip_net_value_new_v4_valid() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0));
        let net = IpNetValue::new(ip, 24);
        assert!(net.is_some());
        let net = net.unwrap();
        assert_eq!(format!("{}", net), "192.168.1.0/24");
    }

    #[test]
    fn test_ip_net_value_new_v4_max_prefix() {
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        // /32 is valid for IPv4
        let net = IpNetValue::new(ip, 32);
        assert!(net.is_some());
        assert_eq!(format!("{}", net.unwrap()), "10.0.0.1/32");
    }

    #[test]
    fn test_ip_net_value_new_v4_invalid_prefix() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0));
        // /33 is invalid for IPv4
        let net = IpNetValue::new(ip, 33);
        assert!(net.is_none());
    }

    #[test]
    fn test_ip_net_value_new_v6_valid() {
        let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0));
        let net = IpNetValue::new(ip, 64);
        assert!(net.is_some());
        assert_eq!(format!("{}", net.unwrap()), "2001:db8::/64");
    }

    #[test]
    fn test_ip_net_value_new_v6_max_prefix() {
        let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        // /128 is valid for IPv6
        let net = IpNetValue::new(ip, 128);
        assert!(net.is_some());
    }

    #[test]
    fn test_ip_net_value_new_v6_invalid_prefix() {
        let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0));
        // /129 is invalid for IPv6
        let net = IpNetValue::new(ip, 129);
        assert!(net.is_none());
    }

    #[test]
    fn test_ip_net_value_display() {
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0));
        let net = IpNetValue::new(ip, 8).unwrap();
        assert_eq!(format!("{}", net), "10.0.0.0/8");
    }

    #[test]
    fn test_ip_net_value_clone_eq() {
        let ip = IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0));
        let net1 = IpNetValue::new(ip, 12).unwrap();
        let net2 = net1.clone();
        assert_eq!(net1, net2);
    }

    // ========== DomainT tests ==========

    #[test]
    fn test_domain_t_new() {
        let domain = DomainT("example.com".into());
        assert_eq!(domain.0.as_str(), "example.com");
    }

    #[test]
    fn test_domain_t_display() {
        let domain = DomainT("test.example.org".into());
        assert_eq!(format!("{}", domain), "test.example.org");
    }

    #[test]
    fn test_domain_t_clone_eq() {
        let d1 = DomainT("domain.com".into());
        let d2 = d1.clone();
        assert_eq!(d1, d2);
    }

    // ========== UrlValue tests ==========

    #[test]
    fn test_url_value_new() {
        let url = UrlValue("https://example.com/path".into());
        assert_eq!(url.0.as_str(), "https://example.com/path");
    }

    #[test]
    fn test_url_value_display() {
        let url = UrlValue("http://localhost:8080/api".into());
        assert_eq!(format!("{}", url), "http://localhost:8080/api");
    }

    #[test]
    fn test_url_value_clone_eq() {
        let u1 = UrlValue("https://test.com".into());
        let u2 = u1.clone();
        assert_eq!(u1, u2);
    }

    // ========== EmailT tests ==========

    #[test]
    fn test_email_t_new() {
        let email = EmailT("user@example.com".into());
        assert_eq!(email.0.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_t_display() {
        let email = EmailT("admin@test.org".into());
        assert_eq!(format!("{}", email), "admin@test.org");
    }

    #[test]
    fn test_email_t_clone_eq() {
        let e1 = EmailT("a@b.com".into());
        let e2 = e1.clone();
        assert_eq!(e1, e2);
    }

    // ========== Serde tests ==========

    #[test]
    fn test_ip_net_value_serde() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0));
        let net = IpNetValue::new(ip, 16).unwrap();
        let json = serde_json::to_string(&net).unwrap();
        let parsed: IpNetValue = serde_json::from_str(&json).unwrap();
        assert_eq!(net, parsed);
    }

    #[test]
    fn test_domain_t_serde() {
        let domain = DomainT("serde.test.com".into());
        let json = serde_json::to_string(&domain).unwrap();
        let parsed: DomainT = serde_json::from_str(&json).unwrap();
        assert_eq!(domain, parsed);
    }

    #[test]
    fn test_url_value_serde() {
        let url = UrlValue("https://serde.example.com".into());
        let json = serde_json::to_string(&url).unwrap();
        let parsed: UrlValue = serde_json::from_str(&json).unwrap();
        assert_eq!(url, parsed);
    }

    #[test]
    fn test_email_t_serde() {
        let email = EmailT("serde@test.com".into());
        let json = serde_json::to_string(&email).unwrap();
        let parsed: EmailT = serde_json::from_str(&json).unwrap();
        assert_eq!(email, parsed);
    }

    // ========== IpAddrValue tests ==========

    #[test]
    fn test_ip_addr_value() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let val = IpAddrValue(ip);
        assert_eq!(val.0, ip);

        let cloned = val.clone();
        assert_eq!(val, cloned);
    }

    #[test]
    fn test_ip_addr_value_serde() {
        let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let val = IpAddrValue(ip);
        let json = serde_json::to_string(&val).unwrap();
        let parsed: IpAddrValue = serde_json::from_str(&json).unwrap();
        assert_eq!(val, parsed);
    }
}
