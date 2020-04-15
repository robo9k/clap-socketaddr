use clap::Clap;

/// Example for socket address in command line arguments
#[derive(Clap, Debug)]
struct Cli {
    #[clap(flatten)]
    address: clap_socketaddr::SocketAddrArgs,
}

fn main() {
    let args = Cli::parse();
    println!("args {:?}", args);

    let tcp_listener = std::net::TcpListener::bind::<std::net::SocketAddr>(args.address.into());
    println!("tcp listener {:?}", tcp_listener);
}
