from bean_rs import load
from bean_rs._bean_rs import Transaction


def test_load() -> None:
    ledger = load("example.bean")
    assert ledger.opts.operating_currency == "GBP"
    for d in ledger.dirs:
        match d[0]:  # each element in enum is single-element tuple
            case Transaction(payee='"Shop"', narration=narration):
                assert narration == '"More food"'
            case _:
                pass
