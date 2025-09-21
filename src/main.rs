use crossterm::event::{self, Event}; 
use ratatui::{text::Text, Frame}; 
use std::io::{self, BufRead}; 
use std::error::Error;

fn main() {
    let mut dir = String::new();
    if dir == "" {
        get_dir(&mut dir);
    } 
    let mut terminal = ratatui::init(); 
    loop {
        terminal.draw(draw).expect("failed to draw frame"); 
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore(); 
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World");
    frame.render_widget(text, frame.area());
}

fn get_dir(dir: &mut String) -> Result<(), Box<dyn Error>> {
    println!("Enter the PATH for your project directory"); 
    let stdin = io::stdin(); 
    let mut handle = stdin.lock(); 
    let mut buf = vec![]; 
    if dir == "" {
        handle.read_until(b'\n', &mut buf)?;
        *dir = String::from_utf8_lossy(&buf).to_string(); 
    }

    Ok(())
} 
