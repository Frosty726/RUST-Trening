use std::cmp::Ordering;
use std::cmp::Ord;

// TODO: Сделайте функцию min которая вызывается в main.
fn min<T : Ord>(first : T, second : T) -> T {
    match first.cmp(&second) {
        Ordering::Less    => first,
        Ordering::Equal   => first,
        Ordering::Greater => second,
    }
}

pub fn demonstrate() {
    println!("--- Task 8 ---");
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
    print!("Все тесты пройдены!");
    println!();
}