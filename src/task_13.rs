
// РАБОЧИЙ, ПОМНИ! ВСЕ ЗНАЧЕНИЯ В ДЕРЕВЕ ДОЛЖНЫ БЫТЬ УНИКАЛЬНЫМИ!!!!!!!

/// Узел двоичного дерева.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// Возможно пустое поддерево.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// Контейнер сохраняющий множество значений, с помощью двоичного дерева.
///
/// Если одно значение добавляется несколько раз, сохраняется только одно.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Реализуйте методы new, insert, len, и has для `Subtree`.
impl <T : Ord> Subtree<T> {
    fn new() -> Self {
        Subtree(None)
    }

    // Добавление элемента в дерево. Рекурсивное
    fn insert(&mut self, value : T) {
        // Если элемент отсутствует (т.е. поддерево является листом)
        // то происходит присваивание значения
        if self.0.is_none() {
            self.0 = Some(Box::new(
                Node {
                    value: value,
                    left: Subtree(None),
                    right: Subtree(None),
                }
            ));
        // Иначе ищем ниже, куда присвоить значение
        } else {
            // Проверка на наличие значения. Если оно есть - выход.
            if self.0.as_ref().unwrap().value == value {
                return
            }

            // Решаем, присвоить влево или вправо?
            // Рекурсивный вызов этого же метода для
            // поддерева ниже.
            if value < self.0.as_ref().unwrap().value {
                self.0.as_mut().unwrap().left.insert(value);
            } else {
                self.0.as_mut().unwrap().right.insert(value);
            }
        }
    }

    // Проверка наличия элемента. Рекурсивная
    fn has(&self, value : &T) -> bool {
        // Проверка разных случаев

        // 1. Элемент не найден
        if self.0.is_none() {
            return false
        }

        // 2. Элемент успешно найден
        if self.0.as_ref().unwrap().value == *value {
            return true
        }

        // 3. Рекурсивный вызов
        if *value < self.0.as_ref().unwrap().value {
            self.0.as_ref().unwrap().left.has(value)
        } else {
            self.0.as_ref().unwrap().right.has(value)
        }
    }

    // Расчёт размера делева. Рекурсивный
    fn len(&self) -> usize {
        // Идея та же, что и в has()

        // 1. Поддеревьев нет
        if self.0.is_none() {
            return 0
        }

        // 2. Есть поддеревья.
        // Добавляем к результату единицу за текущее дерево
        // и прибавляем результат поддеревьев.
        1 + self.0.as_ref().unwrap().left.len() + self.0.as_ref().unwrap().right.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}

pub fn demonstrate() {
    println!("--- Task 13 ---");
    println!("Тесты успешно пройдены!");
    println!();
}