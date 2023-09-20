use rand::Rng;

pub struct GameState {
    pub board: Vec<Vec<u8>>,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub score: i32,
    pub is_paused: bool,
    pub is_game_over: bool,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub shape: Vec<Vec<u8>>,
    pub x: i32,
    pub y: i32,
}

pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub enum GameStateEvent {
    LinesCleared(usize),
    GameOver(i32),
    Continue,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; 10]; 20],
            current_piece: Self::generate_random_piece(),
            next_piece: Self::generate_random_piece(),
            score: 0,
            is_paused: false,
            is_game_over: false,
        }
    }

    pub fn update(&mut self) -> Vec<GameStateEvent> {
        let mut events = Vec::new();

        if self.is_paused {
            return events;
        }

        self.current_piece.y += 1;

        if self.is_collision() {
            self.current_piece.y -= 1;
            self.merge_piece();

            let lines_cleared = self.clear_lines();
            self.score += lines_cleared as i32 * 100;
            if lines_cleared > 0 {
                events.push(GameStateEvent::LinesCleared(lines_cleared));
            }

            self.current_piece = self.next_piece.clone();
            self.next_piece = Self::generate_random_piece();

            if self.is_collision() {
                events.push(GameStateEvent::GameOver(self.score));
                self.is_game_over = true;
            }
        }

        if events.is_empty() {
            events.push(GameStateEvent::Continue);
        }

        events
    }

    pub fn restart(&mut self) {
        self.board = vec![vec![0; 10]; 20];
        self.current_piece = Self::generate_random_piece();
        self.next_piece = Self::generate_random_piece();
        self.score = 0;
        self.is_paused = false;
        self.is_game_over = false;
    }

    fn is_collision(&self) -> bool {
        for (y, row) in self.current_piece.shape.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 0 {
                    continue;
                }

                let board_x = self.current_piece.x + x as i32;
                let board_y = self.current_piece.y + y as i32;

                if board_x < 0 || board_x >= self.board[0].len() as i32 || board_y >= self.board.len() as i32 {
                    return true;
                }

                if board_y >= 0 && self.board[board_y as usize][board_x as usize] != 0 {
                    return true;
                }
            }
        }

        false
    }

    fn clear_lines(&mut self) -> usize {
        let mut lines_to_clear = Vec::new();

        for (y, row) in self.board.iter().enumerate() {
            if row.iter().all(|&cell| cell != 0) {
                lines_to_clear.push(y);
            }
        }

        for &line in lines_to_clear.iter().rev() {
            self.board.remove(line);
            self.board.insert(0, vec![0; 10]);
        }

        lines_to_clear.len()
    }

    fn generate_random_piece() -> Piece {
        let mut rng = rand::thread_rng();
        let piece_type = match rng.gen_range(0..=6) {
            0 => PieceType::I,
            1 => PieceType::O,
            2 => PieceType::T,
            3 => PieceType::S,
            4 => PieceType::Z,
            5 => PieceType::J,
            6 => PieceType::L,
            _ => unreachable!(),
        };

        match piece_type {
            PieceType::I => Piece {
                shape: vec![
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                x: 4,
                y: 0,
            },
            PieceType::O => Piece {
                shape: vec![
                    vec![2, 2],
                    vec![2, 2],
                ],
                x: 4,
                y: 0,
            },
            PieceType::T => Piece {
                shape: vec![
                    vec![0, 3, 0],
                    vec![3, 3, 3],
                    vec![0, 0, 0],
                ],
                x: 4,
                y: 0,
            },
            PieceType::S => Piece {
                shape: vec![
                    vec![0, 4, 4],
                    vec![4, 4, 0],
                    vec![0, 0, 0],
                ],
                x: 4,
                y: 0,
            },
            PieceType::Z => Piece {
                shape: vec![
                    vec![5, 5, 0],
                    vec![0, 5, 5],
                    vec![0, 0, 0],
                ],
                x: 4,
                y: 0,
            },
            PieceType::J => Piece {
                shape: vec![
                    vec![6, 0, 0],
                    vec![6, 6, 6],
                    vec![0, 0, 0],
                ],
                x: 4,
                y: 0,
            },
            PieceType::L => Piece {
                shape: vec![
                    vec![0, 0, 7],
                    vec![7, 7, 7],
                    vec![0, 0, 0],
                ],
                x: 4,
                y: 0,
            },
        }
    }

    fn merge_piece(&mut self) {
        for (y, row) in self.current_piece.shape.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 0 {
                    continue;
                }

                let board_x = self.current_piece.x + x as i32;
                let board_y = self.current_piece.y + y as i32;

                if board_x >= 0 && board_x < self.board[0].len() as i32 &&
                    board_y >= 0 && board_y < self.board.len() as i32 {
                    self.board[board_y as usize][board_x as usize] = value;
                }
            }
        }
    }

    pub fn move_piece(&mut self, dx: i32, dy: i32) {
        self.current_piece.x += dx;
        self.current_piece.y += dy;

        if self.is_collision() {
            self.current_piece.x -= dx;
            self.current_piece.y -= dy;
        }
    }

    pub fn hard_drop(&mut self) {
        while !self.is_collision() {
            self.current_piece.y += 1;
        }

        self.current_piece.y -= 1;
        self.merge_piece();

        let lines_cleared = self.clear_lines();
        self.score += lines_cleared as i32 * 100;

        self.current_piece = self.next_piece.clone();
        self.next_piece = Self::generate_random_piece();
    }

    pub fn rotate_piece(&mut self) {
        let mut new_shape: Vec<Vec<u8>> = vec![vec![0; self.current_piece.shape.len()]; self.current_piece.shape[0].len()];
        for y in 0..self.current_piece.shape.len() {
            for x in 0..self.current_piece.shape[y].len() {
                new_shape[x][y] = self.current_piece.shape[y][x];
            }
        }

        for row in new_shape.iter_mut() {
            row.reverse();
        }

        let mut collision_detected = false;
        for (y, row) in new_shape.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 0 {
                    continue;
                }

                let board_x = self.current_piece.x + x as i32;
                let board_y = self.current_piece.y + y as i32;

                if board_x < 0 || board_x >= self.board[0].len() as i32 || board_y >= self.board.len() as i32 {
                    collision_detected = true;
                    break;
                }

                if board_y >= 0 && self.board[board_y as usize][board_x as usize] != 0 {
                    collision_detected = true;
                    break;
                }
            }
        }

        if !collision_detected {
            self.current_piece.shape = new_shape;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }
}