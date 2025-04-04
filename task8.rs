//use std::cmp::Ordering;

// TODO: Сделайте функцию min которая вызывается в main.

fn min<T: std::cmp::Ord>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
