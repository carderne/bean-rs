# bean-rs

Basic [beancount](https://github.com/beancount/beancount) clone (one day...) in Rust!

Using [pest](https://pest.rs/) for parsing. Two useful links:
- [pest bootstrap parsing](https://github.com/pest-parser/pest/tree/master/meta/src)
- [playground](https://pest.rs/#editor)

Planned features:
- [x] Parse beancount files
- [x] Stricter transaction keywords
- [x] Propagate line numbers for debugging
- [x] Calculate account balances
- [x] Use proper Decimal handling
- [x] Validate transactions against `open`/`close` directives
- [x] Validate `balance` directives
- [x] Pad statements
- [x] Open/close with multiple currencies
- [ ] Currency conversions
- [ ] Price/cost and FIFO

## Usage
### Install
```bash
cargo install bean-rs
```

### Run
```
$ bean-rs

Commands:
  balance
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Development
### Build
```bash
make build
```
