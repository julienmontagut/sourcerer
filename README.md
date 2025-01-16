# GitHub Multi CLI

An easy way to manage multiple Git repositories at once.

## Installation

Using [cargo](https://crates.io/):

```bash
cargo install srcman
```

## Usage

```bash
srcman [OPTIONS] [SUBCOMMAND]
```

## Subcommands

```bash
init        Initialize srcman in the current directory
list        List all repositories
fetch       Fetch all repositories
push        Push changes from all repositories that have unpushed changes
status      Show the status of all repositories
```
