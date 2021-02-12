# CFF - Canonical File Finder

## Readme

This is a cli app that scans given directory for files with unique hash contents and outputs the paths to "canonical" files,
ready to use with some specialized file manipulation utilities such as cp or mv.

See the excerpt from the `--help` option below:

```
USAGE:
    cff [FLAGS] <INPUT>

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Scan directories recursively (false by default)
    -V, --version      Prints version information

ARGS:
    <INPUT>    directory to scan for duplicates
```

## Motivation

This project has been implemented to refresh a bit on Rust after a brake I had since last time I tried it in October.
