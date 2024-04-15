from bean_rs import load


def test_load() -> None:
    ledger = load("example.bean")
    assert ledger.opts.operating_currency == "GBP"
