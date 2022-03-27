use std::fmt::Display;
use std::str::FromStr;

use clap::{crate_description, crate_name, crate_version, Arg, ArgEnum, Command, PossibleValue};

pub(crate) const SCALE: &str = "scale";

#[derive(ArgEnum, Clone, Copy)]
pub(crate) enum Scale {
    Quarter,
    Half,
    Hour,
}

impl FromStr for Scale {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl Scale {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        Self::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

pub(crate) fn build() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new(SCALE)
                .help("scale of schedule")
                .default_value("hour")
                .takes_value(true)
                .possible_values(Scale::possible_values())
                .required(false),
        )
}
