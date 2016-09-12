use create::Regexp;
use thompson_nfa;

pub fn is_match(regexp: &Regexp, input: &str) -> bool {
    let insts = thompson_nfa::compile_regexp(regexp);
    let result = thompson_nfa::thompson_vm(&insts, input);
    result
}
