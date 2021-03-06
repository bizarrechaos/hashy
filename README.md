# hashy

hashy is a command line tool that, when ran and given a filepath as an argument, will return multiple hashes for the file.
Supported hashes are md5, sha1, sha256, sha512, and ssdeep.

Currently all hashes are output, there is no way to filter the output.

The main goal of this project was to get familar with rust, while also creating a tool useful in malware analysis.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

What things you need to install the software and how to install them

```
rust
cargo
ssdeep
```

### Installing

A step by step series of examples that tell you how to get a development env running

Clone this repository

```
git clone https://github.com/bizarrechaos/hashy.git
```

Change directory to the project folder
```
cd ./hashy
```

Use cargo to build a binary

```
cargo build --release
```

Optionally copy the binary from the target directory to a folder in your $PATH
```
sudo cp ./target/release/hashy /usr/bin/
```

### Usage

![Example](./usage.svg)