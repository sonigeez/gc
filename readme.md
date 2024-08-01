
# gc (git clipper)

A simple command line tool to copy all files in a single string with respecting your .gitignore file.

## Installation

```bash
sudo curl -Lo /usr/local/bin/gc https://github.com/sonigeez/gc/releases/download/v0.0.1/gc

sudo chmod +x /usr/local/bin/gc
```


## Usage

```bash
gc [OPTIONS]
```

### Options

- `-h`, `--help`: Print help information
- `-V`, `--version`: Print version information
- `-p`, `--path`: Path to search for files
- `-H`, `--include-hidden`: Include hidden files
- `-i`, `--no-ignore`: Ignore .gitignore files
- `-f`, `--folders`: Specify folders to search for files



## Format of String
comment with folder path relative to current working directory
content of file
