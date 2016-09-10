mod thompson_nfa;

#[cfg(test)]
mod tests {
    fn compile_regexp_test() {
        let regexp = ::create::regexp_from_string("abc");
        let insts = super::thompson_nfa::compile_regexp(regexp);
    }
}
