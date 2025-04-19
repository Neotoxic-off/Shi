use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Shi")]
pub struct Arguments {
    #[arg(short, long = "ip-count", default_value = "100")]
    pub ip_count: usize,

    #[arg(short, long, default_value = "22")]
    pub port: u64,

    #[arg(short, long, default_value = "5")]
    pub timeout: u64,

    #[arg(short, long, default_value = "results.csv")]
    pub output: String
}
