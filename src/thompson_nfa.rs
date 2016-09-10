use create::Regexp;

pub enum Inst {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize)
}

pub struct ThreadState {
    /// The compiled regexp program
    insts: Vec<Inst>,
    /// The program counters; indexes into insts
    pcs: Vec<usize>
}

impl ThreadState {
    pub fn from_regexp(regexp: Regexp) -> ThreadState {
        ThreadState { insts: compile_regexp(regexp),
                      pcs: vec![0] }
    }
}

pub fn update_threadstate(thread_state: &mut ThreadState, c: char) {
    let mut new_pcs = Vec::new();
    for index in &thread_state.pcs {
        let inst = &(thread_state.insts[*index]);
        match *inst {
            Inst::Char(inst_char) => if c == inst_char {
                new_pcs.push(index+1);
            },
            Inst::Match => (),
            Inst::Jump(jump_index) => new_pcs.push(jump_index),
            Inst::Split(s1_index, s2_index) => {
                new_pcs.push(s1_index);
                new_pcs.push(s2_index);
            }
        }
    }
    thread_state.pcs = new_pcs;
}

pub fn compile_regexp(regexp: Regexp) -> Vec<Inst> {
    let mut insts = compile_regexp_offset(regexp, 0);
    insts.push(Inst::Match);
    insts
}

fn compile_regexp_offset(regexp: Regexp, offset: usize) -> Vec<Inst> {
    let mut insts = Vec::new();
    use create::Regexp::*;
    use self::Inst::{Jump,Split};
    match regexp {
        Char(c) => {
            let char_inst = Inst::Char(c);
            insts.push(char_inst);
        },
        Concatenation(regexps) => {
            let mut num_insts = 0;
            for sub_regexp in regexps {
                let mut sub_insts = compile_regexp_offset(sub_regexp,
                                                          offset + num_insts);
                num_insts += sub_insts.len();
                insts.append(&mut sub_insts);
            }
        },
        Alternation(regexps) => {
            let mut alternatives_insts: Vec<Vec<Inst>> = Vec::new();
            for sub_regexp in regexps {
                let sub_insts = compile_regexp_offset(sub_regexp, offset);
                alternatives_insts.push(sub_insts);
            }
            let total_len = alternatives_insts.iter()
                .fold(0, |sum,x| sum + x.len())
                + alternatives_insts.len() - 1; // + alternates_insts.len() to account
                                              // for the Jump instruction we add
            for sub_insts in &mut alternatives_insts {
                let jump_to_end = Jump(total_len + offset);
                sub_insts.push(jump_to_end);
            }
        },
        Optional(inner_regexp) => {
            let mut inner_insts = compile_regexp_offset(*inner_regexp,
                                                        offset + 1);
            let split_inst = Split(offset + 1, offset + inner_insts.len() + 1);
            insts.push(split_inst);
            insts.append(&mut inner_insts);
        },
        Repeated(inner_regexp) => {
            let mut inner_insts = compile_regexp_offset(*inner_regexp,
                                                        offset);
            let split_inst = Split(offset, offset + inner_insts.len() + 2);
            insts.append(&mut inner_insts);
            insts.push(split_inst);
        },
        OptionalRepeated(inner_regexp) => {
            let mut inner_insts = compile_regexp_offset(*inner_regexp,
                                                        offset + 1);
            let split_inst = Split(offset, offset + inner_insts.len() + 2);
            let jump_inst = Jump(offset);
            insts.push(split_inst);
            insts.append(&mut inner_insts);
            insts.push(jump_inst);
        }
    }
    insts
}

// struct State {
//     c: Option<char>,
//     action: Action
// }

// enum Action {
//     AdvanceTo(Vec<Box<State>>),
//     Match
// }

// fn construct_state_tree<'a>(regexp: Regexp) -> Box<State> {
//     let match_state = Box::new(State {c: None, action: Action::Match });
//     construct_state_tree_next(regexp, match_state)
// }

// fn construct_state_tree_next<'a>(regexp: Regexp, next: Box<State>) -> Box<State> {
//     let match_state = Box::new(State {c: None, action: Action::Match });
//     use self::Action::*;
//     if let Regexp::Concatenation(regexps) = regexp {
//         let mut concated_states: Vec<Box<State>> = Vec::new();
//         for inner_regexp in regexps {
//             let sub_state = construct_state_tree(inner_regexp);
//             concated_states.push(sub_state);
//         }
//         let mut prev_last = next;
//         while concated_states.len() > 0 {
//             let mut last = concated_states.pop().unwrap();
//             match last.action {
//                 AdvanceTo(ref mut next_states) => {
//                     next_states.push(prev_last);
//                 },
//                 Match => panic!("Illegal State: Sub-state of concatenation cannot \
//                                  contain Match")
//             }
//             prev_last = last;
//         }
//         prev_last
//     } else if let Regexp::Alternation(regexps) = regexp {
//         let mut parts: Vec<Box<State>> = Vec::new();
//         for inner_regexp in regexps {
//             let sub_state =
//                 construct_state_tree_next(inner_regexp, next);
//             parts.push(sub_state);
//         }
//         let wrapping_state = State { c: None, action: AdvanceTo(parts) };
//         Box::new(wrapping_state)
//     } else {
//         panic!("Error");
//     }
//     match regexp {
//         Regexp::Concatenation(regexps) => {
//             let mut concated_states: Vec<Box<State>> = Vec::new();
//             for inner_regexp in regexps {
//                 let sub_state = construct_state_tree(inner_regexp);
//                 concated_states.push(sub_state);
//             }
//             let mut prev_last = next;
//             while concated_states.len() > 0 {
//                 let mut last = concated_states.pop().unwrap();
//                 match last.action {
//                     AdvanceTo(ref mut next_states) => {
//                         next_states.push(prev_last);
//                     },
//                     Match => panic!("Illegal State: Sub-state of concatenation cannot \
//                                      contain Match")
//                 }
//                 prev_last = last;
//             }
//             prev_last
//         },
//         Regexp::Alternation(regexps) => {
//             let mut parts: Vec<Box<State>> = Vec::new();
//             for inner_regexp in regexps {
//                 let sub_state =
//                     construct_state_tree_next(inner_regexp, next);
//                 parts.push(sub_state);
//             }
//             let wrapping_state = State { c: None, action: AdvanceTo(parts) };
//             Box::new(wrapping_state)
//         }
//         Regexp::Char(c) => Box::new(State {c: Some(c), action: AdvanceTo(vec![next])}),
//         Regexp::Optional(inner_regexp) => {
//             let inner_state = construct_state_tree_next(*inner_regexp, next);
//             let optional_state = Box::new(State { c: None,
//                                              action: AdvanceTo(vec![inner_state,
//                                                                     next])});
//             optional_state
//         },
//         Regexp::Repeated(inner_regexp) => {
//             let inner_state = construct_state_tree_next(*inner_regexp, next);
//             let mut repeated_state = Box::new(State { c: None,
//                                              action: AdvanceTo(vec![inner_state,
//                                                                     next])});
//             match repeated_state.action {
//                 AdvanceTo(ref mut next_states) => next_states.push(repeated_state),
//                 Match => unreachable!()
//             }
//             repeated_state
//         },
//         _ => panic!("Hello")
        
//     }
// }
