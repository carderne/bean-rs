use crate::directives::Directive;

pub fn print_directives(directives: Vec<Directive>) {
    for d in directives {
        println!("{d}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn useless_print_directives() {
        let vec = Vec::new();
        print_directives(vec)
    }
}
