# pushenv
A CLI utility that reads a .env file before starting a process

## Installation

You can install `pushenv` using Cargo, the Rust package manager, or by
downloading a precompiled binary from the latest release.

```bash
cargo install pushenv
```

## Example usage

### Without specifying an env file

This will attempt to read the `.env` file in the current directory.

```bash
pushenv -- echo $SOME_VAR
```

### Specifying an env file

You can specify an env file to read from as the first argument.

```bash
pushenv some.env.file -- echo $SOME_VAR
```
