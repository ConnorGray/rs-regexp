#[test]
fn test_regexp_parser() {
    let regexp_strings = vec![
        "(a+b)",
        "(a|b)|(c|d)",
        "((a|b)|(c|d)+)+",
        "((a|b))",
        "((a))",
        "((a)",
        "(a))",
        "((a|b)b|(c+d|q))+",
        "(abc)(123)+",
    ];
}

#[test]
fn test_regexs_error_detection() {
    let regexp_strings = vec![
        "a+b",
        "((a|b)|(c|d))",
        "((a|b)|(c|d)+)+",
        "(a|b)",
        "a",
        "((a)",
        "(a))",
        "((a|b)b|(c+d|q))+",
    ];

    for regexp_string in regexp_strings.iter() {
        let regexp = regexp_from_string(&regexp_string);
        let output_string = regexp_to_string(&regexp);

        assert_eq!(**regexp_string, *output_string);
    }
}
