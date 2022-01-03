#![allow(unused)]

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == 0 && b.len() == 0 {
        return Comparison::Equal;
    }

    if a.len() == b.len() {
        if a[0] != b[0] {
            return Comparison::Unequal;
        }

        return sublist(&a[1..], &b[1..])
    }

    if a.len() < b.len() {
        return is_sublist(a, b)
    }

    is_superlist(a, b)
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == 0 {
        return Comparison::Sublist;
    }

    if a.len() > b.len() {
        return Comparison::Unequal;
    }

    if a[0] != b[0] {
        return is_sublist(a, &b[1..]);
    }

    is_sublist(&a[1..], b)
}

fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match is_sublist(b, a) {
        Comparison::Sublist => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
