//! `die [--check a,b] [--handover socket] [--restart-after-frame] [--restart-loop] [--frontmost]
//! [--background] [--wait] [--] [file...]`: the owner's DIE, with the files named. It's die-dev's
//! `main` with this config's extensions, keys, faces and greeting on top
//! ([`die_config::extra`]): in DIE.app the one instance, or from a terminal the client that sends
//! files to it.

#[cfg(test)]
use die_test_cap as _;

use std::process::ExitCode;

fn main() -> ExitCode {
    die_dev::main_with(die_config::extra)
}
