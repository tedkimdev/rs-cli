use std::rc::Rc;

mod models;
mod db;
mod ui;
mod io_utils;
mod navigator;

use crate::{db::JiraDatabase, io_utils::{get_user_input, wait_for_key_press}, navigator::Navigator};

fn main() {
    let db = JiraDatabase::new("data/db.json".to_string());
    
    let mut navigator = Navigator::new(Rc::new(db));
    
    loop {
        clearscreen::clear().unwrap();
        let current_page = navigator.get_current_page();
        if current_page.is_none() {
            break;
        }
        let page = current_page.unwrap();
        if let Err(e) = page.draw_page() {
            println!("Error rendering page: {}\nPress any key to continue...", e);
            wait_for_key_press();
        }
        let input = get_user_input();
        match page.handle_input(&input) {
            Ok(Some(action)) => {
                if let Err(e) = navigator.handle_action(action) {
                    println!("Error handling action: {}\nPress any key to continue...", e);
                    wait_for_key_press();
                }
            }
            Ok(None) => {
                println!("Please enter a valid input\nPress any key to continue...");
                wait_for_key_press();
            }
            Err(e) => {
                println!("Error handling input: {}\nPress any key to continue...", e);
                wait_for_key_press();
            }
        }
    }
}
