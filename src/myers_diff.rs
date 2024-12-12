use crate::edit_graph::EditGraph;

#[derive(PartialEq, Debug)]
pub enum EditAction {
    Add(usize, String),
    Delete(usize),
}

pub struct EditScript {
    before: String,
    after: String,
}

impl EditScript {
    pub fn new(before: String, after: String) -> Self {
        Self {
            before,
            after
        }
    }

    fn myers_diff(&self, len_of_lcs: usize, before: &[&str], after: &[&str]) -> Result<(usize, Vec<EditAction>), String> {
        dbg!(len_of_lcs, before, after);
        
        if before.len() == 0 {
            let adds = after
                .iter()
                .enumerate()
                .map(|(idx, &str)| EditAction::Add(idx+len_of_lcs, str.to_string()))
                .collect::<Vec<EditAction>>();

            return Ok((len_of_lcs, adds));
        } else if after.len() == 0 {
            let deletes = before
                .iter()
                .enumerate()
                .map(|(idx, _)| EditAction::Delete(idx+len_of_lcs))
                .collect::<Vec<EditAction>>();

            return Ok((len_of_lcs, deletes));
        }

        let mut edit_graph = EditGraph::new(before, after);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle snake");

        let before_front = before
            .iter()
            .take(middle_snake.x.0)
            .map(|&t| t)
            .collect::<Vec<&str>>();
        let after_front = after
            .iter()
            .take(middle_snake.y.0)
            .map(|&t| t)
            .collect::<Vec<&str>>();

        let before_back = before
            .iter()
            .skip(middle_snake.x.1)
            // .take(before.len() - middle_snake.x.1 + 1)
            .map(|&t| t)
            .collect::<Vec<&str>>();
        let after_back = after
            .iter()
            .skip(middle_snake.y.1)
            // .take(after.len() - middle_snake.y.1 + 1)
            .map(|&t| t)
            .collect::<Vec<&str>>();

        let (temp, mut front) = self.myers_diff(len_of_lcs, &before_front, &after_front)?;
        let len_of_lcs = len_of_lcs + temp + middle_snake.x.1 - middle_snake.x.0;
        let (len_of_lcs, back)= self.myers_diff(len_of_lcs, &before_back, &after_back)?;
        front.extend(back);
        
        Ok((len_of_lcs, front))
    }

    pub fn diff(&self) -> Result<Vec<EditAction>, String> {
        let before: Vec<&str> = self.before.lines().collect::<Vec<&str>>();
        let after: Vec<&str> = self.after.lines().collect::<Vec<&str>>();

        self.myers_diff(0, &before, &after)
            .map(|(_, v)| v)
    }
}

#[cfg(test)] 
pub mod tests {
    use std::fs;
    use super::*;

    #[test] 
    pub fn edit_nothing() {
        let file1 = fs::read_to_string("src/samples/same1.txt").expect("missing same1.txt");
        let file2 = fs::read_to_string("src/samples/same2.txt").expect("missing same2.txt");

        let edit_script = EditScript::new(file1, file2);

        assert_eq!(edit_script.diff(), Ok(vec![]));
    }

    #[test]
    pub fn edit_one_line() {
        let file1 = fs::read_to_string("src/samples/hello_world1.txt").expect("missing hello_world1.txt");
        let file2 = fs::read_to_string("src/samples/hello_world2.txt").expect("missing hello_world2.txt");

        let edit_script = EditScript::new(file1, file2);
        let edit_actions = vec![
            EditAction::Add(4, "Have a great day".to_string()),
            EditAction::Delete(4),
        ];

        assert_eq!(edit_script.diff(), Ok(edit_actions));
    }
}