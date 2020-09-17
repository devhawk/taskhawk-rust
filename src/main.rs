use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Error, ErrorKind};

use std::process::{Command, Stdio};

fn exec_command(cmd: &str, arg: &str) -> io::Result<()> {
    println!("{} {}", cmd, arg);
    let output = Command::new(cmd)
        .arg(arg)
        .stdout(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = std::str::from_utf8(&output.stderr)
            .map_err(|_| Error::new(ErrorKind::Other, "stderr utf8 error"))?;
        Err(Error::new(ErrorKind::Other, err_msg))
    }
}

fn main() -> io::Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or(Error::new(ErrorKind::Other, "no task file path given"))?;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("//") {
            continue;
        }
        let (cmd, arg) = match line.find(' ') {
            Some(pos) => (&line[..pos], &line[pos + 1..]),
            None => (&line[..], ""),
        };

        let _ = exec_command(&cmd, &arg)?;
    }

    Ok(())
}
