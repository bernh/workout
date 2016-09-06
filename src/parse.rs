fn preprocess_input(input: String) -> String {
    input.split_whitespace().collect()
}

#[test(cfg)]
#[test]
fn preprocess() {
    assert_eq!(preprocess_input("3 H + 4*(300 H + 200jg)".to_string()),
               "3H+4*(300H+200jg)");
}
