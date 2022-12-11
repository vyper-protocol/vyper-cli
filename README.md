# Vyper-Cli

Command Line to interact with on chain Vyper program suite

[![rust checks](https://github.com/vyper-protocol/vyper-cli/actions/workflows/rust-checks.yml/badge.svg)](https://github.com/vyper-protocol/vyper-cli/actions/workflows/rust-checks.yml)

[![deploy cargo docs](https://github.com/vyper-protocol/vyper-cli/actions/workflows/cargo-docs.yml/badge.svg)](https://github.com/vyper-protocol/vyper-cli/actions/workflows/cargo-docs.yml)

## Installation

### 1. Download the source code.

```bash
$ git clone git@github.com:vyper-protocol/vyper-cli.git
$ cd vyper-cli
```

### 2. Build

```bash
$ cargo build
```

### 3. Install

```bash
$ cargo install --path .
```

## Usage

```bash
$ vyper <option> <command>
```

## Example

##### Get current version

```bash
$ vyper --version
```

##### Fetch tranche configuration

```bash
$ vyper core fetch 5Q8tNcpKjBSGHyW3LEr1rGAJeuuLWCbYB5frL79XacRC
```
