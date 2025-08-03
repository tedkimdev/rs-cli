mod models;

use std::collections::HashMap;

use models::*;

fn main() {
    let mut db = DBState {
        last_item_id: 0,
        epics: HashMap::new(),
        stories: HashMap::new(),
    };

    let epic = Epic::new("project 1".to_string(), "This is project 1".to_string());
    let story = Story::new("story 1".to_string(), "This is story 1".to_string());

    db.epics.insert(epic.id, epic);
    db.stories.insert(story.id, story);

    println!("{:?}", db);
}
