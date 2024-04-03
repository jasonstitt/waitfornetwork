use clap::Parser;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "8.8.8.8")]
    host: String,

    #[clap(long, default_value = "53")]
    port: u16,

    #[clap(long, default_value = "60")]
    timeout: u64,

    #[clap(long, default_value = "5")]
    interval: u64,
}

fn main() {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let timeout = Duration::from_secs(args.timeout);
    let interval = Duration::from_secs(args.interval);

    let start = Instant::now();
    loop {
        if TcpStream::connect(addr.clone()).is_ok() {
            return;
        }

        if start.elapsed() >= timeout {
            println!(
                "network unavalable: failed {}:{} after {} seconds",
                args.host, args.port, args.timeout
            );
            return;
        }

        sleep(interval);
    }
}
