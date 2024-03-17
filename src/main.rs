use fltk::{app, enums::Event, input::MultilineInput, prelude::*, window::Window};
use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // create the window itself
    let app = app::App::default();
    let mut wind = Window::default().with_size(620, 130);
    let folder_path = "/home/mat/";

    let mut editor = MultilineInput::new(5, 5, 610, 390, "");
    let editor_for_closure = editor.clone();

    wind.end();
    wind.show();

    // this is necessary for transparency, for some reason
    let mut wind_clone = wind.clone();
    wind_clone.set_opacity(0.75);

    editor.handle(move |_, ev| match ev {
        Event::KeyDown => {
            if app::event_key() == fltk::enums::Key::from_char('q') && app::is_event_ctrl() {
                // Generate the filename based on the current date and time.
                let filename = Local::now().format("%Y-%m-%d_%H-%M-%S.txt").to_string();
                
                // Use PathBuf to construct the full path (folder path + filename).
                let mut file_path = PathBuf::from(folder_path);
                file_path.push(&filename);
                
                // Create a file at the specified path.
                let mut file = File::create(&file_path).expect("Failed to create file");
                file.write_all(editor_for_closure.value().as_bytes()).expect("Failed to write to file");
                println!("File saved to {:?}", file_path);
                std::process::exit(0); // Exit after saving
            }
            true
        }
        _ => false,
    });

    // send to ankiConnect

    app.run().unwrap();
}
