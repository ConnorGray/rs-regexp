mod create;
mod thompson_nfa;

pub use self::create::*;
pub use self::thompson_nfa::*;

#[cfg(test)]
mod tests {
    use self::create;
        
    fn compile_regexp_test() {
        let regexp = create::regexp_from_string("abc");
        let insts = super::thompson_nfa::compile_regexp(regexp);
    }
}
