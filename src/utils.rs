use crate::directives::{AccBal, Directive};

pub fn print_directives(directives: &Vec<Directive>) {
    for d in directives {
        println!("{d}")
    }
}

pub fn print_bals(bals: AccBal) {
    println!("-- Balances --");
    for (acc, ccy_bals) in bals {
        for (ccy, amount) in ccy_bals {
            println!("{acc} {amount} {ccy}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn useless_print_directives() {
        let vec = Vec::new();
        print_directives(&vec)
    }
}
