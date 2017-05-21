extern crate clap;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{self, Read, BufReader};
use clap::{Arg, App};
use reqwest::Client;

#[derive(Deserialize, Debug)]
struct Gist {
    html_url: String,
}

fn read_stdin(to_buf: &mut String) -> Result<usize, io::Error> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(to_buf)
}

fn read_file(path: String, to_buf: &mut String) -> Result<usize, io::Error> {
    match File::open(path) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            reader.read_to_string(to_buf)
        },
        Err(e) => Err(e)
    }
}

fn main() {

    let args = App::new("Command line gist client")
        .version("0.1.0")
        .author("Ben Wilber <benwilber@gmail.com>")
        .about("Upload gists from the command line")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("File to upload.  Defaults to stdin."))
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .takes_value(true)
            .help("Filename of the gist."))
        .arg(Arg::with_name("description")
            .short("d")
            .long("description")
            .takes_value(true)
            .help("Gist description."))
        .arg(Arg::with_name("public")
            .short("p")
            .long("public")
            .help("Make this a public gist."))
        .get_matches();

    let username = match env::var("GITHUB_USERNAME") {
        Ok(username) => username,
        Err(_) => panic!("Github username and password required.")
    };
    let password = match env::var("GITHUB_PASSWORD") {
        Ok(password) => password,
        Err(_) => panic!("Github username and password required.")
    };

    let mut buf = String::new();
    match args.value_of("file") {
        None => {
            match read_stdin(&mut buf) {
                Ok(_) => {},
                Err(e) => panic!("Got error: {:?}", e)
            }
        },
        Some(path) => {
            match read_file(String::from(path), &mut buf) {
                Ok(_) => {},
                Err(e) => panic!("Got error: {:?}", e)
            }
        }
    }

    let body = json!({
        "description": args.value_of("description").unwrap_or("gist"),
        "public": args.is_present("public"),
        "files": {
            args.value_of("name").unwrap_or("gist.txt"): {
                "content": buf
            }
        }
    });

    let url = "https://api.github.com/gists";
    match Client::new() {
        Ok(client) => {
            let resp = client.post(url)
                .basic_auth(username, Some(password))
                .json(&body)
                .send();
            match resp {
                Ok(mut r) => {
                    let gist: Gist = r.json().unwrap();
                    println!("{}", gist.html_url);
                },
                Err(e) => panic!("Got error: {:?}", e)
            }
        },
        Err(e) => panic!("Got error: {:?}", e)
    }

}
