#[derive(PartialEq, Debug)]
pub struct MiddleSnake {
    pub x: (usize, usize),
    pub y: (usize, usize),
}

impl MiddleSnake {
    pub fn from_edit_graph(diagonal: isize, x1: usize, x2: usize) -> MiddleSnake {
        let y1 = x1 as isize - diagonal;
        let y2 = x2 as isize - diagonal;
        
        MiddleSnake {
            x: (x1, x2),
            y: (y1 as usize, y2 as usize),
        }
    }
}

#[derive(Debug)]
pub struct EditGraph<'a>
{
    a: &'a [&'a str],
    b: &'a [&'a str],
    forward_diagonals: Vec<usize>,
    backward_diagonals: Vec<usize>,
}

impl<'a> EditGraph<'a>
{
    pub fn new(a: &'a [&'a str], b: &'a [&'a str]) -> Self {
        let max = std::cmp::max(a.len(), b.len());
        let mut new_edit_graph = EditGraph {
            a,
            b,
            forward_diagonals: vec![0; 2*max + 1],
            backward_diagonals: vec![a.len(); 2*max + 1],
        };
        
        let d0 = b.len();
        new_edit_graph.forward_diagonals[d0] = 0;
        
        let d_delta = (new_edit_graph.delta() + new_edit_graph.b.len() as isize) as usize;
        new_edit_graph.backward_diagonals[d_delta] = a.len();

        new_edit_graph
    }
    
    fn delta(&self) -> isize {
        self.a.len() as isize - self.b.len() as isize
    }

    pub fn update_forward_diagonal(&mut self, k: isize, d: isize) {
        let idx = (k + self.b.len() as isize) as usize;
        
        let r = self.forward_diagonals[idx + 1];
        let l = self.forward_diagonals[idx - 1];
        
        let mut x = if k==-d || (k!=d && r > l) {
            r
        } else {
            l + 1
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
    pub fn update_backward_diagonal(&mut self, k: isize, d: isize) {
        let idx = (self.delta() + k + self.b.len() as isize) as usize;
        
        let r = self.backward_diagonals[idx + 1];
        let l = self.backward_diagonals[idx - 1];
        let mut x = if k==self.delta()+d || (k!=self.delta()-d && l <= r) {
            l
        } else {
            r - 1
        };

        let mut y = (x as isize - k - self.delta()) as usize;
        while x > 0 && y > 0 && self.a[x-1] == self.b[y-1] {
            x -= 1;
            y -= 1;
        }
        self.backward_diagonals[idx] = x;
    }

    pub fn find_middle_snake(&mut self) -> Option<MiddleSnake> {
        for d in 0..= self.a.len().div_ceil(self.b.len()) as isize {
            //phase 1
            for k in (-d..=d).step_by(2) {
                self.update_forward_diagonal(k, d);

                if self.delta().abs() % 2 == 1 && self.delta() - (d - 1) <= k && k <= self.delta() + (d + 1) {
                    let idx = (k + self.b.len() as isize) as usize;
                    
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
}


#[cfg(test)] 
pub mod tests {
    use super::*;

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
}