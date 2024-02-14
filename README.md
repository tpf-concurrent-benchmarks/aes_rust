# AES in Rust

This is a Rust implementation of the [Advanced Encryption Standard (AES)](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.197.pdf).

The objective of this project is to analyze the concurrent capabilities of Rust, under various workloads, and to compare the performance between different languages.

## Requirements

- [Docker](https://www.docker.com/)

The project is containerized, so you don't need to have Rust installed on your machine.

## Usage

### Build

Builds the image with the binary:
```bash
make build
```

### Run

Runs the binary:
```bash
make run
```

### Tests

Runs the tests (also in a container)
```bash
make test
```
