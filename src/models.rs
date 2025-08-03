use std::collections::HashMap;

#[derive(Debug)]
pub enum Status {
    Open,
    InProgress,
    Resoved,
    Closed,
}

#[derive(Debug)]
pub struct Epic {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 0, // TODO: use global integer id counter
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Story {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 0, // TODO: use global integer id counter
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Debug)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}