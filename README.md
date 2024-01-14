# beaners

Basic [beancount](https://github.com/beancount/beancount) clone (one day...) in Rust!

Find it [here on crates.io](https://crates.io/crates/beaners).

Using [pest](https://pest.rs/) for parsing. Two useful links:
- [pest bootstrap parsing](https://github.com/pest-parser/pest/tree/master/meta/src)
- [playground](https://pest.rs/#editor)

Planned featuers:
- [x] Parse beancount files
- [x] Stricter transaction keywords
- [x] Propagate line numbers for debugging
- [ ] Calculate account balances
- [ ] Use proper Decimal handling
- [ ] Validate transactions against `open`/`close` directives
- [ ] Validate `balance` directives
- [ ] Open/close with multiple currencies

## Usage
### Install
```bash
cargo install beaners
```

### Run
```
$ beaners

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
