# sq-32

> **sq-32** - *Named for the fact that a standard checkerboard is 8x8, yet draughts only uses 32 out of the 64 squares.*

sq-32 is a draughts/checkers engine written in Rust. Currently, it also comes packaged with a CLI interface for interacting with the engine directly.

## Installation

Currently, the only way to use sq-32 is to build from source. This requires a local [Rust environment](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/ProspectPyxis/sq-32.git
# Or download the source code from the latest release and untar

cd sq-32/
cargo build --release
# Due to the program's very early alpha stage, installing directly is not recommended yet!

# To run the program:
./target/release/sq-32
```

Building only the sq-32 library for use with other crates is currently untested.

## Usage

-- WIP --

## Roadmap

The program is currently in a very heavily alpha stage, and things are expected to rapidly change as progress is made. Current potential plans for the future include:

- Making a WebAssembly version of the engine, to allow it to be used for online projects more easily.
- Migrate the program to use an established communication protocol, such as Hub-1.
- Split non-essential CLI features (such as pretty board printing) into its own draughts GUI that can interoperate with other engines.
- Make the engine flexible enough to allow analysis of other draughts variants, particularly international draughts.

## Contributing

-- WIP --

## License

The project is licensed under [GNU GPL v3](LICENSE.md).
