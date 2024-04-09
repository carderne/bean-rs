use std::collections::HashMap;

use bean_rs::data::AccBal;
use bean_rs::ledger::Ledger;
use bean_rs::{balance, load};
use rust_decimal::Decimal;

#[test]
fn test_load() {
    let text = std::fs::read_to_string("example.bean").expect("cannot read file");
    let Ledger {
        dirs,
        errs: _,
        opts: _,
    } = load(text);
    bean_rs::utils::print_directives(&dirs);
    // TODO check the output!
}

#[test]
fn test_balance() {
    let (bals, _) = balance("example.bean");
    let want: AccBal = HashMap::from([
        (
            "Assets:Invest".to_string(),
            HashMap::from([("GOO".to_string(), Decimal::new(111, 0))]),
        ),
        (
            "Income:Job".to_string(),
            HashMap::from([("GBP".to_string(), Decimal::new(-1000, 0))]),
        ),
        (
            "Equity:Bals".to_string(),
            HashMap::from([("GOO".to_string(), Decimal::new(-111, 0))]),
        ),
        (
            "Assets:Bank".to_string(),
            HashMap::from([("GBP".to_string(), Decimal::new(86000, 2))]),
        ),
        (
            "Expenses:Food".to_string(),
            HashMap::from([
                ("USD".to_string(), Decimal::new(4000, 2)),
                ("GBP".to_string(), Decimal::new(100, 0)),
            ]),
        ),
    ]);
    assert!(bals.eq(&want));
}
