use crate::diagonals_in_square::DiagonalsInSquare;

pub struct EditGraph<T> {
    a: Vec<T>,
    b: Vec<T>,
    forward_diagonals: DiagonalsInSquare,
    backward_diagonals: DiagonalsInSquare,
}

impl<T> EditGraph<T> 
where T: Clone + PartialEq
{
    pub fn new(a: Vec<T>, b: Vec<T>) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            forward_diagonals: DiagonalsInSquare::new(0, a.len(), b.len()),
            backward_diagonals: DiagonalsInSquare::new(a.len(), a.len(), b.len()),
        }
    }

    pub fn update_forward_diagonal(&mut self, k: isize, d: isize) {
        let mut x = if k==d || (k!=-d && self.forward_diagonals.get_x(k-1) > self.forward_diagonals.get_x(k+1)) {
            self.forward_diagonals.get_x(k-1)
        } else {
            self.forward_diagonals.get_x(k+1) + 1
        };
        let y = x as isize + k;
        assert!(y >= 0);
        let mut y = y as usize;

        while x < self.a.len() && y < self.b.len() && self.a[x] == self.b[y] {
            x += 1;
            y += 1;
        }
        self.forward_diagonals.set_x(k, x);
    }
    pub fn update_backward_diagonal(&mut self, k: isize, d: isize) {
        let mut x = if k==-d || (k!=d && self.backward_diagonals.get_x(k+1) < self.backward_diagonals.get_x(k-1)) {
            self.backward_diagonals.get_x(k+1)
        } else {
            self.backward_diagonals.get_x(k-1) - 1
        };
        let y = x as isize + k;
        assert!(y >= 0);
        let mut y = y as usize;

        while x > 0 && y > 0 && self.a[x-1] == self.b[y-1] {
            x -= 1;
            y -= 1;
        }
        self.backward_diagonals.set_x(k, x);
    }

    pub fn find_middle_snake(&mut self) -> Option<(isize, usize, usize)> {
        let boundary = (self.a.len() + self.b.len()) / 2;
        for d in 0..=boundary as isize {
            //phase 1
            for k in (-d..=d).step_by(2) {
                self.update_forward_diagonal(k, d);

                let l = self.forward_diagonals.get_x(k);
                let r = self.backward_diagonals.get_x(k);
                if l >= r {
                    return Some((k, l, r));
                }
            }

            // phase 2
            for k in (-d..=d).step_by(2) {
                self.update_backward_diagonal(k, d);

                let l = self.forward_diagonals.get_x(k);
                let r = self.backward_diagonals.get_x(k);
                if l >= r {
                    return Some((k, l, r));
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
        let edit_graph = EditGraph::new(vec![0, 1], vec![0, 1]);
        
        assert_eq!(edit_graph.forward_diagonals.get_x(0), 0);
        assert_eq!(edit_graph.backward_diagonals.get_x(0), 2);
    }
    
    #[test] 
    pub fn forward_once() {
        let mut edit_graph = EditGraph::new(vec![0, 1], vec![0, 1]);
        edit_graph.update_forward_diagonal(0, 0);
        assert_eq!(edit_graph.forward_diagonals.get_x(0), 2);
    }

    #[test] 
    pub fn backward_once() {
        let mut edit_graph = EditGraph::new(vec![0, 1], vec![0, 1]);
        edit_graph.update_backward_diagonal(0, 0);

        assert_eq!(edit_graph.backward_diagonals.get_x(0), 0);
    }

    #[test] 
    pub fn edit_none() {
        let mut edit_graph = EditGraph::new(vec![0, 1], vec![2, 3]);
        edit_graph.update_forward_diagonal(0, 0);
        
        assert_eq!(edit_graph.forward_diagonals.get_x(0), 0);
    }
    #[test] 
    pub fn edit_once() {
        let mut edit_graph = EditGraph::new(vec![0, 1], vec![2, 3]);
        edit_graph.update_forward_diagonal(0, 0);
        edit_graph.update_forward_diagonal(1, 1);
        edit_graph.update_forward_diagonal(-1, 1);
        
        assert_eq!(edit_graph.forward_diagonals.get_x(1), 0);
        assert_eq!(edit_graph.forward_diagonals.get_x(-1), 1);

        edit_graph.update_backward_diagonal(0, 0);
        edit_graph.update_backward_diagonal(1, 1);
        edit_graph.update_backward_diagonal(-1, 1);

        assert_eq!(edit_graph.backward_diagonals.get_x(-1), 2);
        assert_eq!(edit_graph.backward_diagonals.get_x(1), 1);
    }

    #[test] 
    pub fn find_middle_snake_1() {
        let mut edit_graph = EditGraph::new(vec![0, 1], vec![1, 3]);
        let middle_snake = edit_graph.find_middle_snake().expect("middle snake not found");
        assert_eq!(middle_snake, (-1, 1, 2));
    }
}