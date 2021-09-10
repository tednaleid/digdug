use anyhow::Result;
use clap::{App, Arg};
use std::env;
use std::ffi::OsString;
use std::net::IpAddr;

pub struct Args {
    pub ip_address: IpAddr,
}

impl Args {
    pub fn parse() -> Result<Self> {
        Args::parse_from(&mut env::args_os())
    }

    pub fn parse_from<I, T>(itr: I) -> Result<Self>
        where
            I: IntoIterator<Item=T>,
            T: Into<OsString> + Clone,
    {
        let matches = App::new("digdug")
            .arg(
                Arg::new("address")
                    .about("The IP address to look up")
                    .required(true)
                    .index(1),
            )
            .get_matches_from(itr);

        let ip_address = matches.value_of_t("address").unwrap_or_else(|e| e.exit());

        Ok(Self { ip_address })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::net::Ipv4Addr;

    #[test]
    fn parse_method_success() -> Result<()> {
        assert_eq!(
            Args::parse_from(vec!["digdug", "127.0.0.1"])?.ip_address,
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        );
        Ok(())
    }
}
