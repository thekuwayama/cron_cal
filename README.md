# cron_cal

[![CI](https://github.com/thekuwayama/cron_cal/workflows/CI/badge.svg)](https://github.com/thekuwayama/cron_cal/actions?workflow=CI)
[![license](https://img.shields.io/badge/license-MIT/Apache--2.0-blue?style=flat)](https://raw.githubusercontent.com/thekuwayama/cron_cal/main/LICENSE-APACHE)

`cron_cal` is CLI to calculate cron schedule.


## Install

You can install `cron_cal` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/cron_cal.git --branch main
```


## Usage

```sh-session
$ cron_cal --help
cron_cal 0.1.0
cron calender

USAGE:
    cron_cal [OPTIONS]

OPTIONS:
    -d, --date <date>    start date
        --days <days>    target days [default: 1]
    -h, --help           Print help information
    -V, --version        Print version information
```

```sh-session
$ cat << EOS | cron_cal
> "30 9 * * * *", 5
> "30 12 * * * *", 5
> "30 15 * * * *", 5
> EOS
2022-04-07 09:30:00 UTC ~ 2022-04-07 09:35:00 UTC
2022-04-07 12:30:00 UTC ~ 2022-04-07 12:35:00 UTC
2022-04-07 15:30:00 UTC ~ 2022-04-07 15:35:00 UTC
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/thekuwayama/cron_cal/blob/main/LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/thekuwayama/cron_cal/blob/main/LICENSE-MIT) or http://opensource.org/licenses/MIT)
