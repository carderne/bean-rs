# bean-rs

Basic [beancount](https://github.com/beancount/beancount) clone (one day...) in Rust!

Still very very alpha and doesn't do most things that are necessary to be at all useful.

Python bindings are a WIP using [PyO3](https://pyo3.rs);

The libraries:
- Rust: [crates/bean-rs](https://crates.io/crates/bean-rs)
- Python: [pypi/bean-rs](https://pypi.org/project/bean-rs/)

Planned features:
- [x] Parse beancount files using [pest](https://pest.rs/)
- [x] Stricter transaction keywords
- [x] Propagate line numbers for debugging
- [x] Calculate account balances
- [x] Use proper Decimal handling
- [x] Validate transactions against `open`/`close` directives
- [x] Validate `balance` directives
- [x] Pad statements
- [x] Open/close with multiple currencies
- [x] Add Python bindings
- [ ] Support `includes`
- [ ] Come up with a more punny name
- [ ] Currency conversions
- [ ] Price/cost and FIFO

## (Deliberate) differences from beancount
- Postings can't omit the currency

## Use from Rust
### Install
```bash
cargo install bean-rs
```

### Run
```bash
$ bean-rs

Usage: bean-rs <COMMAND>

Commands:
  balance  Display account balances
  check    Check for errors and quit
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Calculate balances
```bash
bean-rs balance example.bean
```

## Use from Python
More to come...
```python
import bean_rs
ledger = bean_rs.py_load("example.bean")
print(ledger.opts)
```

## Development
### Build
```bash
make build
```

### Test
```bash
make test
```
