use std::rc::Rc;

use itertools::Itertools;
use anyhow::{anyhow, Result};

use crate::db::JiraDatabase;
use crate::models::Action;
use crate::ui::pages::page::Page;
use crate::ui::pages::page_helpers::get_column_string;

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for EpicDetail {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        println!("{:^5} | {:<12} | {:<27} | {:^18}", self.epic_id, get_column_string(&epic.name, 12), get_column_string(&epic.description, 27), epic.status);
  
        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &db_state.stories;

        for (id, story) in stories
            .iter()
            .sorted_by_key(|(id, _)| *id) {
            println!("{:2^11} | {:<32} | {:^18}", id, get_column_string(&story.name, 32), story.status);
        }

        println!();
        println!();

        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus { epic_id: self.epic_id })),
            "d" => Ok(Some(Action::DeleteEpic { epic_id: self.epic_id })),
            "c" => Ok(Some(Action::CreateStory { epic_id: self.epic_id })),
            _ => {
                if let Ok(id) = input.parse::<u32>() {
                    let db_state =self.db.read_db()?;
                    match db_state.stories.get(&id) {
                        Some(_) => Ok(Some(Action::NavigateToStoryDetail { epic_id: self.epic_id, story_id: id })),
                        None => Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::test_utils::MockDB};
    use crate::models::{Epic, Story};

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert!(page.draw_page().is_ok());
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert!(page.handle_input("").is_ok());
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = EpicDetail { epic_id: 999, db };
            assert!(page.draw_page().is_err());
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateEpicStatus { epic_id: 1 }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteEpic { epic_id: 1 }));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateStory { epic_id: 1 }));
            assert_eq!(page.handle_input(&story_id.to_string()).unwrap(), Some(Action::NavigateToStoryDetail { epic_id: 1, story_id: 2 }));
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }
}