use std::{
    env::args_os,
    process::{exit, Command},
};

fn main() {
    #[cfg(windows)]
    nu_ansi_term::enable_ansi_support().ok();

    let mut args = args_os().skip(1);
    exit(match args.next() {
        None => -1,
        Some(program) => match Command::new(program).args(args).spawn() {
            Err(err) => {
                eprintln!("{}", err);
                -2
            }
            Ok(mut child) => match child.wait() {
                Err(err) => {
                    eprintln!("{}", err);
                    -3
                }
                Ok(code) => match code.code() {
                    None => -4,
                    Some(code) => code,
                },
            },
        },
    });
}
