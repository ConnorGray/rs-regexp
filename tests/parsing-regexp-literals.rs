extern crate regexp;
use regexp::*;

#[test]
fn test_regexp_parser() {
    use regexp::Regexp::*;
    // let regexp_strings = vec![
    //     "(a+b)",
    //     "(a|b)|(c|d)",
    //     "((a|b)|(c|d)+)+",
    //     "((a|b)b|(c+d|q))+",
    //     "(abc)(123)+",
    // ];

    let a = Concatenation(vec![Alternation(vec![Char('b'), Char('a')]),
                               Char('b'),
                               Char('c')]);
    let b = Concatenation(vec![Alternation(vec![Char('b'), Char('a')]),
                               Char('b'),
                               Char('c')]);

    assert_eq!(a, b);
}

#[test]
fn test_regexp_simplification() {
    let pairs = vec![
        ("((a))", "a"),
        ("((a|b))", "(a|b)"),
        ("((a)(b))", "ab"),
        ("((a?)(b)+)", "a?b+"),
        ("(((a)?+|b(c+))|(((d)+)d+))", "((a?+|bc+)|d+d+)")
    ];

    for pair in pairs {
        let regexp = Regexp::from_string(&(pair.0)).unwrap();
        let regexp_string = regexp_to_string(&regexp);
        println!("\tinput:\t\t{},\n\
                  \texpected ouput:\t{}\n\
                  \tactual output:\t{}", pair.0, pair.1, regexp_string);
        assert_eq!(pair.1, regexp_string);
    }
}


#[test]
fn test_regexs_error_detection() {
    use regexp::RegexpError::*;
    
    let pairs = vec![
        ("((a)", UnmatchedParenthesis(0)),
        ("(a))", UnmatchedParenthesis(3)),
        ("(((a))((b())", UnmatchedParenthesis(6)),
    ];

    for pair in pairs.iter() {
        let err = Regexp::from_string(&(pair.0)).unwrap_err();
        assert_eq!(pair.1, err);
    }
}
