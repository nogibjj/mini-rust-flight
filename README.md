# Flight Status Checker

A command-line interface (CLI) program written in Rust that checks flight status using the AviationStack API.

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) programming language
- [AviationStack API Key](https://aviationstack.com/) - Sign up for a free account to get an API key.

## Installation

1. Clone this repository:

```
git clone https://github.com/your-username/flight-status-checker.git
```
2. Change to the project directory:
```
cd flight-status-checker
```
3. Add your AviationStack API key to the src/main.rs file:
```
let api_key = "your_api_key";
```
Replace your_api_key with your actual API key.

4. Build the program:

```
cargo build --release
```

## Usage
To check the status of a flight, run the following command:

```
./target/release/flight_status_checker --flight <FLIGHT_CODE>
```
Replace <FLIGHT_CODE> with the flight code you want to check (e.g., AA123).

For help with command-line options, run:

```
./target/release/flight_status_checker --help
```



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
