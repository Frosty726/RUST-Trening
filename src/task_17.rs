// Объявление о существовании модуля
mod widgets;

// Импорт элементов из модуля в текущее пространство
use widgets::{Button, Label, Window};
// Импорт трейта для видимости его методов
use widgets::Widget;

pub fn demonstrate() {
    println!("--- Task 17 ---");
    let mut window = Window::new("Демонстрация графического интерфейса Rust 1.23");
    window.add_widget(Box::new(Label::new("Это маленькая демонстрация графического интерфейса.")));
    window.add_widget(Box::new(Button::new("Нажми меня!")));
    window.draw();
    println!();
}