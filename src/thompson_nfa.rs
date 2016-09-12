use create::Regexp;

#[derive(Debug, PartialEq)]
pub enum Inst {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize)
}

#[derive(Debug, PartialEq)]
pub struct Thread {
    pub saw_char: bool,
    pub pc: usize
}


pub fn thompson_vm(insts: &Vec<Inst>, input: &str) -> bool {
    fn addthread(list: &mut Vec<Thread>, thread: Thread) {
        let mut contains = false;
        for elem in list.iter() {
            if thread == *elem { contains = true; break; }
        }
        if !contains { list.push(thread); }
    }

    let mut cur_stack: Vec<Thread> = Vec::new();
    let mut new_stack: Vec<Thread> = Vec::new();

    addthread(&mut cur_stack, Thread { saw_char: false, pc: 0});

    for (_, cur_char) in input.chars().enumerate() {
        let mut i = 0;

        loop {
            if i >= cur_stack.len() { break; }

            let thread_pc: usize = cur_stack[i].pc;
            let thread_saw_char: bool = cur_stack[i].saw_char;
            let inst = &insts[thread_pc];

            use Inst::*;
            match *inst {
                Char(c) => {
                    if thread_saw_char {
                        addthread(&mut new_stack, Thread { saw_char: false,
                                                      pc: thread_pc });
                    } else {
                        if c == cur_char {
                            addthread(&mut cur_stack, Thread { saw_char: true,
                                                          pc: thread_pc + 1});
                        }
                    }
                },
                Match => return true,
                Jump(jump_pc) => {
                    addthread(&mut cur_stack,
                              Thread { saw_char: thread_saw_char,
                                       pc: jump_pc });
                },
                Split(s1_pc, s2_pc) => {
                    addthread(&mut cur_stack,
                              Thread { saw_char: thread_saw_char,
                                       pc: s1_pc});
                    addthread(&mut cur_stack,
                              Thread { saw_char: thread_saw_char,
                                       pc: s2_pc});
                }
            }
            
            i += 1
        }
        cur_stack = new_stack;
        new_stack = Vec::new();
    }
    false
}

pub fn compile_regexp(regexp: &Regexp) -> Vec<Inst> {
    let mut insts = compile_regexp_offset(&regexp, 0);
    insts.push(Inst::Match);
    insts
}

fn compile_regexp_offset(regexp: &Regexp, offset: usize) -> Vec<Inst> {
    let mut insts = Vec::new();
    use create::Regexp::*;
    use self::Inst::{Jump,Split};
    match *regexp {
        Char(c) => {
            let char_inst = Inst::Char(c);
            insts.push(char_inst);
        },
        Concatenation(ref regexps) => {
            let mut num_insts = 0;
            for sub_regexp in regexps {
                let mut sub_insts = compile_regexp_offset(&sub_regexp,
                                                          offset + num_insts);
                num_insts += sub_insts.len();
                insts.append(&mut sub_insts);
            }
        },
        Alternation(ref regexps) => {
            let num_alternatives = regexps.len();
            let mut alternatives_insts: Vec<Vec<Inst>> = Vec::new();
            for sub_regexp in regexps {
                let sub_insts = compile_regexp_offset(&sub_regexp, offset);
                alternatives_insts.push(sub_insts);
            }
            let total_len = alternatives_insts.iter()
                .fold(0, |sum,x| sum + x.len())
                + alternatives_insts.len() - 1; // + alternates_insts.len() - 1
                                                // accounts for the Jumps we add
            for (i, sub_insts) in alternatives_insts.iter_mut().enumerate() {
                // Add a jump instruction to all but the last alternative
                if i < num_alternatives - 1 {
                    let jump_to_end = Jump(total_len + offset);
                    sub_insts.push(jump_to_end);
                }
                insts.append(sub_insts);
            }
        },
        Optional(ref inner_regexp) => {
            let mut inner_insts = compile_regexp_offset(&*inner_regexp,
                                                        offset + 1);
            let split_inst = Split(offset + 1, offset + inner_insts.len() + 1);
            insts.push(split_inst);
            insts.append(&mut inner_insts);
        },
        Repeated(ref inner_regexp) => {
            let mut inner_insts = compile_regexp_offset(&*inner_regexp,
                                                        offset);
            let split_inst = Split(offset, offset + inner_insts.len() + 1);
            insts.append(&mut inner_insts);
            insts.push(split_inst);
        },
        OptionalRepeated(ref inner_regexp) => {
            let mut inner_insts = compile_regexp_offset(&*inner_regexp,
                                                        offset + 1);
            let split_inst = Split(offset + 1, offset + inner_insts.len() + 2);
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
