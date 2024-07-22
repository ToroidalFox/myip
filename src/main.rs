use clap::{Parser, Subcommand};
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Ip>,

    #[arg(short, long, default_value_t = 1)]
    timeout: u64,
}

#[derive(Subcommand)]
enum Ip {
    /// Prints public ipv4 address.
    V4,
    /// Prints public ipv6 address.
    V6,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut cli = Cli::parse();
    cli.timeout = cli.timeout.max(1);

    match cli.command {
        None => {
            let client = reqwest::Client::new();
            let ip = fetch_public_ip(&client, Duration::from_secs(cli.timeout))
                .await
                .unwrap();
            println!("{}", ip);
        }
        Some(Ip::V4) => {
            let client = reqwest::Client::builder()
                .local_address(Some(std::net::Ipv4Addr::new(0, 0, 0, 0).into()))
                .build()
                .unwrap();
            let ip = fetch_public_ip(&client, Duration::from_secs(cli.timeout))
                .await
                .unwrap();
            println!("{}", ip);
        }
        Some(Ip::V6) => {
            let client = reqwest::Client::builder()
                .local_address(Some(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into()))
                .build()
                .unwrap();
            let ip = fetch_public_ip(&client, Duration::from_secs(cli.timeout))
                .await
                .unwrap();
            println!("{}", ip);
        }
    }
}

const CLOUDFLARE_TRACE: &str = "https://cloudflare.com/cdn-cgi/trace";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("web request failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("parsing failed")]
    Parse(#[from] ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("no line starts with ip")]
    NoLineStartsWithIp,
    #[error("format changed")]
    IpLineFormatChanged,
    #[error("failed to parse ip address")]
    AddrParse(#[from] std::net::AddrParseError),
}

/// Fetch public IP from Cloudflare CDN trace.
pub async fn fetch_public_ip(
    client: &reqwest::Client,
    timeout: Duration,
) -> Result<std::net::IpAddr, crate::Error> {
    let trace = client
        .get(CLOUDFLARE_TRACE)
        .timeout(timeout)
        .send()
        .await?
        .text()
        .await?;

    trace
        // get line that starts with ip
        .lines()
        .filter(|line| line.starts_with("ip"))
        .next()
        .ok_or(ParseError::NoLineStartsWithIp)?
        // split line with `=` and get 2nd string
        .split("=")
        .nth(1)
        .ok_or(ParseError::IpLineFormatChanged)?
        // parse as ip address
        .parse::<std::net::IpAddr>()
        .map_err(|e| Into::<ParseError>::into(e).into())
}
