use std::collections::HashMap;

mod models;
mod db;
mod ui;
mod io_utils;

use models::*;

fn main() {
    let mut db_state = DBState {
        last_item_id: 0,
        epics: HashMap::new(),
        stories: HashMap::new(),
    };

    let epic = Epic::new("project 1".to_string(), "This is project 1".to_string());
    let story = Story::new("story 1".to_string(), "This is story 1".to_string());

    db_state.epics.insert(1, epic);
    db_state.stories.insert(2, story);

    println!("{:?}", db_state);
}
