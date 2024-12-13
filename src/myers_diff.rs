#[derive(PartialEq, Debug)]
pub enum EditAction {
    Add(usize, String),
    Delete(usize),
}

pub fn myers_diff(before: String, after: String) -> Result<Vec<EditAction>, String> {
    let before: Vec<&str> = before.lines().collect::<Vec<&str>>();
    let after: Vec<&str> = after.lines().collect::<Vec<&str>>();

    let mut edit_graph = EditGraph::new(&before, &after);
    
    edit_graph.get_edit_script(0)
        .map(|(_, v)| v)
}






#[derive(PartialEq, Debug)]
struct MiddleSnake {
    pub x: (usize, usize),
    pub y: (usize, usize),
}

impl MiddleSnake {
    fn from_edit_graph(diagonal: isize, x1: usize, x2: usize) -> MiddleSnake {
        let y1 = x1 as isize - diagonal;
        let y2 = x2 as isize - diagonal;
        
        MiddleSnake {
            x: (x1, x2),
            y: (y1 as usize, y2 as usize),
        }
    }
}

#[derive(Debug)]
struct EditGraph<'a>
{
    a: &'a [&'a str],
    b: &'a [&'a str],
    forward_diagonals: Vec<usize>,
    backward_diagonals: Vec<usize>,
}

impl<'a> EditGraph<'a>
{
    fn new(a: &'a [&'a str], b: &'a [&'a str]) -> Self {
        let max = std::cmp::max(a.len(), b.len());
        EditGraph {
            a,
            b,
            forward_diagonals: vec![0; 2*max + 1],
            backward_diagonals: vec![a.len(); 2*max + 1],
        }
    }

    fn max_axis(&self) -> usize {
        std::cmp::max(self.a.len(), self.b.len())
    }
    
    fn delta(&self) -> isize {
        self.a.len() as isize - self.b.len() as isize
    }

    fn update_forward_diagonal(&mut self, k: isize, d: isize) {
        let idx = (k + self.max_axis() as isize) as usize;
        
        let l = || self.forward_diagonals[idx-1];
        let r = || self.forward_diagonals[idx+1];
        
        let mut x = if k==-d || (k!=d && r() > l()) {
            r()
        } else {
            l() + 1
        };

        let mut y = (x as isize - k) as usize;
        // x + 1 <= a.len() && y + 1 <= b.len() && (x+1)th a == (y+1)th b
        while x < self.a.len() && y < self.b.len() && self.a[x] == self.b[y] {
            x += 1;
            y += 1;
        }
        self.forward_diagonals[idx] = x;
    }

    // k and d are related to length of edit script, not direct diagonal index
    fn update_backward_diagonal(&mut self, k: isize, d: isize) {
        let idx = (self.delta() + k + self.max_axis() as isize) as usize;
        
        let l = || self.backward_diagonals[idx-1];
        let r = || self.backward_diagonals[idx+1];
        
        let mut x = if k==d || (k != -d && l() < r()) {
            l()
        } else {
            r() - 1
        };

        let mut y = x as isize - k - self.delta();
        while x > 0 && y > 0 && self.a[x-1] == self.b[y as usize - 1] {
            x -= 1;
            y -= 1;
        }
        self.backward_diagonals[idx] = x;
    }

    fn find_middle_snake(&mut self) -> Option<MiddleSnake> {
        let boundary = self.a.len() + self.b.len();

        for d in 0..= boundary.div_ceil(2) as isize {
            //phase 1
            for k in (-d..=d).step_by(2) {
                self.update_forward_diagonal(k, d);

                if self.delta().abs() % 2 == 1 && self.delta() - (d - 1) <= k && k <= self.delta() + (d + 1) {
                    let idx = (k + self.max_axis() as isize) as usize;
                    
                    let forward_x = self.forward_diagonals[idx];
                    let backward_x = self.backward_diagonals[idx];
                    if backward_x <= forward_x {
                        return Some(MiddleSnake::from_edit_graph(k, backward_x, forward_x));
                    }
                }
            }

            // phase 2
            for k in (-d..=d).step_by(2) {
                self.update_backward_diagonal(k, d);

                if self.delta().abs() % 2 == 0 && -d <= k + self.delta() && k + self.delta() <= d {
                    let idx = (self.delta() + k + self.b.len() as isize) as usize;
                    
                    let forward_x = self.forward_diagonals[idx];
                    let backward_x = self.backward_diagonals[idx];
                    if backward_x <= forward_x {
                        return Some(MiddleSnake::from_edit_graph(k + self.delta(), backward_x, forward_x));
                    }
                }
            }
        }

        None
    }

    fn get_edit_script(&mut self, base: usize) -> Result<(usize, Vec<EditAction>), String> {
        if self.a.len() == 0 {
            let adds = self.b
                .iter()
                .enumerate()
                .map(|(_, &str)| EditAction::Add(base, str.to_string()))
                .collect::<Vec<EditAction>>();

            return Ok((base, adds));
        } else if self.b.len() == 0 {
            let deletes = self.a
                .iter()
                .enumerate()
                .map(|_| EditAction::Delete(base))
                .collect::<Vec<EditAction>>();

            return Ok((base, deletes));
        }

        let middle_snake = self.find_middle_snake().expect("missing middle_snake");

        let before_front = self.a
            .iter()
            .take(middle_snake.x.0)
            .map(|&t| t)
            .collect::<Vec<&str>>();
        let after_front = self.b
            .iter()
            .take(middle_snake.y.0)
            .map(|&t| t)
            .collect::<Vec<&str>>();

        let mut front_graph = EditGraph::new(&before_front, &after_front);
        let (base, mut front) = front_graph.get_edit_script(base)?;
        let base = base + middle_snake.x.1 - middle_snake.x.0;

        let before_back = self.a
            .iter()
            .skip(middle_snake.x.1)
            .map(|&t| t)
            .collect::<Vec<&str>>();
        let after_back = self.b
            .iter()
            .skip(middle_snake.y.1)
            .map(|&t| t)
            .collect::<Vec<&str>>();
        let mut back_graph = EditGraph::new(&before_back, &after_back);
        let (base, back)= back_graph.get_edit_script(base)?;
        
        front.extend(back);

        Ok((base, front))
    }
}


#[cfg(test)] 
pub mod tests {
    use super::*;
    use std::fs;

    #[test] 
    pub fn initialize_edit_graph() {
        let edit_graph = EditGraph::new(&["0", "1"], &["0", "1"]);
        
        assert_eq!(edit_graph.forward_diagonals[2], 0);
        assert_eq!(edit_graph.backward_diagonals[2], 2);
    }
    
    #[test] 
    pub fn forward_once() {
        let mut edit_graph = EditGraph::new(&["0", "1"], &["0", "1"]);
        edit_graph.update_forward_diagonal(0, 0);
        
        assert_eq!(edit_graph.forward_diagonals[2], 2);
    }

    #[test] 
    pub fn backward_once() {
        let mut edit_graph = EditGraph::new(&["0", "1"], &["0", "1"]);
        edit_graph.update_backward_diagonal(0, 0);

        assert_eq!(edit_graph.backward_diagonals[2], 0);
    }

    #[test] 
    pub fn edit_none() {
        let mut edit_graph = EditGraph::new(&["0", "1"], &["2", "3"]);
        edit_graph.update_forward_diagonal(0, 0);
        
        assert_eq!(edit_graph.forward_diagonals[2], 0);
    }
    #[test] 
    pub fn edit_once() {
        let mut edit_graph = EditGraph::new(&["0", "1"], &["2", "3"]);
        edit_graph.update_forward_diagonal(0, 0);
        edit_graph.update_forward_diagonal(-1, 1);
        edit_graph.update_forward_diagonal(1, 1);
        
        assert_eq!(edit_graph.forward_diagonals[3], 1);
        assert_eq!(edit_graph.forward_diagonals[1], 0);

        edit_graph.update_backward_diagonal(0, 0);
        edit_graph.update_backward_diagonal(-1, 1);
        edit_graph.update_backward_diagonal(1, 1);

        assert_eq!(edit_graph.backward_diagonals[1], 1);
        assert_eq!(edit_graph.backward_diagonals[3], 2);
    }

    #[test] 
    pub fn find_middle_snake_1() {
        let mut edit_graph = EditGraph::new(&["0", "1"], &["1", "3"]);
        let middle_snake = edit_graph.find_middle_snake().expect("middle snake not found");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(1, 1, 2));
    }

    #[test] 
    pub fn find_middle_snake_2() {
        let mut edit_graph = EditGraph::new(&["a", "b", "c"], &["b", "c", "d"]);
        let middle_snake = edit_graph.find_middle_snake().expect("middle snake not found");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(1, 1, 3));
    }


    #[test] 
    pub fn edit_nothing() {
        let file1 = fs::read_to_string("src/samples/hello_world.txt").expect("missing hello_world.txt");
        let file2 = fs::read_to_string("src/samples/hello_world.txt").expect("missing hello_world.txt");

        let edit_actions = myers_diff(file1, file2).expect("failed to diff");
        
        assert_eq!(edit_actions, vec![]);
    }

    #[test]
    pub fn edit_one_line() {
        let file1 = fs::read_to_string("src/samples/hello_world1.txt").expect("missing hello_world1.txt");
        let file2 = fs::read_to_string("src/samples/hello_world2.txt").expect("missing hello_world2.txt");

        let edit_actions = myers_diff(file1, file2).expect("failed to diff");
        let ans = vec![
            EditAction::Add(4, "Have a great day".to_string()),
            EditAction::Delete(4),
        ];

        assert_eq!(edit_actions, ans);
    }

    #[test]
    pub fn delete_one_line() {
        let file1 = fs::read_to_string("src/samples/hello_world.txt").expect("missing file1");
        let file2 = fs::read_to_string("src/samples/nothing.txt").expect("missing file2");
    
        let re: Vec<EditAction> = myers_diff(file1, file2).expect("failed to diff");
        let ans = vec![
            EditAction::Delete(0)
        ];

        assert_eq!(re, ans);
    }

    #[test] 
    pub fn edit_dog_to_cat() {
        let file1 = fs::read_to_string("src/samples/dog.txt").expect("missing file1");
        let file2 = fs::read_to_string("src/samples/cat.txt").expect("missing file2");

        let re = myers_diff(file1, file2).expect("failed to diff");

        assert_eq!(re, vec![]);
    }
}