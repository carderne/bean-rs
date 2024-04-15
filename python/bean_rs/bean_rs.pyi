import datetime

class Options:
    title: str
    operating_currency: str

class BeanError(Exception):
    pass

class Directive:
    date: datetime.date

class Ledger:
    dirs: list[Directive]
    errs: list[BeanError]
    opts: Options

def load(path: str) -> Ledger:
    pass
