from bean_rs._bean_rs import Directive

class Options:
    title: str
    operating_currency: str

class BeanError(Exception):
    pass

class Ledger:
    dirs: list[Directive]
    errs: list[BeanError]
    opts: Options

def load(path: str) -> Ledger:
    pass
