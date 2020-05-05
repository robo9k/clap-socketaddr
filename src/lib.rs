use clap::Clap;

/// Arguments to create one [`SocketAddr`](std::net::SocketAddr)
#[derive(Clap, Debug)]
pub struct SocketAddrArgs {
    /// IP address
    #[clap(short, long, env, default_value = "127.0.0.1")]
    address: std::net::IpAddr,
    /// IP port number
    #[clap(short, long, env, default_value = "0")]
    port: u16,
}

impl SocketAddrArgs {
    /// Returns the parsed/default IP address argument
    #[must_use]
    pub const fn address(&self) -> &std::net::IpAddr {
        &self.address
    }

    /// Returns the parsed/default IP port number
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }
}

impl From<SocketAddrArgs> for std::net::SocketAddr {
    fn from(args: SocketAddrArgs) -> Self {
        (args.address, args.port).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let args = SocketAddrArgs::parse_from(&["dummy"]);

        assert!(args.address().is_loopback());
        assert_eq!(args.port(), 0);
    }

    #[test]
    fn test_address_value() {
        use std::net::{IpAddr, Ipv4Addr};
        let args = SocketAddrArgs::parse_from(&["dummy", "--address", "192.0.2.42"]);

        assert_eq!(args.address(), &IpAddr::V4(Ipv4Addr::new(192, 0, 2, 42)));
    }

    #[test]
    fn test_port_value() {
        let args = SocketAddrArgs::parse_from(&["dummy", "--port", "666"]);

        assert_eq!(args.port(), 666);
    }

    #[test]
    fn test_socketaddr() {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        let args = SocketAddrArgs::parse_from(&["dummy", "--port", "42"]);
        let socket_addr: SocketAddr = args.into();

        assert_eq!(socket_addr.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(socket_addr.port(), 42);
    }
}
