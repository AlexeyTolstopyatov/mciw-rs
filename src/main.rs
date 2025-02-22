use iced::widget::{button, column, row, text, Column};

//
// Declare Counter entity
// It needs to hold application's entity values
//
#[derive(Default)]
struct Counter {
    value: i64,
}

//
// Declare Message enumeration for
//
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Multiply,
    Reverse,
}

fn main() {
    iced::run(
        "Window Title",
        view_update,
        view_counter
    ).expect("something went wrong with view");
}

fn view_counter(counter: &Counter) -> Column<Message> {
    // rust return values stays without semicolon
    // build window makrup here?
    column![
        text(format!("Value: {}", counter.value)),
        row![
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
            button("Multiply").on_press(Message::Multiply),
            button("Reverse").on_press(Message::Reverse),
        ].spacing(5)
    ].spacing(10)
}

fn view_update(counter: &mut Counter, message: Message) -> () {
    match message {
        // set by pointer
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
        Message::Multiply => counter.value *= counter.value,
        // x/0 panic handling
        Message::Reverse => counter.value *= 1 / counter.value,
    }
}