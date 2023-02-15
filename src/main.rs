use std::io::Write;
use std::process::{exit, Child, ChildStdout, Command, Stdio};

use nonblock::NonBlockingReader;

fn main() {
    let mut args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() < 2 {
        println!("usage: spin <num copies> <cmd> [args...]");
        exit(1);
    }
    let num_copies: u32 = args
        .remove(0)
        .to_string_lossy()
        .parse()
        .expect("Invalid value for number of copies");
    let cmd = args.remove(0);

    // Spin up N copies of the command
    let mut children: Vec<(Child, NonBlockingReader<ChildStdout>)> = vec![];
    let mut cmd = Command::new(cmd);
    cmd.args(&args);
    cmd.stdin(Stdio::null());
    for _ in 0..num_copies {
        cmd.stdout(Stdio::piped());
        let mut handle = match cmd.spawn() {
            Ok(handle) => handle,
            Err(err) => panic!("Failed to spawn: {err:?}"),
        };
        let Some(stdout) = handle.stdout.take() else {
            panic!("Failed to get stdout from child");
        };
        let Ok(nonblock_stdout) = NonBlockingReader::from_fd(stdout) else {
            panic!("Failed to make non-blocking reader for stdout");
        };

        children.push((handle, nonblock_stdout));
    }

    // Children are removed from this list when they stop producing output; as long as we have
    // anything left in the Vec, there is work to do.
    while !children.is_empty() {
        // Consume the output of each of the child processes. We iterate through the list in reverse
        // order by index so we can remove children that have finished without disturbing the other
        // children we haven't iterated over yet.
        let mut remaining_children = vec![];
        for (handle, mut nonblock_stdout) in children.into_iter() {
            let mut buf = vec![];
            let bytes_read = nonblock_stdout
                .read_available(&mut buf)
                .expect("Failed to read from child process");

            if bytes_read > 0 {
                match String::from_utf8(buf) {
                    Ok(str) => {
                        // prefix every line of output with the child's process number
                        let prefixed_str = str
                            .trim()
                            .split("\n")
                            .map(|line| format!("[{}] {}", handle.id(), line))
                            .collect::<Vec<String>>()
                            .join("\n");
                        println!("{prefixed_str}");
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

            if !nonblock_stdout.is_eof() {
                remaining_children.push((handle, nonblock_stdout));
            }
        }
        children = remaining_children;
    }
}
