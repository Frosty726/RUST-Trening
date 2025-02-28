#[derive(Debug)]
/// Событие лифта на которое должен реагировать контроллер.
enum Event {
    Arrived(i32),
    DoorOpened,
    DoorClosed,
    Called{floor : i32, dir : Direction},
    ButtonPushed(i32),
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// Кабина приехала на заданный этаж.
fn car_arrived(floor: i32) -> Event {
    Event::Arrived(floor)
}

/// Двери кабины открыты.
fn car_door_opened() -> Event {
    Event::DoorOpened
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::Called{floor: floor, dir: dir}
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPushed(floor)
}

pub fn demonstrate() {
    println!("--- Task 5 ---");
    println!(
        "Пассажир на первом этаже нажал кнопку вызова: {:?}",
        lobby_call_button_pressed(1, Direction::Up)
    );
    println!("Лифт приехал на первый этаж: {:?}", car_arrived(1));
    println!("Дверь лифта открылась: {:?}", car_door_opened());
    println!(
        "Пассажир нажал кнопку третьего этажа: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Двери лифта закрылись: {:?}", car_door_closed());
    println!("Лифт прибыл на третий этаж: {:?}", car_arrived(3));
}