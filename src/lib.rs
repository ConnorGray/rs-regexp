mod create;
mod thompson_nfa;
mod matching;

pub use self::create::*;
pub use self::thompson_nfa::*;
pub use self::matching::*;

#[cfg(test)]
mod tests {
    use super::create::*;
    use super::thompson_nfa::*;
    use super::matching::*;
        
    #[test]
    fn compile_regexp_test() {
        // let regexp = create::regexp_from_string("abc").unwrap();
        // let insts = super::thompson_nfa::compile_regexp(regexp);
    }

    #[test]
    fn thompson_vm_match_tests() {
        let test_cases = vec![
            // (regexp, input, extected result)
            ("a", "a", true),
            ("a", "b", false),
            ("aa", "aa", true),
            ("aa", "a", false),
            // Optional tests
            ("a?b", "ab", true),
            ("a?b", "b", true),
            // Repeated tests
            ("a+b", "aab", true),
            ("a+b", "b", false),
            // OptionalRepeated tests
            ("a*b", "aab", true),
            ("a*b", "b", true),
            // Multi-operator tests
            ("a+b*c", "aac", true),
            ("a+b*c", "aabbc", true),
            ("a+b*c", "aabb", false),
            ("(ab)+cd", "ababcd", true),
            ("colou?r", "color", true),
            ("colou?r", "colour", true),
        ];

        println!("");

        for (i, test_case) in test_cases.iter().enumerate() {
            let regexp = regexp_from_string(test_case.0).unwrap();
            let input = test_case.1;
            let expected_result = test_case.2;

            let insts = compile_regexp(&regexp);

            let result = thompson_vm(&insts, input);
            if result != expected_result {
                let error_message = format!(
                    "\t=== Regexp:\t\t\"{}\" > {:?}\n\
                     \t=== Compiled:\t\t{:?}\n\
                     \t=== Input:\t\t{}\n\
                     \t=== Expected Result:\t{}\n\
                     \t=== Actual Result:\t{}\n",
                    test_case.0, regexp,
                    insts,
                    input,
                    expected_result,
                    result);
                panic!("Unexpected Regexp match result:\n{}", error_message);
            }
        }
    }

    #[test]
    fn matching_test() {
        // let regexp = regexp_from_string("a+q?c?").unwrap();
        // let string = "aqab";
        // let expected_match = "aq";
        // let test_cases = vec![
        //     // (regexp, input, expected output)
        //     ("a", "a", "a"),
        //     ("a+", "aa", "aa"),
        //     ("a?", "ab", "a"),
        //     ("a+", "aba", "a"),
        //     ("a+b?", "aaab", "aaab"),
        //     ("b?", "a", ""),
        //     ("b?a+c", "baaac", "baaac"),
        //     ("b?a+c", "aac", "aac")
        // ];
        // for test_case in test_cases {
        //     let regexp = regexp_from_string(test_case.0).unwrap();
        //     let input = test_case.1;
        //     let expected_match = test_case.2;
        //     match first_match_anchored_start(&regexp, input) {
        //         Some(matched_text) => if matched_text != expected_match {
        //             panic!("\n\tBad regexp match for regexp {:?}: \
        //                     matched_text({}) != expected_match({})",
        //                    regexp, matched_text, expected_match)
        //         },
        //         None => panic!("Regexp({}, {:?}) found no match on \"{}\" \
        //                         when \"{}\" was expected",
        //                        test_case.0, regexp, test_case.1, test_case.2)
        //     }
        // }
    }

    #[test]
    fn instruction_generation() {
        use Inst::*;
        let con_insts = compile_regexp(&regexp_from_string("abc").unwrap());
        let alt_insts = compile_regexp(&regexp_from_string("a|b|c").unwrap());
        let opt_insts = compile_regexp(&regexp_from_string("a?(bc)?").unwrap());
        let rep_insts = compile_regexp(&regexp_from_string("a+(bc)+").unwrap());
        let oprep_insts = compile_regexp(&regexp_from_string("a*(bc)*").unwrap());

        assert_eq!(con_insts, vec![Char('a'), Char('b'), Char('c'), Match]);
        assert_eq!(alt_insts, vec![Char('a'), Jump(5),
                                   Char('b'), Jump(5),
                                   Char('c'), Match]);
        assert_eq!(opt_insts, vec![Split(1, 2), Char('a'),
                                   Split(3, 5),
                                   Char('b'), Char('c'),
                                   Match]);
        assert_eq!(rep_insts, vec![Char('a'), Split(0, 2),
                                   Char('b'), Char('c'), Split(2, 5),
                                   Match]);
        assert_eq!(oprep_insts, vec![Split(1, 3), Char('a'), Jump(0),
                                     Split(4, 7), Char('b'), Char('c'), Jump(3),
                                     Match])
    }
}













