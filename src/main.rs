use iced::widget::{button, column, text, Column};
// ARGS Parser
// use clap::Parser;

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    // Reset,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
    // fn view(&self) -> Column<Message> {
    //     // The buttons
    //     let increment = button("+").on_press(Message::Increment);
    //     let decrement = button("-").on_press(Message::Decrement);
    //
    //     // The number
    //     let counter = text(self.value);
    //
    //     // The layout
    //     let interface = column![increment, counter, decrement];
    //
    //     interface
    // }
    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}

fn main() {
    // let args = Args::parse();
    // if is_verbose_mode(&args) {
    //     println!("DEBUG {args:?}");
    // }

    // Init the state of the runtime as per https://book.iced.rs/the-runtime.html
    // let mut counter = Counter { value: 0 };
    // let mut counter = Counter::default();

    iced::run("A cool counter", Counter::update, Counter::view).expect("REASON")
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter { value: 0 };
    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);
    counter.update(Message::Increment);

    assert_eq!(counter.value, 2)
}
