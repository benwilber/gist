extern crate clap;
extern crate exitcode;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, stderr, Write, Read, BufReader, Result};
use std::path::PathBuf;
use std::process;
use clap::{Arg, App};
use reqwest::Client;

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
    match File::open(path) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            reader.read_to_string(buf)
        }
        Err(e) => Err(e),
    }
}

fn main() {

    let args = App::new("Command line gist client")
        .version("v1.0.0")
        .author("Ben Wilber <benwilber@gmail.com>")
        .about("Upload gists from the command line")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("File to upload.  Defaults to stdin"))
        .arg(Arg::with_name("name")
                 .short("n")
                 .long("name")
                 .takes_value(true)
                 .help("Filename of the gist"))
        .arg(Arg::with_name("description")
                 .short("d")
                 .long("description")
                 .takes_value(true)
                 .help("Gist description"))
        .arg(Arg::with_name("public")
                 .short("p")
                 .long("public")
                 .help("Make the gist public"))
        .get_matches();

    let username = match env::var("GIST_GITHUB_USERNAME") {
        Ok(username) => username,
        Err(_) => {
            writeln!(stderr(), "Github username and password required.")
                .expect("Failed to write to stderr.");
            process::exit(exitcode::USAGE);
        }
    };
    let password = match env::var("GIST_GITHUB_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            writeln!(stderr(), "Github username and password required.")
                .expect("Failed to write to stderr.");
            process::exit(exitcode::USAGE);
        }
    };

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
            args.value_of("name").unwrap_or(filename.to_str().unwrap()): {
                "content": buf
            }
        }
    });

    let url = "https://api.github.com/gists";
    match Client::new() {
        Ok(client) => {
            let resp = client
                .post(url)
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
