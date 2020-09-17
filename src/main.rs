use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use std::process::{Command, Stdio};
use std::path::Path;

fn exec_command(cmd: &str, arg: &str) -> io::Result<bool> {

    println!("{} {}", cmd, arg);
    let status = Command::new(cmd)
        .arg(arg)
        .stdout(Stdio::inherit())
        .status()?;

    Ok(status.success())
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("no task file path given");
    if !Path::new(&path).exists()
    {
        panic!("{} task file does not exist", path)
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (a,b) = match line.find(' ') {
            Some(pos) => (&line[..pos], &line[pos+1..]),
            None => (&line[..], ""),
        };

        let exec_result = exec_command(&a, &b)?;
        if !exec_result { break; }
    }
    // let path = std::fs::canonicalize(&path);

    Ok(())
}
