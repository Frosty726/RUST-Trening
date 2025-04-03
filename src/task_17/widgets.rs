// Описание структуры модуля
pub mod button;
pub mod label;
pub mod window;

pub use button::Button;
pub use label::Label;
pub use window::Window;

// Общий трейт для всех подмодулей
pub trait Widget {
    /// Ширина self.
    fn width(&self) -> usize;

    /// Прорисовка виджета в буфер.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Прорисовка виджета.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}