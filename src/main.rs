use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use std::env;
use std::net::ToSocketAddrs;

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <IP>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let mut futures = Vec::new();

    for port in 1..=65535 {
        
        let ip = ip.clone();
        let future = tokio::spawn(async move {
            let addr = format!("{}:{}", ip, port);
            if let Ok(_) = time::timeout(Duration::from_millis(100), TcpStream::connect(&addr)).await {
                println!("Port {} is open", port);
            }
        });
        futures.push(future);
    }

    futures::future::join_all(futures).await;
}
