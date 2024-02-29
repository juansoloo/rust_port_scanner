use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use std::env;

pub const MOST_COMMON_PORTS_1002: &[u16] = &[
    5601, 9300, 80, // truncated for brevity
    // Include all other ports here...
    3307,
];

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <IP>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let mut futures = Vec::new();

    for port in MOST_COMMON_PORTS_1002.iter() {
        let ip = ip.clone();
        let future = tokio::spawn(async move {
            let addr = format!("{}:{}", ip, port);
            if let Ok(_) = time::timeout(Duration::from_millis(2000), TcpStream::connect(&addr)).await {
                println!("Port {} is open", port);
            }
        });
        futures.push(future);
    }


        futures::future::join_all(futures).await;
    }
