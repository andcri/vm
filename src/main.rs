mod helpers;
use helpers::left_get;
use helpers::right_get;
use helpers::number_get;
use helpers::tree_get;
use helpers::tree_set;
use helpers::test_op_codes;
use std::fs;
use std::rc::Rc;
use std::io::{stdin,stdout,Write};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Noun {
    Atom(u64),
    Cell(Rc<Noun>, Rc<Noun>),
}

// Trampoling
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Outcome {
    Done(Noun),
    Continue(Noun, Noun)
}



fn eval_step(subject: Noun, formula: Noun) -> Outcome {
    match formula {
        Noun::Cell(x, y) => match *x {
            Noun::Atom(0) => match (*y).clone() {
                Noun::Atom(x) =>  Outcome::Done(tree_get(subject, x)),
                _ => panic!("Unkown"),
            }
            Noun::Atom(1) => Outcome::Done((*y).clone()),
            Noun::Atom(2) => match (*y).clone() {
                Noun::Cell(x, y) => {
                    let new_subject = eval(subject.clone(), (*x).clone());
                    let new_formula = eval(subject.clone(), (*y).clone());
                    Outcome::Continue(new_subject, new_formula)
                },
                _ => panic!("Unkown"),
            },
            Noun::Atom(3) => match eval(subject, (*y).clone()) {
                Noun::Cell(_x, _y) => Outcome::Done(Noun::Atom(0)),
                Noun::Atom(_x) => Outcome::Done(Noun::Atom(1)),
            },
            Noun::Atom(4) => match eval(subject, (*y).clone()) {
                Noun::Atom(x) => Outcome::Done(Noun::Atom(x+1)),
                _ => panic!("Unkown"),
            },
            Noun::Atom(5) => match (*y).clone() {
                Noun::Cell(x, y) => if eval(subject.clone(), (*x).clone()) == eval(subject.clone(), (*y).clone()) { Outcome::Done(Noun::Atom(0)) } else { Outcome::Done(Noun::Atom(1)) }
                _ => panic!("Unkown"),
            }
            Noun::Atom(6) => match (*y).clone() {
                Noun::Cell(x, y) => match eval(subject.clone(), (*x).clone()) == Noun::Atom(0) {
                    true => Outcome::Continue(subject.clone(), left_get((*y).clone())),
                    false => Outcome::Continue(subject.clone(), right_get((*y).clone())),
                }
                _ => panic!("Unkown"),
            }
            Noun::Atom(7) => match (*y).clone() {
                Noun::Cell(a, b) => Outcome::Continue(eval(subject.clone(), (*a).clone()), (*b).clone()),
                _ => panic!("Unkown"),
            }
            Noun::Atom(8) => match (*y).clone() {
                Noun::Cell(a, b) => Outcome::Continue(Noun::Cell(Rc::new(eval(subject.clone(), (*a).clone())), Rc::new(subject.clone())), (*b).clone()),
                _ => panic!("Unkown"),
            }
            Noun::Atom(9) => match (*y).clone() {
                Noun::Cell(a, b) => {
                    let core = eval(subject.clone(), (*b).clone());
                    Outcome::Continue(core.clone(), tree_get(core.clone(), number_get((*b).clone())))
                }
                _ => panic!("Unkown"),
            }
            Noun::Atom(10) => match (*y).clone() {
                Noun::Cell(a, b) => {
                    let noun = eval(subject.clone(), (*b).clone());
                    let replacement = eval(subject.clone(), right_get((*a).clone()));
                    Outcome::Done(tree_set(noun, number_get(left_get((*a).clone())) , replacement))
                }
                _ => panic!("Unkown"),
            }
            Noun::Atom(11) => match (*y).clone() {
                Noun::Cell(a, b) => {
                    Outcome::Continue(subject, (*b).clone())
                }
                Noun::Atom(a) => {
                    Outcome::Continue(subject, Noun::Atom(a))
                }
            }
            _ => panic!("Unkown"),
        },
        _ => panic!("Unkown"),
    }
}

pub fn eval(mut subject: Noun, mut formula: Noun) -> Noun {
    loop {
        match eval_step(subject, formula) {
            Outcome::Done(x) => return x,
            Outcome::Continue(x, y) => {
                subject = x;
                formula = y;
            }
        }
    }
}


fn serialize(noun: Noun) -> Vec<u8> {
    match noun {
        Noun::Atom(x) => {
            let mut bytes = Vec::new();
            bytes.push(0x00);
            bytes.extend_from_slice(&x.to_le_bytes());
            bytes

        }
        Noun::Cell(x, y) => {
            let mut bytes = Vec::new();
            bytes.push(0x01);
            bytes.extend(serialize((*x).clone()));
            bytes.extend(serialize((*y).clone()));
            bytes
        }
    }
}

fn deserialize(bytes: &[u8], arena: &mut Arena) -> Noun {
    let (noun, _) = deserialize_inner(bytes, arena);
    noun
}

// A Vec<u8> (your memory block)
// An offset tracking where the next free slot is
// A function that takes a Noun, writes it into the block at the current offset, advances the offset, and returns a reference
struct Arena {
    memory: Vec<u8>,
    offset: usize,
}

impl Arena {
    fn alloc(&mut self, noun: Noun) -> () {
        //arena.memory.push(&noun as *const Noun as *const u8);
        println!("arena: {:?}", &noun as *const Noun as *const u8);
        // add the bytes to the vector starting at the offset
        // update the offset
        // return the Rc that points to that vector index range
    }
}

fn deserialize_inner(bytes: &[u8], arena: &mut Arena) -> (Noun, usize) {
    match bytes {
        [0x00, rest @ ..] => {
            let number = u64::from_le_bytes(rest[..8].try_into().unwrap());
            (Noun::Atom(number), 9)
        }
        [0x01, rest @ ..] => {
            let (left, left_bytes_used) = deserialize_inner(rest, arena);
            let (right, right_bytes_used) = deserialize_inner(&rest[left_bytes_used..], arena);
            arena.alloc(left.clone());
            (Noun::Cell(Rc::new(left), Rc::new(right)), left_bytes_used + right_bytes_used + 1)

        }
        _ => panic!("empty"),

    }

}

fn parser(line: &[u8]) -> (Noun, usize) {
    //read the line, match on start of cell [ and end of cell ]
    match line {
        [head @ b'0'..=b'9', rest @ ..] => {
            let mut atom_value = std::slice::from_ref(head);
            let mut combined = Vec::new();
            let mut index = 0;
            combined.extend_from_slice(atom_value);
            for (i, digit) in rest.iter().enumerate() {
                if digit.is_ascii_digit() {
                    combined.extend_from_slice(std::slice::from_ref(digit));
                } else {
                    index = i;
                    break;
                }
            }
            let result = std::str::from_utf8(&combined).unwrap().parse::<u64>().unwrap();
            (Noun::Atom(result), index+1)
        }
        [b'[', rest @ ..] => {
            let (left, pos) = parser(rest);
            let (next, next_pos) = parser(&rest[pos+1..]);
            (Noun::Cell(Rc::new(left), Rc::new(next)), pos + next_pos + 1)
        }
        [b']', rest @ ..] => {
            parser(rest)
        }

        [b' ', rest @ ..] => {
            parser(rest)
        }
        _ => panic!("Invalid command")
    }

}

fn pretty_print(noun: Noun) -> String {
    match noun {
        Noun::Atom(x) => {
            x.to_string()
        }
        Noun::Cell(x, y) => {
            "[".to_owned() + &pretty_print((*x).clone()) + " " + &pretty_print((*y).clone()) + "]"
        }
    }
}

fn main() -> std::io::Result<()> {
   //test_op_codes();

  let mut arena = Arena {
      memory: Vec::new(),
      offset: 0
  };
  let noun = Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(20)), Rc::new(Noun::Atom(30)))));

  println!("{:?}", assert_eq!(deserialize(&serialize(noun.clone()), &mut arena), noun));

  match fs::write("test.noun", serialize(noun.clone()))? {
      () => {
          let data: Vec<u8> = fs::read("test.noun")?;
          println!("{:?}", deserialize(&data, &mut arena));
          Ok(())
      }
      _ => panic!("File not written")
  }
}

//fn main() -> std::io::Result<()> {
//    loop {
//        print!("nock>");
//        let mut s=String::new();
//        let _=stdout().flush();
//        stdin().read_line(&mut s).expect("Did not enter a correct string");
//        let (noun, _) = parser(&s.as_bytes());
//        println!("{}", pretty_print(noun.clone()));
//        let left = left_get(noun.clone());
//        let right = right_get(noun.clone());
//        println!("{}", pretty_print(eval(left, right)));
//    }
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_atom() {
        let (noun, _) = parser(b"42 ");
        assert_eq!(noun, Noun::Atom(42));
    }

    #[test]
    fn parse_multi_digit_atom() {
        let (noun, _) = parser(b"123 ");
        assert_eq!(noun, Noun::Atom(123));
    }

    #[test]
    fn parse_simple_cell() {
        let (noun, _) = parser(b"[1 2]");
        assert_eq!(noun, Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(2))));
    }

    #[test]
    fn parse_nested_cell_right() {
        let (noun, _) = parser(b"[1 [2 3]]");
        assert_eq!(noun, Noun::Cell(
            Rc::new(Noun::Atom(1)),
            Rc::new(Noun::Cell(Rc::new(Noun::Atom(2)), Rc::new(Noun::Atom(3))))
        ));
    }

    #[test]
    fn parse_nested_cell_left() {
        let (noun, _) = parser(b"[[1 2] 3]");
        assert_eq!(noun, Noun::Cell(
            Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(2)))),
            Rc::new(Noun::Atom(3))
        ));
    }

    #[test]
    fn parse_deeply_nested() {
        let (noun, _) = parser(b"[[1 2] [3 4]]");
        assert_eq!(noun, Noun::Cell(
            Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(2)))),
            Rc::new(Noun::Cell(Rc::new(Noun::Atom(3)), Rc::new(Noun::Atom(4))))
        ));
    }

    #[test]
    fn parse_multi_digit_in_cell() {
        let (noun, _) = parser(b"[10 20]");
        assert_eq!(noun, Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))));
    }
}
