use std::collections::HashMap;

/// Counter определяет количество значений типа T в коллекции.
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl <T : std::hash::Hash + std::cmp::Eq> Counter<T> {
    /// Создаем новый счетчик Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Считает количество появлений заданного значения.
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Возвращает количество появлений заданного значения.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

pub fn demonstrate() {
    println!("--- Task 9 ---");
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("Значение {} видели {} раз", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("Получили {} яблок", strctr.times_seen("apple"));
    println!();
}