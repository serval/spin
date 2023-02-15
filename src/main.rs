use std::io::Write;
use std::process::{exit, Command, Stdio};

use nonblock::NonBlockingReader;

fn main() {
    let mut args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.is_empty() {
        println!("usage: spin [cmd] [args...]");
        exit(1);
    }
    let cmd = args.remove(0);

    let mut cmd = Command::new(cmd);
    cmd.args(&args);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    let mut handle = match cmd.spawn() {
        Ok(handle) => handle,
        Err(err) => panic!("Failed to spawn: {err:?}"),
    };
    let Some(stdout) = handle.stdout.take() else {
        panic!("Failed to get stdout from child");
    };
    let Ok(mut nonblock_stdout) = NonBlockingReader::from_fd(stdout) else {
        panic!("Failed to make non-blocking reader for stdout");
    };
    let line_prefix = format!("\n[{}] ", handle.id());
    let mut output_prefix = line_prefix.to_owned();
    while !nonblock_stdout.is_eof() {
        let mut buf = vec![];
        let bytes_read = nonblock_stdout
            .read_available(&mut buf)
            .expect("Failed to read from child process");
        if bytes_read == 0 {
            continue;
        }

        match String::from_utf8(buf) {
            Ok(str) => {
                // prefix every line of output with the child's process number
                print!("{}{}", output_prefix, str.replace("\n", &line_prefix));
                if !output_prefix.is_empty() {
                    output_prefix = String::from("");
                }
            }
            Err(err) => {
                // child isn't writing UTF-8 data, apparently; just write it out as-is
                // (it's possible that we just read in the middle of a multi-byte sequence, but
                // there's no way to know)
                let buf = err.into_bytes();
                std::io::stdout()
                    .write(&buf[..])
                    .expect("Failed to write output");
            }
        };
    }
    // println!("{:?}", stdout.);

    // println!("Error: {}", err);
}
