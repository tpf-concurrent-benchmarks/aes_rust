# AES in Rust

## Objective

This is a Rust implementation of the [Advanced Encryption Standard (AES)](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.197.pdf).

The objective of this project is to analyze the concurrent capabilities of Rust, under various workloads, and to compare the performance between different languages.

## Deployment

### Requirements

- [Docker](https://www.docker.com/)

> The project is fully containerized, so you don't need to have Rust installed on your machine.

### Configuration

The configuration is done through the `.env` file. The following variables are available:

- `N_THREADS`: Number of threads to be used in the encryption process
- `REPEAT`: Number of times the encryption/decryption process will be repeated
- `PLAIN_TEXT`: Path to the file with the data to be encrypted
- `ENCRYPTED_TEXT`: Path to the file where the encrypted data will be stored
- `DECRYPTED_TEXT`: Path to the file where the decrypted data will be stored

> Having a `PLAIN_TEXT` and `ENCRYPTED_TEXT` will mean encrypting the data, while having a `ENCRYPTED_TEXT` and `DECRYPTED_TEXT` will mean decrypting the data. Having all three will mean encrypting and decrypting the data.

### Commands

#### Setup

- `make setup`: Starts docker and creates needed directories
- `make dummy_file`: Creates a dummy with data to be encrypted
- `make build`: Builds the image with the binary

#### Run

- `make deploy`: Runs the container and monitoring services
- `make remove`: Stops and removes the container and services
- `make run`: Runs only the encryption process
- `make test`: Runs the tests

## Libraries

- [Rayon](https://docs.rs/rayon/latest/rayon/)
