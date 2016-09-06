use JD_grammar;

fn preprocess_input(input: String) -> String {
    input.split_whitespace().collect()
}

#[test(cfg)]
#[test]
fn preprocess() {
    assert_eq!(preprocess_input("3 H + 4*(300 H + 200jg)".to_string()),
               "3H+4*(300H+200jg)");
}

#[test]
fn simple_step() {
    let r = JD_grammar::parse_Step("3E");
    // TODO clone() really needed?
    assert_eq!(r.clone().unwrap().distance, 3.0);
    assert_eq!(r.clone().unwrap().speed, 3.0);
}
