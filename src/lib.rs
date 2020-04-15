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
    pub fn address(&self) -> &std::net::IpAddr {
        &self.address
    }

    /// Returns the parsed/default IP port number
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl From<SocketAddrArgs> for std::net::SocketAddr {
    fn from(args: SocketAddrArgs) -> Self {
        (args.address, args.port).into()
    }
}
