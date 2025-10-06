mod app; 
mod ui; 

use std::io::{self, Write}; 
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::app::App;

fn main() -> Result<(),Box<dyn Error>> {
    ensure_tmux_available();
    let mut dir = String::new();
    check_config(&mut dir); 
    if dir.is_empty() {
        dir = get_dir()?;
        save_config(&dir);
    } 
    load_config(&mut dir); 
    let paths = get_paths(&dir)?;
    let terminal = ratatui::init(); 
    let app_result = App::new().run(terminal, paths);
    ratatui::restore();
    app_result 
}

fn check_config(dir: &mut String) -> Result<(), Box<dyn Error>> {
    if Path::new("quickshot.config").exists() {
        *dir = "config".to_string();
        Ok(())
    } else {
        Ok(())
    } 
} 

fn load_config(dir: &mut String) {
    let mut temp = std::fs::read_to_string("quickshot.config").unwrap();
    temp = temp.split_off(5);
    temp.pop();
    temp.pop();
    println!("{}", temp);
    *dir = temp.trim().to_string();
} 

fn save_config(dir: &String) {
    let path_config = format!("dir={}", dir);  
    std::fs::write("quickshot.config", path_config);
} 

fn get_paths(string_dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = Path::new(string_dir); 
    
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)? 
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect(); 
    paths.sort();
    Ok(paths)
} 

fn get_dir() -> Result<(String), Box<dyn Error>> {
    println!("\nEnter the PATH for your project directory in the form\n \"/home/YOUR_USERNAME/YOUR_PROJECT_DIR/\""); 
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.to_string())
} 

fn ensure_tmux_available() -> io::Result<()> {
    let status = Command::new("tmux").arg("-V").status();
    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "tmux not found (install it, or run under WSL/mac/Linux)"
        )),
    }
}
