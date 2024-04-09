use bean_rs;

#[test]
fn test_load() {
    let text = std::fs::read_to_string("example.bean").expect("cannot read file");
    let (dirs, _) = bean_rs::load(text);
    bean_rs::utils::print_directives(&dirs);
    // TODO check the output!
}
