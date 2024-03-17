use fltk::{app, enums::Event, input::MultilineInput, prelude::*, window::Window};
use chrono::Local;
use std::fs::File;
use std::io::Write;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(620, 130);
    //wind.set_border(false);

    let mut editor = MultilineInput::new(5, 5, 610, 390, "");
    let editor_for_closure = editor.clone();

    wind.end();
    wind.show();

    let mut wind_clone = wind.clone();
    wind_clone.set_opacity(0.75);

    editor.handle(move |_, ev| match ev {
        Event::KeyDown => {
            if app::event_key() == fltk::enums::Key::from_char('q') && app::is_event_ctrl() {
                let filename = Local::now().format("%Y-%m-%d_%H-%M-%S.txt").to_string();
                let mut file = File::create(&filename).expect("Failed to create file");
                file.write_all(editor_for_closure.value().as_bytes()).expect("Failed to write to file");
                std::process::exit(0); // Exit after saving
            }
            true
        }
        _ => false,
    });

    app.run().unwrap();
}
