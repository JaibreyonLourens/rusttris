use eframe::egui::Color32;

// SRS Rotation States
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RotationState {
    Zero,   // Spawn state
    R,      // Clockwise rotation from spawn
    Two,    // 180 degrees from spawn
    L,      // Counter-clockwise rotation from spawn
}

impl RotationState {
    fn to_index(self) -> usize {
        match self {
            RotationState::Zero => 0,
            RotationState::R => 1,
            RotationState::Two => 2,
            RotationState::L => 3,
        }
    }

    fn next_cw(self) -> Self {
        match self {
            RotationState::Zero => RotationState::R,
            RotationState::R => RotationState::Two,
            RotationState::Two => RotationState::L,
            RotationState::L => RotationState::Zero,
        }
    }

    fn next_ccw(self) -> Self {
        match self {
            RotationState::Zero => RotationState::L,
            RotationState::L => RotationState::Two,
            RotationState::Two => RotationState::R,
            RotationState::R => RotationState::Zero,
        }
    }
}

// SRS Wall Kick Data for J, L, S, T, Z pieces
// Format: [test1, test2, test3, test4, test5] where each test is (x_offset, y_offset)
const JLSTZ_KICKS: [[[(i32, i32); 5]; 4]; 4] = [
    // From 0 (spawn)
    [
        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],  // 0->R
        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],     // 0->L
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
    // From R
    [
        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],      // R->0
        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],      // R->2
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
    // From 2
    [
        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],     // 2->L
        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],  // 2->R
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
    // From L
    [
        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],   // L->0
        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],   // L->2
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
];

// SRS Wall Kick Data for I piece
const I_KICKS: [[[(i32, i32); 5]; 4]; 4] = [
    // From 0 (spawn)
    [
        [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],    // 0->R
        [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],    // 0->L
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
    // From R
    [
        [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],    // R->0
        [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],    // R->2
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
    // From 2
    [
        [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],    // 2->L
        [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],    // 2->R
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
    // From L
    [
        [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],    // L->0
        [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],    // L->2
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
        [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],       // Unused
    ],
];

// I piece uses 4x4 grid - 4 rotations
const I_ROTATIONS: [[[u8; 4]; 4]; 4] = [
    // Rotation 0 - Horizontal
    [
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    // Rotation 1 - Vertical
    [
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
    ],
    // Rotation 2 - Horizontal
    [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
    ],
    // Rotation 3 - Vertical
    [
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
    ],
];

// O piece - doesn't rotate
const O_SHAPE: [[u8; 2]; 2] = [
    [2, 2],
    [2, 2],
];

// T piece rotations
const T_ROTATIONS: [[[u8; 3]; 3]; 4] = [
    // Rotation 0
    [
        [0, 3, 0],
        [3, 3, 3],
        [0, 0, 0],
    ],
    // Rotation 1
    [
        [0, 3, 0],
        [0, 3, 3],
        [0, 3, 0],
    ],
    // Rotation 2
    [
        [0, 0, 0],
        [3, 3, 3],
        [0, 3, 0],
    ],
    // Rotation 3
    [
        [0, 3, 0],
        [3, 3, 0],
        [0, 3, 0],
    ],
];

// S piece rotations
const S_ROTATIONS: [[[u8; 3]; 3]; 4] = [
    // Rotation 0
    [
        [0, 4, 4],
        [4, 4, 0],
        [0, 0, 0],
    ],
    // Rotation 1
    [
        [0, 4, 0],
        [0, 4, 4],
        [0, 0, 4],
    ],
    // Rotation 2
    [
        [0, 0, 0],
        [0, 4, 4],
        [4, 4, 0],
    ],
    // Rotation 3
    [
        [4, 0, 0],
        [4, 4, 0],
        [0, 4, 0],
    ],
];

// Z piece rotations
const Z_ROTATIONS: [[[u8; 3]; 3]; 4] = [
    // Rotation 0
    [
        [5, 5, 0],
        [0, 5, 5],
        [0, 0, 0],
    ],
    // Rotation 1
    [
        [0, 0, 5],
        [0, 5, 5],
        [0, 5, 0],
    ],
    // Rotation 2
    [
        [0, 0, 0],
        [5, 5, 0],
        [0, 5, 5],
    ],
    // Rotation 3
    [
        [0, 5, 0],
        [5, 5, 0],
        [5, 0, 0],
    ],
];

// J piece rotations
const J_ROTATIONS: [[[u8; 3]; 3]; 4] = [
    // Rotation 0
    [
        [6, 0, 0],
        [6, 6, 6],
        [0, 0, 0],
    ],
    // Rotation 1
    [
        [0, 6, 6],
        [0, 6, 0],
        [0, 6, 0],
    ],
    // Rotation 2
    [
        [0, 0, 0],
        [6, 6, 6],
        [0, 0, 6],
    ],
    // Rotation 3
    [
        [0, 6, 0],
        [0, 6, 0],
        [6, 6, 0],
    ],
];

// L piece rotations
const L_ROTATIONS: [[[u8; 3]; 3]; 4] = [
    // Rotation 0
    [
        [0, 0, 7],
        [7, 7, 7],
        [0, 0, 0],
    ],
    // Rotation 1
    [
        [0, 7, 0],
        [0, 7, 0],
        [0, 7, 7],
    ],
    // Rotation 2
    [
        [0, 0, 0],
        [7, 7, 7],
        [7, 0, 0],
    ],
    // Rotation 3
    [
        [7, 7, 0],
        [0, 7, 0],
        [0, 7, 0],
    ],
];

#[derive(Clone)]
enum PieceShape {
    Large([[[u8; 4]; 4]; 4]),  // For I piece - 4 rotations
    Small([[[u8; 3]; 3]; 4]),  // For other pieces - 4 rotations
    Mini([[u8; 2]; 2]),         // For O piece - no rotation
}

#[derive(Clone)]
pub struct Piece {
    name: String,
    color: Color32,
    rotation: RotationState,
    id: u8,
    xpos: i32,
    ypos: i32,
    shape: PieceShape,
}

impl Piece {
    pub fn new_large(name: &str, color: Color32, id: u8, shape: [[[u8; 4]; 4]; 4]) -> Self {
        Self {
            name: name.to_string(),
            color,
            rotation: RotationState::Zero,
            id,
            xpos: 3,
            ypos: 0,
            shape: PieceShape::Large(shape),
        }
    }

    pub fn new_small(name: &str, color: Color32, id: u8, shape: [[[u8; 3]; 3]; 4]) -> Self {
        Self {
            name: name.to_string(),
            color,
            rotation: RotationState::Zero,
            id,
            xpos: 3,
            ypos: 0,
            shape: PieceShape::Small(shape),
        }
    }

    pub fn new_mini(name: &str, color: Color32, id: u8, shape: [[u8; 2]; 2]) -> Self {
        Self {
            name: name.to_string(),
            color,
            rotation: RotationState::Zero,
            id,
            xpos: 4,
            ypos: 0,
            shape: PieceShape::Mini(shape),
        }
    }

    pub fn create_piece(name: &str) -> Self {
        match name {
            "I" => Self::new_large("I", Color32::from_rgb(0, 255, 255), 1, I_ROTATIONS),
            "O" => Self::new_mini("O", Color32::from_rgb(255, 255, 0), 2, O_SHAPE),
            "T" => Self::new_small("T", Color32::from_rgb(128, 0, 128), 3, T_ROTATIONS),
            "S" => Self::new_small("S", Color32::from_rgb(0, 255, 0), 4, S_ROTATIONS),
            "Z" => Self::new_small("Z", Color32::from_rgb(255, 0, 0), 5, Z_ROTATIONS),
            "J" => Self::new_small("J", Color32::from_rgb(0, 0, 255), 6, J_ROTATIONS),
            "L" => Self::new_small("L", Color32::from_rgb(255, 165, 0), 7, L_ROTATIONS),
            _ => Self::new_small("T", Color32::from_rgb(128, 0, 128), 3, T_ROTATIONS),
        }
    }

    pub fn get_blocks(&self) -> Vec<(i32, i32)> {
        let mut blocks = Vec::new();
        
        match &self.shape {
            PieceShape::Large(rotations) => {
                let shape = &rotations[self.rotation.to_index()];
                for (row, line) in shape.iter().enumerate() {
                    for (col, &cell) in line.iter().enumerate() {
                        if cell == self.id {
                            blocks.push((self.ypos + row as i32, self.xpos + col as i32));
                        }
                    }
                }
            }
            PieceShape::Small(rotations) => {
                let shape = &rotations[self.rotation.to_index()];
                for (row, line) in shape.iter().enumerate() {
                    for (col, &cell) in line.iter().enumerate() {
                        if cell == self.id {
                            blocks.push((self.ypos + row as i32, self.xpos + col as i32));
                        }
                    }
                }
            }
            PieceShape::Mini(shape) => {
                // O piece doesn't rotate
                for (row, line) in shape.iter().enumerate() {
                    for (col, &cell) in line.iter().enumerate() {
                        if cell == self.id {
                            blocks.push((self.ypos + row as i32, self.xpos + col as i32));
                        }
                    }
                }
            }
        }
        
        blocks
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(&self) -> Color32 {
        self.color
    }

    pub fn move_left(&mut self) {
        self.xpos -= 1;
    }

    pub fn move_right(&mut self) {
        self.xpos += 1;
    }

    pub fn move_down(&mut self) {
        self.ypos += 1;
    }

    pub fn move_up(&mut self) {
        self.ypos -= 1;
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotation = self.rotation.next_cw();
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.rotation = self.rotation.next_ccw();
    }

    pub fn rotate_180(&mut self) {
        self.rotation = self.rotation.next_cw().next_cw();
    }

    pub fn back_to_start_position(&mut self) {
        self.rotation = RotationState::Zero;
    }
    
    /// Try to rotate clockwise with SRS wall kicks
    /// Returns a list of (x_offset, y_offset) kick positions to try, in order
    pub fn get_cw_kick_offsets(&self) -> Vec<(i32, i32)> {
        let from_state = self.rotation;
        let to_state = from_state.next_cw();
        self.get_kick_offsets(from_state, to_state)
    }
    
    /// Try to rotate counterclockwise with SRS wall kicks
    /// Returns a list of (x_offset, y_offset) kick positions to try, in order
    pub fn get_ccw_kick_offsets(&self) -> Vec<(i32, i32)> {
        let from_state = self.rotation;
        let to_state = from_state.next_ccw();
        self.get_kick_offsets(from_state, to_state)
    }
    
    fn get_kick_offsets(&self, from: RotationState, to: RotationState) -> Vec<(i32, i32)> {
        // O piece doesn't rotate, no kicks needed
        if matches!(self.shape, PieceShape::Mini(_)) {
            return vec![(0, 0)];
        }
        
        let kick_table = if self.name == "I" {
            &I_KICKS
        } else {
            &JLSTZ_KICKS
        };
        
        let from_idx = from.to_index();
        let kick_idx = match (from, to) {
            (RotationState::Zero, RotationState::R) => 0,
            (RotationState::Zero, RotationState::L) => 1,
            (RotationState::R, RotationState::Zero) => 0,
            (RotationState::R, RotationState::Two) => 1,
            (RotationState::Two, RotationState::L) => 0,
            (RotationState::Two, RotationState::R) => 1,
            (RotationState::L, RotationState::Zero) => 0,
            (RotationState::L, RotationState::Two) => 1,
            _ => 0, // Should not happen
        };
        
        kick_table[from_idx][kick_idx].to_vec()
    }
    
    pub fn get_xpos(&self) -> i32 {
        self.xpos
    }
    
    pub fn get_ypos(&self) -> i32 {
        self.ypos
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.xpos, self.ypos)
    }
    
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.xpos = x;
        self.ypos = y;
    }

    pub fn draw_preview(&self, ui: &mut eframe::egui::Ui, cell_size: f32) {
        let blocks = self.get_blocks();
        
        if blocks.is_empty() {
            return;
        }
        
        // Find bounding box of the piece
        let min_row = blocks.iter().map(|(r, _)| *r).min().unwrap_or(0);
        let max_row = blocks.iter().map(|(r, _)| *r).max().unwrap_or(0);
        let min_col = blocks.iter().map(|(_, c)| *c).min().unwrap_or(0);
        let max_col = blocks.iter().map(|(_, c)| *c).max().unwrap_or(0);
        
        let width = (max_col - min_col + 1) as f32 * cell_size;
        let height = (max_row - min_row + 1) as f32 * cell_size;
        
        let (rect, _response) = ui.allocate_exact_size(
            eframe::egui::vec2(width, height),
            eframe::egui::Sense::hover(),
        );
        
        let painter = ui.painter();
        
        // Draw the piece blocks
        for (row, col) in &blocks {
            let local_row = (row - min_row) as f32;
            let local_col = (col - min_col) as f32;
            
            let cell_rect = eframe::egui::Rect::from_min_size(
                rect.min + eframe::egui::vec2(local_col * cell_size, local_row * cell_size),
                eframe::egui::vec2(cell_size, cell_size),
            );
            
            painter.rect_filled(cell_rect.shrink(1.0), 2.0, self.color);
        }
    }

    // Helper method to get piece color with custom transparency (useful for ghost pieces)
    pub fn get_color_with_alpha(&self, alpha: u8) -> Color32 {
        let [r, g, b, _] = self.color.to_array();
        Color32::from_rgba_unmultiplied(r, g, b, alpha)
    }
}