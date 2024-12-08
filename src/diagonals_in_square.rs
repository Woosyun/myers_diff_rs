pub struct DiagonalsInSquare {
    x_len: usize,
    y_len: usize,
    diagonals: Vec<usize>,
}

impl DiagonalsInSquare {
    pub fn new(fill: usize, x_len: usize, y_len: usize) -> Self {
        Self {
            x_len,
            y_len,
            diagonals: vec![fill; x_len + y_len + 1],
        }
    }
    
    fn diagonal(&self, diagonal: isize) -> usize {
        assert!(diagonal >= -(self.x_len as isize));
        assert!(diagonal <= self.y_len as isize);
        
        let diagonal = diagonal + self.x_len as isize;
        diagonal as usize
    }
    pub fn get_x(&self, diagonal: isize) -> usize {
        let diagonal = self.diagonal(diagonal);
        let x = self.diagonals.get(diagonal).expect("diagonal index is out of bound").to_owned();

        x
    }
    pub fn set_x(&mut self, diagonal: isize, value: usize) {
        assert!(value <= self.x_len, "value should be smaller than or equal to length of x");
        
        let diagonal = self.diagonal(diagonal);
        self.diagonals[diagonal] = value;
    }
}