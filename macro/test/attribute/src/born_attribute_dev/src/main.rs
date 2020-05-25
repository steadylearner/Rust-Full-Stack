#[macro_use] extern crate attribute;

#[web_event_base()]
#[derive(Debug)]
enum WebEvent {
    KeyPress(char),
    Click { x: i64, y: i64 },
    Paste(String),
}

fn main() {

}
