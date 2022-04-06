use clap::{crate_description, crate_name, crate_version, Arg, Command};

pub(crate) const DATE: &str = "date";
pub(crate) const DAYS: &str = "days";

pub(crate) fn build() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new(DATE)
                .long(DATE)
                .short('d')
                .help("start date")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new(DAYS)
                .long(DAYS)
                .help("target days")
                .default_value("1")
                .takes_value(true)
                .required(false),
        )
}
