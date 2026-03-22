use std::rc::Rc;
use super::Noun;
use super::eval;


pub fn left_get(cell: Noun) -> Noun {
    match cell {
        Noun::Cell(x, _y) => (*x).clone(),
        _ => panic!("Unkown"),
    }
}

pub fn right_get(cell: Noun) -> Noun {
    match cell {
        Noun::Cell(_x, y) => (*y).clone(),
        _ => panic!("Unkown"),
    }
}

pub fn number_get(cell: Noun) -> u64 {
    match cell {
        Noun::Atom(x) => x,
        _ => panic!("Unkown"),
    }
}

pub fn tree_get(noun: Noun, address: u64) -> Noun {
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

pub fn tree_set(noun: Noun, axis: u64, replacement: Noun) -> Noun {
    match axis {
        1 => replacement,
        _val if axis%2 == 0 => {
            let sibling = tree_get(noun.clone(), axis + 1);
            tree_set(noun.clone(), axis/2, Noun::Cell(Rc::new(replacement), Rc::new(sibling)))
        }
        _ => {
            let sibling = tree_get(noun.clone(), axis - 1);
            tree_set(noun.clone(), axis/2, Noun::Cell(Rc::new(sibling), Rc::new(replacement)))
        }
    }
}

pub fn test_op_codes() {

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
}
