mod helpers;
use helpers::left_get;
use helpers::right_get;
use helpers::number_get;
use helpers::tree_get;
use helpers::tree_set;
use std::fs;
use std::rc::Rc;

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

fn eval(mut subject: Noun, mut formula: Noun) -> Noun {
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

fn deserialize(bytes: &[u8]) -> Noun {
    let (noun, _) = deserialize_inner(bytes);
    noun
}

fn deserialize_inner(bytes: &[u8]) -> (Noun, usize) {
    match bytes {
        [0x00, rest @ ..] => {
            let number = u64::from_le_bytes(rest[..8].try_into().unwrap());
            (Noun::Atom(number), 9)
        }
        [0x01, rest @ ..] => {
            let (left, left_bytes_used) = deserialize_inner(rest);
            let (right, right_bytes_used) = deserialize_inner(&rest[left_bytes_used..]);
            (Noun::Cell(Rc::new(left), Rc::new(right)), left_bytes_used + right_bytes_used + 1)

        }
        _ => panic!("empty"),

    }

}

fn main() -> std::io::Result<()> {
  // opcode 1 test
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(7)))));

//  // opcode 0 test
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(20)), Rc::new(Noun::Atom(30))))), Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))));

  // opcode 3 tests: is-cell?
  // eval_step([10 20], [3 [0 1]]) → 0 (result is [10 20], a cell)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(3)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(1)))))));
  // eval_step([10 20], [3 [0 2]]) → 1 (result is 10, an atom)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(3)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))))));

  // opcode 4 tests: increment
  // eval_step([10 20], [4 [0 2]]) → 11 (eval_step [0 2] → 10, then 10+1)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(4)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))))));
  // eval_step([10 20], [4 [0 3]]) → 21 (eval_step [0 3] → 20, then 20+1)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(4)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))))));

  // opcode 5 tests: equals
  // eval([10 10], [5 [0 2] [0 3]]) → 0 (10 == 10)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(10))), Noun::Cell(Rc::new(Noun::Atom(5)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))))))));
  // eval([10 20], [5 [0 2] [0 3]]) → 1 (10 != 20)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(5)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))))))));

  // opcode 6 tests: if-then-else
  // eval([10 20], [6 [1 0] [0 2] [0 3]]) → 10 (test=0 → true → eval [0 2] → 10)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(6)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(0)))), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))))))))));
  // eval([10 20], [6 [1 1] [0 2] [0 3]]) → 20 (test=1 → false → eval [0 3] → 20)
  println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Atom(20))), Noun::Cell(Rc::new(Noun::Atom(6)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(1)))), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))))))))));

  // opcode 7 tests: compose
  // eval(42, [7 [1 10] [4 [0 1]]]) → 11 (eval [1 10] → 10, then eval(10, [4 [0 1]]) → 11)
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Rc::new(Noun::Atom(7)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(10)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(4)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(1)))))))))));

  // deep tree_get test
  // eval([10 [20 30]], [0 6]) → should be 20
  println!("deep: {:?}", eval(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(20)), Rc::new(Noun::Atom(30))))), Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(6)))));

  // opcode 8 tests: push
  // eval(42, [8 [1 10] [0 2]]) → 10 (push 10 onto subject → [10 42], then [0 2] → 10)
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Rc::new(Noun::Atom(8)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(10)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(2)))))))));
  // eval(42, [8 [1 10] [0 3]]) → 42 (push 10 onto subject → [10 42], then [0 3] → 42)
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Rc::new(Noun::Atom(8)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(1)), Rc::new(Noun::Atom(10)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(3)))))))));

  // Infinite loop test — this WILL crash with stack overflow
  // Program: [2 [0 1] [0 1]] — evaluates itself against itself forever
    // println!("{:?}", eval(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(2)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(1)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(1)))))))), Rc::new(Noun::Atom(0))), Noun::Cell(Rc::new(Noun::Atom(2)), Rc::new(Noun::Cell(Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(1)))), Rc::new(Noun::Cell(Rc::new(Noun::Atom(0)), Rc::new(Noun::Atom(1)))))))));
  // serialize(Noun::Atom(1));

  // serialize(Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(20)), Rc::new(Noun::Atom(30))))));

  // deserialize(&serialize(Noun::Atom(10)));

  let noun = Noun::Cell(Rc::new(Noun::Atom(10)), Rc::new(Noun::Cell(Rc::new(Noun::Atom(20)), Rc::new(Noun::Atom(30)))));

    println!("{:?}", assert_eq!(deserialize(&serialize(noun.clone())), noun));

    // save to disk a Noun and then read it

    match fs::write("test.noun", serialize(noun.clone()))? {
        () => {
            let data: Vec<u8> = fs::read("test.noun")?;
            println!("{:?}", deserialize(&data));
            Ok(())
        }
        _ => panic!("File not written")
    }
}
