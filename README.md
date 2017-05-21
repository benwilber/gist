# gist
Command line gist client

# Usage
```
$ gist -h
Command line gist client 0.1.0
Ben Wilber <benwilber@gmail.com>
Upload gists from the command line

USAGE:
    gist [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -p, --public     Make this a public gist.
    -V, --version    Prints version information

OPTIONS:
    -d, --description <description>    Gist description.
    -f, --file <file>                  File to upload.  Defaults to stdin.
    -n, --name <name>                  Filename of the gist.
```

# Example
```
$ export GITHUB_USERNAME=benwilber
$ export GITHUB_PASSWORD=<password>
$ gist --public --name main.rs --description "My first rust program" --file src/main.rs
https://gist.github.com/f0b0b2934f6b6ba735711c13b7bf87da
```