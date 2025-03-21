use async_arp::{Client, ClientConfigBuilder, ClientSpinner, ProbeStatus, Result};
use clap::Parser;
use std::io::Write;
use std::time::{Duration, Instant};

mod common;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = common::Args::parse();
    let interface = common::interface_from(&args.iface);
    let net = common::net_from(&interface).unwrap();

    let client = Client::new(
        ClientConfigBuilder::new(&args.iface)
            .with_response_timeout(Duration::from_millis(500))
            .build(),
    )?;
    let spinner = ClientSpinner::new(client).with_retries(9);

    let start = Instant::now();
    let outcomes = spinner
        .probe_batch(&common::generate_probe_inputs(net, interface))
        .await;

    let occupied = outcomes?
        .into_iter()
        .filter(|outcome| outcome.status == ProbeStatus::Occupied);
    let scan_duration = start.elapsed();

    {
        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "Found hosts:").unwrap();
        for outcome in occupied {
            writeln!(stdout, "{:?}", outcome).unwrap();
        }
        writeln!(stdout, "Scan took {:?}", scan_duration).unwrap();
    }

    Ok(())
}
