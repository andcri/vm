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
