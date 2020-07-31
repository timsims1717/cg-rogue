use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Character {
    pub x: usize,
    pub y: usize,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Movement {
    path: Vec<(usize, usize)>,
    path_i: usize,
    smooth: bool,
}

impl Movement {
    pub fn new(path: Vec<(usize, usize)>, smooth: bool) -> Movement {
        Movement{
            path,
            path_i: 0,
            smooth,
        }
    }

    pub fn path_complete(&self) -> bool {
        self.path_i >= self.path.len()
    }

    pub fn get_move(&mut self) -> (usize, usize, usize, usize) {
        if self.path_i == 0 {
            self.path_i += 1;
        }
        let (a_x, a_y) = self.path[self.path_i-1];
        let (b_x, b_y) = self.path[self.path_i];
        (a_x, a_y, b_x, b_y)
    }

    pub fn next_move(&mut self) {
        self.path_i += 1;
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}