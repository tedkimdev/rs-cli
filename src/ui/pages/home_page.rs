use std::rc::Rc;

use itertools::Itertools;
use anyhow::Result;

use crate::db::JiraDatabase;
use crate::models::Action;
use crate::ui::pages::page::Page;

pub struct HomePage {
    pub db: Rc<JiraDatabase>
}

impl Page for HomePage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        let db_state = self.db.read_db()?;
        for (id, epic) in db_state.epics
            .iter()
            .sorted_by_key(|(id, _)| *id){
            println!("{} | {} | {} ", id, epic.name, epic.status);
        }


        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            _ => {
                if let Ok(id) = input.parse::<u32>() {
                    let db_state = self.db.read_db()?;
                    match db_state.epics.get(&id) {
                        Some(_) => Ok(Some(Action::NavigateToEpicDetail { epic_id: id })),
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
    use crate::models::{Epic};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert!(page.draw_page().is_ok());
        }
        
        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert!(page.handle_input("").is_ok());
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(page.handle_input(&valid_epic_id).unwrap(), Some(Action::NavigateToEpicDetail { epic_id: 1 }));
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }
}