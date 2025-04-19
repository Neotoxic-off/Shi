use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use log::{info, error};
use rand::Rng;
use env_logger;
use clap::Parser;

mod scan;
mod arguments;


fn write_result(output: &str, results: HashMap<String, String>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output)?;

    for result in results {
        writeln!(file, "{};{}", result.0, result.1)?;
    }

    Ok(())
}


fn get_ip() -> String {
    let mut rng: rand::prelude::ThreadRng = rand::rng();

    let a: i32 = rng.random_range(1..255);
    let b: i32 = rng.random_range(0..255);
    let c: i32 = rng.random_range(0..255);
    let d: i32 = rng.random_range(1..255);

    format!("{}.{}.{}.{}", a, b, c, d)
}

fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
    ).init();
}

fn main() {
    init_logger();

    let args: arguments::Arguments = arguments::Arguments::parse();
    let mut results: HashMap<String, String> = HashMap::new();
    let scan: scan::Scan = scan::Scan::new(args.timeout);

    info!("IP: {}", args.ip_count);
    info!("PORT: {}", args.port);
    info!("TIMEOUT: {}", args.timeout);
    info!("OUTPUT: {}", args.output);

    for _ in 1..args.ip_count {
        let ip: String = get_ip();
        match scan.scan(&ip, args.port) {
            Ok(result) => {
                info!("{:<16} {}", ip, result);
                results.insert(ip, result);
            },
            Err(e) => {
                error!("{:<16} {}", ip, e);
                results.insert(ip, e.to_string());
            }
        }
    }

    let _ = write_result(&args.output, results);
}
