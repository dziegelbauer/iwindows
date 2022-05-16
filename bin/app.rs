extern crate iwindows;
use iwindows::Window;

fn main() -> () {
    let mut win = Window::new();
    if let Ok(()) = win.initialize() {
        win.show();
        win.process_messages();
    }
}