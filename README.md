# gist
Command line gist client

[![Build Status](https://travis-ci.org/benwilber/gist.svg?branch=master)](https://travis-ci.org/benwilber/gist)

# Usage
```shell
$ gist -h
Command line gist client 0.1.0
Ben Wilber <benwilber@gmail.com>
Upload gists from the command line

USAGE:
    gist [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -p, --public     Make this a public gist
    -V, --version    Prints version information

OPTIONS:
    -d, --description <description>    Gist description
    -f, --file <file>                  File to upload.  Defaults to stdin
    -n, --name <name>                  Filename of the gist
```

# Example
```shell
$ export GITHUB_USERNAME=benwilber
$ export GITHUB_PASSWORD=<personal-access-token>
$ gist --public --name main.rs --description "My first rust program" --file src/main.rs
https://gist.github.com/f0b0b2934f6b6ba735711c13b7bf87da
```

You will want to create a [Personal Access Token](https://github.com/blog/1509-personal-api-tokens) to use instead of your normal Github password.  It should only have the `gist` scope.

# Building
Build with `cargo`
```shell
$ cargo build
```
