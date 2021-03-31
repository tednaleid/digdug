use anyhow::Result;
use digdug::args::Args;
use dns_lookup::lookup_addr;

fn main() -> Result<()> {
    let Args { ip_address } = Args::parse()?;
    let host = lookup_addr(&ip_address)?;
    println!("{}", host);
    Ok(())
}
