extern crate clap;
extern crate exitcode;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use clap::{App, Arg};
use reqwest::Client;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, stderr, BufReader, Read, Result, Write};
use std::path::PathBuf;
use std::process;

#[derive(Deserialize, Debug)]
struct Gist {
    html_url: String,
}

fn read_stdin(buf: &mut String) -> Result<usize> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(buf)
}

fn read_file(path: &PathBuf, buf: &mut String) -> Result<usize> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    reader.read_to_string(buf)
}

fn main() {
    let args = App::new("Command line gist client")
        .version("v1.0.1")
        .author("Ben Wilber <benwilber@gmail.com>")
        .about("Create gists from the command line.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File to upload.  Defaults to stdin."),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("Filename of the gist.  Defaults to gist.txt for stdin."),
        )
        .arg(
            Arg::with_name("description")
                .short("d")
                .long("description")
                .takes_value(true)
                .help("Gist description."),
        )
        .arg(
            Arg::with_name("public")
                .short("p")
                .long("public")
                .help("Make the gist public."),
        )
        .get_matches();

    let username = env::var("GIST_USERNAME").unwrap_or_else(|_err| {
        writeln!(stderr(), "Github username and password required.")
            .expect("Failed to write to stderr.");
        process::exit(exitcode::USAGE);
    });

    let password = env::var("GIST_PASSWORD").unwrap_or_else(|_err| {
        writeln!(stderr(), "Github username and password required.")
            .expect("Failed to write to stderr.");
        process::exit(exitcode::USAGE);
    });

    let mut buf = String::new();
    let mut path = PathBuf::new();
    let mut filename = OsStr::new("gist.txt");
    match args.value_of("file") {
        None => {
            read_stdin(&mut buf).expect("Error reading stdin");
        }
        Some(f) => {
            path.push(f);
            filename = path.file_name().unwrap();
            read_file(&path, &mut buf).expect("Error reading file");
        }
    }

    let body = json!({
        "description": args.value_of("description").unwrap_or("gist"),
        "public": args.is_present("public"),
        "files": {
            args.value_of("name").unwrap_or_else(|| filename.to_str().unwrap()): {
                "content": buf
            }
        }
    });

    match Client::new() {
        Ok(client) => {
            let resp = client
                .post("https://api.github.com/gists")
                .basic_auth(username, Some(password))
                .json(&body)
                .send();
            match resp {
                Ok(mut r) => {
                    let gist: Gist = r.json().unwrap();
                    println!("{}", gist.html_url);
                }
                Err(e) => panic!("Got error: {:?}", e),
            }
        }
        Err(e) => panic!("Got error: {:?}", e),
    }
}
