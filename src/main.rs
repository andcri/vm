#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Noun {
    Atom(u64),
    Cell(Box<crate::Noun>, Box<crate::Noun>),
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
            Noun::Atom(0) => match *y {
                Noun::Atom(x) =>  Outcome::Done(tree_get(subject, x)),
                _ => panic!("Unkown"),
            }
            Noun::Atom(1) => Outcome::Done(*y),
            Noun::Atom(2) => match *y {
                Noun::Cell(x, y) => Outcome::Done((eval(eval(subject.clone(), *x), eval(subject.clone(), *y)))),
                _ => panic!("Unkown"),
            },
            Noun::Atom(3) => match eval(subject, *y) {
                Noun::Cell(_x, _y) => Outcome::Done(Noun::Atom(0)),
                Noun::Atom(_x) => Outcome::Done(Noun::Atom(1)),
            },
            Noun::Atom(4) => match eval(subject, *y) {
                Noun::Atom(x) => Outcome::Done(Noun::Atom(x+1)),
                _ => panic!("Unkown"),
            },
            Noun::Atom(5) => match *y {
                Noun::Cell(x, y) => if eval(subject.clone(), *x) == eval(subject.clone(), *y) { Outcome::Done(Noun::Atom(0)) } else { Outcome::Done(Noun::Atom(1)) }
                _ => panic!("Unkown"),
            }
            Noun::Atom(6) => match *y {
                Noun::Cell(x, y) => match eval(subject.clone(), *x) == Noun::Atom(0) {
                    true => Outcome::Done(eval(subject.clone(), left_get(*y))),
                    false => Outcome::Done(eval(subject.clone(), right_get(*y))),
                }
                _ => panic!("Unkown"),
            }
            Noun::Atom(7) => match *y {
                Noun::Cell(a, b) => Outcome::Continue(eval(subject.clone(), *a), *b),
                _ => panic!("Unkown"),
            }
            Noun::Atom(8) => match *y {
                Noun::Cell(a, b) => Outcome::Continue(Noun::Cell(Box::new(eval(subject.clone(), *a)), Box::new(subject.clone())), *b),
                _ => panic!("Unkown"),
            }
            Noun::Atom(9) => match *y {
                Noun::Cell(a, b) => {
                    let core = eval(subject.clone(), *b);
                    Outcome::Continue(core.clone(), tree_get(core.clone(), number_get(*a)))
                }
                _ => panic!("Unkown"),
            }
            Noun::Atom(10) => match *y {
                Noun::Cell(a, b) => {
                    let noun = eval(subject.clone(), *b);
                    let replacement = eval(subject.clone(), right_get(*a.clone()));
                    Outcome::Done(tree_set(noun, number_get(left_get(*a.clone())) , replacement))
                }
                _ => panic!("Unkown"),
            }
            Noun::Atom(11) => match *y {
                Noun::Cell(a, b) => {
                    Outcome::Continue(subject, *b)
                }
                Noun::Atom(a) => {
                    Outcome::Continue(subject, Noun::Atom(a))
                }
            }
            _ => panic!("Missing"),
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
                continue;
            }
        }
    }
}

fn left_get(cell: Noun) -> Noun {
    match cell {
        Noun::Cell(x, _y) => *x,
        _ => panic!("Unkown"),
    }
}

fn right_get(cell: Noun) -> Noun {
    match cell {
        Noun::Cell(_x, y) => *y,
        _ => panic!("Unkown"),
    }
}

fn number_get(cell: Noun) -> u64 {
    match cell {
        Noun::Atom(x) => x,
        _ => panic!("Unkown"),
    }
}

fn tree_get(noun: Noun, address: u64) -> Noun {
    match address {
        1 => noun,
        _val if address%2 == 0 => {
            left_get(tree_get(noun, address / 2))
          }
        _ => {
            right_get(tree_get(noun, address / 2))
          }
    }
}

fn tree_set(noun: Noun, axis: u64, replacement: Noun) -> Noun {
    match axis {
        1 => replacement,
        _val if axis%2 == 0 => {
            let sibling = tree_get(noun.clone(), axis + 1);
            tree_set(noun.clone(), axis/2, Noun::Cell(Box::new(replacement), Box::new(sibling)))
        }
        _ => {
            let sibling = tree_get(noun.clone(), axis - 1);
            tree_set(noun.clone(), axis/2, Noun::Cell(Box::new(sibling), Box::new(replacement)))
        }
    }
}

fn main() {
  // opcode 1 test
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(7)))));

//  // opcode 0 test
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Cell(Box::new(Noun::Atom(20)), Box::new(Noun::Atom(30))))), Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))));

  // opcode 3 tests: is-cell?
  // eval_step([10 20], [3 [0 1]]) → 0 (result is [10 20], a cell)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(3)), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)))))));
  // eval_step([10 20], [3 [0 2]]) → 1 (result is 10, an atom)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(3)), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))))));

  // opcode 4 tests: increment
  // eval_step([10 20], [4 [0 2]]) → 11 (eval_step [0 2] → 10, then 10+1)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(4)), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))))));
  // eval_step([10 20], [4 [0 3]]) → 21 (eval_step [0 3] → 20, then 20+1)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(4)), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))))));

  // opcode 5 tests: equals
  // eval([10 10], [5 [0 2] [0 3]]) → 0 (10 == 10)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(10))), Noun::Cell(Box::new(Noun::Atom(5)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))))))));
  // eval([10 20], [5 [0 2] [0 3]]) → 1 (10 != 20)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(5)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))))))));

  // opcode 6 tests: if-then-else
  // eval([10 20], [6 [1 0] [0 2] [0 3]]) → 10 (test=0 → true → eval [0 2] → 10)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(6)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(0)))), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))))))))));
  // eval([10 20], [6 [1 1] [0 2] [0 3]]) → 20 (test=1 → false → eval [0 3] → 20)
  println!("{:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Atom(20))), Noun::Cell(Box::new(Noun::Atom(6)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(1)))), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))))))))));

  // opcode 7 tests: compose
  // eval(42, [7 [1 10] [4 [0 1]]]) → 11 (eval [1 10] → 10, then eval(10, [4 [0 1]]) → 11)
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Box::new(Noun::Atom(7)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(10)))), Box::new(Noun::Cell(Box::new(Noun::Atom(4)), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)))))))))));

  // deep tree_get test
  // eval([10 [20 30]], [0 6]) → should be 20
  println!("deep: {:?}", eval(Noun::Cell(Box::new(Noun::Atom(10)), Box::new(Noun::Cell(Box::new(Noun::Atom(20)), Box::new(Noun::Atom(30))))), Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(6)))));

  // opcode 8 tests: push
  // eval(42, [8 [1 10] [0 2]]) → 10 (push 10 onto subject → [10 42], then [0 2] → 10)
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Box::new(Noun::Atom(8)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(10)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)))))))));
  // eval(42, [8 [1 10] [0 3]]) → 42 (push 10 onto subject → [10 42], then [0 3] → 42)
  println!("{:?}", eval(Noun::Atom(42), Noun::Cell(Box::new(Noun::Atom(8)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(10)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)))))))));

  // Infinite loop test — this WILL crash with stack overflow
  // Program: [2 [0 1] [0 1]] — evaluates itself against itself forever
  // println!("{:?}", eval(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)))))))), Box::new(Noun::Atom(0))), Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Cell(Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)))), Box::new(Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)))))))));
}
