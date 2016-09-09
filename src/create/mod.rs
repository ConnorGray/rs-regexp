use std::fmt;

#[derive(Debug)]
pub enum Regexp {
    Char(char),
    Concatenation(Vec<Regexp>),
    Alternation(Vec<Regexp>),
    Optional(Box<Regexp>),
    Repeated(Box<Regexp>),
    OptionalRepeated(Box<Regexp>),
}

// abc+
// a(bc)*
// as(c|d*)+
// colou?r
// hello,? (W|w)olrd

use std::result::Result;

#[derive(Debug)]
pub enum RegexpError {
    EmptyRegexp,
    EmptyGroup(usize),
    EmptyAlternative(usize),
    MisplacedOperator(usize),
    UnmatchedParenthesis(usize),
}

impl fmt::Display for RegexpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn regexp_from_string(string: &str) -> Result<Regexp, RegexpError> {
    println!("regexp_from_string called with: {}", string);
    let mut stack = Vec::new();
    let mut escaped = false;
    let mut depth = 0;
    let mut group = String::new();
    let mut num_alternatives = 0;
    let mut last_open_paren_index = 0;

    use Regexp::*;
    use RegexpError::*;

    for (i, c) in string.chars().enumerate() {
        if escaped {
            match c {
                '('|')'|'?'|'+'|'*'|'\\' => stack.push(Char('\\')),
                _ => ()
            }
            stack.push(Char(c));
            escaped = false;
            continue;
        }

        match c {
            '(' => { if depth > 0 { group.push('(') };
                     depth += 1;
                     last_open_paren_index = i;
                     continue; },
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Result::Err(UnmatchedParenthesis(i));
                } else if depth == 0 {
                    println!("group: {}", group);
                    let group_regexp = match regexp_from_string(&group) {
                        Ok(value) => value,
                        Err(err) => return Result::Err(match err {
                            EmptyRegexp => EmptyGroup(i-1),
                            EmptyGroup(inner_i)
                                => EmptyGroup(inner_i + i - group.len()),
                            EmptyAlternative(inner_i)
                                => EmptyAlternative(inner_i + i - group.len()),
                            MisplacedOperator(inner_i)
                                => MisplacedOperator(inner_i + i - group.len()),
                            UnmatchedParenthesis(inner_i)
                                => UnmatchedParenthesis(inner_i + i - group.len())
                        })
                    };
                    stack.push(group_regexp);
                } else {
                    group.push(')');
                }
                continue;
            }
            _ => ()
        }
        if depth != 0 {
            group.push(c);
            continue;
        }
        match c {
            '|' => {
                let mut alternative_stack = Vec::new();
                while stack.len() > num_alternatives {
                    alternative_stack.push(stack.pop().unwrap());
                }
                alternative_stack.reverse();
                match alternative_stack.len() {
                    0 => return Result::Err(EmptyAlternative(i)),
                    1 => stack.push(alternative_stack.pop().unwrap()),
                    _ => stack.push(Concatenation(alternative_stack))
                }
                num_alternatives += 1;
            }
            '?' | '+' | '*' => {
                if stack.len() == num_alternatives {
                    return Result::Err(MisplacedOperator(i));
                }
                let prev_regexp = Box::new(match stack.pop() {
                    Some(value) => value,
                    None => return Result::Err(MisplacedOperator(i))
                });
                stack.push(match c {
                    '?' => Optional(prev_regexp),
                    '+' => Repeated(prev_regexp),
                    '*' => OptionalRepeated(prev_regexp),
                    _ => unreachable!()
                });
            }
            _ => stack.push(Char(c))
        }
    }

    if depth > 0 {
        return Result::Err(UnmatchedParenthesis(last_open_paren_index));
    }

    if stack.len() == 0 {
        return Result::Err(EmptyRegexp)
    }

    if num_alternatives > 0 {
        let mut alternative_stack = Vec::new();
        while stack.len() > num_alternatives {
            alternative_stack.push(stack.pop().unwrap());
        }
        alternative_stack.reverse();
        match alternative_stack.len() {
            0 => return Result::Err(EmptyAlternative(string.len()-1)),
            1 => stack.push(alternative_stack.pop().unwrap()),
            _ => stack.push(Concatenation(alternative_stack))
        }
        num_alternatives += 1;
    }

    match num_alternatives {
        0 => match stack.len() {
            1 => return Result::Ok(stack.pop().unwrap()),
            _ => return Result::Ok(Concatenation(stack))
        },
        1 => Result::Err(EmptyAlternative(string.len() - 1)),
        _ => Result::Ok(Alternation(stack))
    }
}

// pub fn regexp_from_string(string: &str) -> Result<Regexp, RegexpError> {
//     #[derive(Debug)]
//     struct Context {
//         stack: Vec<Regexp>,
//         escaped: bool,
//         num_alternatives: usize
//     }

//     let mut contexts: Vec<Context> = vec![Context { stack: Vec::new(),
//                                                     escaped: false,
//                                                     num_alternatives: 0}];

//     for (i, c) in string.chars().enumerate() {
//         if contexts.last().unwrap().escaped {
//             match c {
//                 '(' | ')' | '?' | '+' | '*'
//                     => contexts.last_mut().unwrap().stack.push(Regexp::Char('\\')),
//                     _ => ()
//             }
//             contexts.last_mut().unwrap().stack.push(Regexp::Char(c));
//             contexts.last_mut().unwrap().escaped = false;
//             continue;
//         }

//         match c {
//             '(' => {
//                 contexts.push(Context { stack: Vec::new(),
//                                         escaped: false,
//                                         num_alternatives: 0});
//             },
//             ')' => {
//                 let mut cur_context = contexts.pop().unwrap();
//                 let mut prev_context = match contexts.pop() {
//                     Some(value) => value,
//                     None => return Result::Err(
//                         RegexpError::UnmatchedParenthesis { at_index: i })
//                 };
//                 if cur_context.stack.len() == 0 {
//                     return Result::Err(
//                         RegexpError::EmptyGroup {at_index: i - 1});
//                 } else if cur_context.stack.len() == 1 {
//                     prev_context.stack
//                         .append(&mut (cur_context.stack));
//                 } else {
//                     if cur_context.num_alternatives > 0 {
//                         prev_context.stack.push(
//                             Regexp::Alternation(cur_context.stack));
//                     } else {
//                         prev_context.stack.push(
//                             Regexp::Concatenation(cur_context.stack));
//                     }
//                 }
//                 contexts.push(prev_context);
//             }
//             '|' => {
//                 let cur_context = contexts.last_mut().unwrap();
//                 if cur_context.stack.len() == cur_context.num_alternatives {
//                     return Result::Err(
//                         RegexpError::EmptyAlternative { at_index: i});
//                 }
//                 let mut alternative_regexps = Vec::new();
//                 while cur_context.stack.len()
//                     > cur_context.num_alternatives {
//                         alternative_regexps.push(cur_context.stack.pop().unwrap());
//                 }
//                 alternative_regexps.reverse();
//                 cur_context.stack.push(Regexp::Concatenation(alternative_regexps));
//                 cur_context.num_alternatives += 1;
//             },
//             '?' | '+' | '*' => {
//                 let prev_regexp = match contexts.last_mut().unwrap().stack.pop() {
//                     Some(value) => value,
//                     None => return Result::Err(
//                         RegexpError::MisplacedOperator {at_index: i})
//                 };
//                 let new_regexp = match c {
//                     '?' => Regexp::Optional(Box::new(prev_regexp)),
//                     '+' => Regexp::Repeated(Box::new(prev_regexp)),
//                     '*' => Regexp::OptionalRepeated(Box::new(prev_regexp)),
//                      _  => unreachable!()
//                 };
//                 contexts.last_mut().unwrap().stack.push(new_regexp);
//             }
//             _ => {
//                 contexts.last_mut().unwrap().stack.push(Regexp::Char(c))
//             }
//         }
//     }

//     for context in contexts.iter() {
//         println!("{:?}", context);
//     }

//     if contexts.len() != 1 {
//         panic!("Context stack missized");
//     } 

//     let mut last_context = contexts.pop().unwrap();
//     println!("{:?}", last_context);

//     if last_context.num_alternatives != 0 {
//         let mut alternative_regexps = Vec::new();
//         if last_context.stack.len() == last_context.num_alternatives {
//             return Result::Err(RegexpError::EmptyAlternative {
//                 at_index: string.len()-1});
//         }
//         while last_context.stack.len() > last_context.num_alternatives {
//             alternative_regexps.push(last_context.stack.pop().unwrap());
//         }
//         alternative_regexps.reverse();
//         last_context.stack.push(Regexp::Concatenation(alternative_regexps));
//         last_context.num_alternatives += 1;
//     }

//     match last_context.num_alternatives {
//         0 => match last_context.stack.len() {
//             0 => Result::Err(RegexpError::EmptyRegexp),
//             1 => Result::Ok(last_context.stack.pop().unwrap()),
//             _ => Result::Ok(Regexp::Concatenation(last_context.stack))
//         },
//         1 => Result::Err(RegexpError::EmptyAlternative {at_index: string.len()-1}),
//         _ => match last_context.stack.len() {
//             0 => unreachable!(), // last_context.num_alternatives > 0
//             1 => Result::Ok(last_context.stack.pop().unwrap()),
//             _ => {
//                 Result::Ok(Regexp::Alternation(last_context.stack))   
//             }
//         }
//     }
//     // match last_context.stack.len() {
//     //     0 => Result::Err(RegexpError::EmptyRegexp),
//     //     1 => Result::Ok(last_context.stack.pop().unwrap()),
//     //     _ => match last_context.num_alternatives {
//     //         0 => Result::Ok(Regexp::Concatenation(last_context.stack)),
//     //         _ => {
//     //             Result::Ok(Regexp::Alternation(last_context.stack))
//     //         }
//     //     }
//     // }
// }

// pub fn regexp_from_string(string: &str) -> Result<Regexp, RegexpError>{
//     // println!("regexp_from_string called with: {}", string);
//     let mut regexp_stack: Vec<Regexp> = Vec::new();
//     let mut escaped = false;

//     let mut parenthesized_string = String::new();
//     let mut parenthesized_depth = 0;
//     let mut is_alternation = false;
//     let mut num_alternatives_pushed = 0;

//     for (i, c) in string.chars().enumerate() {
//         if escaped {
//             // match c {
//             //     '
//             // }
//             if parenthesized_depth > 0 {
//                 parenthesized_string.push(c);
//             } else {
//                 regexp_stack.push(Regexp::Char(c));
//             }
//             escaped = false;
//             continue;
//         }

//         match c {
//             '(' => {
//                 if parenthesized_depth > 0 {
//                     parenthesized_string.push(c);
//                 }
//                 parenthesized_depth += 1;
//             }
//             ')' => {
//                 parenthesized_depth -= 1;
//                 if parenthesized_depth > 0 {
//                     parenthesized_string.push(c);
//                 }
//                 if parenthesized_depth == 0 {
//                     println!("Parenthesized string: {}", parenthesized_string);
//                     let regexp =
//                         try!(regexp_from_string(parenthesized_string.as_str()));
//                     regexp_stack.push(regexp);
//                     parenthesized_string = String::new();
//                 } else if parenthesized_depth < 0 {
//                     return Result::Err(RegexpError::
//                                        UnmatchedParenthesis { at_index: i });
//                 }
//             }
//             '\\' => if !escaped { escaped = true },
//             '|' => {
//                 if parenthesized_depth > 0 {
//                     parenthesized_string.push(c);
//                     continue;
//                 }
//                 is_alternation = true;

//                 if regexp_stack.len() == num_alternatives_pushed {
//                     return Result::Err(RegexpError::EmptyAlternative {at_index: i });
//                 }

//                 let mut alternatives_stack: Vec<Regexp> = Vec::new();
//                 while regexp_stack.len() > num_alternatives_pushed {
//                     if let Some(popped_regexp) = regexp_stack.pop() {
//                         alternatives_stack.push(popped_regexp);
//                     } else { panic!(); }
//                 }
//                 alternatives_stack.reverse();

//                 let alternative: Regexp;
//                 if alternatives_stack.len() == 1 {
//                     alternative = alternatives_stack.pop().unwrap();
//                 } else {
//                     alternative = Regexp::Concatenation(alternatives_stack);
//                 }
//                 regexp_stack.push(alternative);

//                 num_alternatives_pushed += 1;
//             }
//             '?' => {
//                 if parenthesized_depth > 0 {
//                     parenthesized_string.push(c);
//                     continue
//                 }
//                 if is_alternation && num_alternatives_pushed == regexp_stack.len() {
//                     return Result::Err(RegexpError::MisplacedOperator { at_index: i });
//                 }
                
//                 if let Some(popped_regexp) = regexp_stack.pop() {
//                     regexp_stack.push(Regexp::Optional(Box::new(popped_regexp)));
//                 } else { panic!(); }
//             } 
//             '+' => {
//                 if parenthesized_depth > 0 {
//                     parenthesized_string.push(c);
//                     continue
//                 }
//                 if is_alternation && num_alternatives_pushed == regexp_stack.len() {
//                     return Result::Err(RegexpError::MisplacedOperator { at_index: i });
//                 }

//                 if let Some(popped_regexp) = regexp_stack.pop() {
//                     regexp_stack.push(Regexp::Repeated(Box::new(popped_regexp)));
//                 } else { panic!(); }
//             }
//             '*' => {
//                 if parenthesized_depth > 0 {
//                     parenthesized_string.push(c);
//                     continue
//                 }
//                 if is_alternation && num_alternatives_pushed == regexp_stack.len() {
//                     return Result::Err(RegexpError::MisplacedOperator { at_index: i });
//                 }

//                 if let Some(popped_regexp) = regexp_stack.pop() {
//                     regexp_stack.push(
//                         Regexp::OptionalRepeated(Box::new(popped_regexp)));
//                 } else { panic!(); }
//             }
//             _ => if parenthesized_depth > 0 {
//                 parenthesized_string.push(c);
//             } else {
//                 regexp_stack.push(Regexp::Char(c));
//             }
//         }
//     }

//     if parenthesized_depth > 0 {
//         panic!("Unmatched opening parenthesis in regexp string passed to \
//                 regexp_from_string");
//     }

//     if is_alternation {
//         if regexp_stack.len() == num_alternatives_pushed {
//             panic!("Missing alternative branch at end of \
//                     regexp string passed to regexp_from_string");
//         }

//         let mut alternatives_stack: Vec<Regexp> = Vec::new();
//         while regexp_stack.len() > num_alternatives_pushed {
//             if let Some(popped_regexp) = regexp_stack.pop() {
//                 alternatives_stack.push(popped_regexp);
//             } else { panic!(); }
//         }
//         alternatives_stack.reverse();

//         let alternative: Regexp;
//         if alternatives_stack.len() == 1 {
//             alternative = alternatives_stack.pop().unwrap();
//         } else {
//             alternative = Regexp::Concatenation(alternatives_stack);
//         }
//         regexp_stack.push(alternative);

//         Result::Ok(Regexp::Alternation(regexp_stack))
//     } else {
//         if regexp_stack.len() == 1 {
//             Result::Ok(regexp_stack.pop().unwrap())
//         } else {
//             Result::Ok(Regexp::Concatenation(regexp_stack))
//         }
//     }
// }

pub fn regexp_to_string(regexp: &Regexp) -> String {
    use self::Regexp::*;
    match *regexp {
        Char(c) => match c {
            '?' | '+' | '*' | '\\' | '(' | ')' =>
                '\\'.to_string() + &(c.to_string()),
            _ => c.to_string()
        },
        Concatenation(ref regexps) => regexps.iter().map(regexp_to_string)
                .collect::<Vec<String>>().join(""),
        Alternation(ref regexps)
            => format!("({})", regexps.iter().map(regexp_to_string)
                       .collect::<Vec<String>>().join("|")),
        Optional(ref inner_regexp)
            | Repeated(ref inner_regexp)
            | OptionalRepeated(ref inner_regexp) => {
                let op_char = match *regexp { Optional(_) => "?",
                                              Repeated(_) => "+",
                                              OptionalRepeated(_) => "*",
                                              _ => unreachable!() };
                let text = regexp_to_string(&inner_regexp);
                match **inner_regexp {
                    Char(_) | Optional(_) | Repeated(_) | OptionalRepeated(_)
                        => format!("{}{}", text, op_char),
                    _ => format!("({}){}", text, op_char)
                }
        }
    }
}

pub fn print_regexp(regexp: &Regexp) {
    print_regexp_depth(regexp, 0);
}

fn print_regexp_depth(regexp: &Regexp, depth: i64) {
    for _ in 0..depth {
        print!("\t");
    }

    let text: String = format!("{:?}", *regexp);
    let text = text[0..text.find('(').unwrap()].to_string();

    println!("{}: {}", text, regexp_to_string(&regexp));

    use self::Regexp::*;
    match *regexp {
        Char(_) => (),
        Concatenation(ref inner_regexps) | Alternation(ref inner_regexps) => {
            for sub_regexp in inner_regexps {
                print_regexp_depth(&sub_regexp, depth + 1);
            }
        },
        Optional(ref inner_regexp)
            | Repeated(ref inner_regexp)
            | OptionalRepeated(ref inner_regexp) => {
                print_regexp_depth(&inner_regexp, depth + 1);
            }
    }
}

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
