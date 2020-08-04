use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Movement {
    path: Vec<(usize, usize)>,
    path_i: usize,
    options: MovementOptions,
    go: bool,
}

impl Movement {
    pub fn new(path: Vec<(usize, usize)>, options: MovementOptions) -> Movement {
        Movement{
            path,
            path_i: 0,
            options,
            go: false,
        }
    }

    pub fn path_complete(&self) -> bool {
        self.path_i >= self.path.len() - 1
    }

    pub fn get_move(&mut self) -> (usize, usize, usize, usize) {
        let (a_x, a_y) = self.path[self.path_i];
        let (b_x, b_y) = self.path[self.path_i+1];
        (a_x, a_y, b_x, b_y)
    }

    pub fn next_move(&mut self) {
        self.path_i += 1;
    }

    pub fn execute(&mut self) {
        self.go = true;
    }

    pub fn is_go(&self) -> bool {
        self.go
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct MovementOptions {
    pub range: usize,
    // flying, incorporeal will go here
    pub line: bool,
}