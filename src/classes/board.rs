use egui::Color32;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub id: u8,  // 0 = empty, 1-7 = piece types
}

fn get_color_from_id(id: u8) -> Option<Color32> {
    match id {
        0 => None,
        1 => Some(Color32::from_rgb(0, 255, 255)),   // I - Cyan
        2 => Some(Color32::from_rgb(255, 255, 0)),   // O - Yellow
        3 => Some(Color32::from_rgb(128, 0, 128)),   // T - Purple
        4 => Some(Color32::from_rgb(0, 255, 0)),     // S - Green
        5 => Some(Color32::from_rgb(255, 0, 0)),     // Z - Red
        6 => Some(Color32::from_rgb(0, 0, 255)),     // J - Blue
        7 => Some(Color32::from_rgb(255, 165, 0)),   // L - Orange
        _ => None,
    }
}

pub struct Board {
    pub cells: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
    pub cell_size: f32,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell { id: 0 }; BOARD_WIDTH]; BOARD_HEIGHT],
            cell_size: 30.0,
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        self.draw_with_overlay(ui, &[]);
    }

    // Draw board with optional overlay blocks (for ghost pieces)
    pub fn draw_with_overlay(&self, ui: &mut egui::Ui, overlay_blocks: &[(i32, i32, Color32)]) {
        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(
                BOARD_WIDTH as f32 * self.cell_size,
                BOARD_HEIGHT as f32 * self.cell_size,
            ),
            egui::Sense::hover(),
        );

        let painter = ui.painter();

        // Draw grid background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 30));

        // Draw cells
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let cell_rect = egui::Rect::from_min_size(
                    rect.min
                        + egui::vec2(
                            col as f32 * self.cell_size,
                            row as f32 * self.cell_size,
                        ),
                    egui::vec2(self.cell_size, self.cell_size),
                );

                // Draw cell
                if let Some(color) = get_color_from_id(self.cells[row][col].id) {
                    painter.rect_filled(cell_rect.shrink(1.0), 2.0, color);
                }

                // Draw grid lines
                painter.rect_stroke(
                    cell_rect,
                    0.0,
                    egui::Stroke::new(1.0, Color32::from_rgb(40, 40, 50)),
                );
            }
        }

        // Draw overlay blocks (ghost pieces, etc.)
        for (row, col, color) in overlay_blocks {
            if *row >= 0 && *col >= 0 && (*row as usize) < BOARD_HEIGHT && (*col as usize) < BOARD_WIDTH {
                let cell_rect = egui::Rect::from_min_size(
                    rect.min
                        + egui::vec2(
                            *col as f32 * self.cell_size,
                            *row as f32 * self.cell_size,
                        ),
                    egui::vec2(self.cell_size, self.cell_size),
                );
                painter.rect_filled(cell_rect.shrink(1.0), 2.0, *color);
            }
        }

        // Draw border
        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(2.0, Color32::from_rgb(100, 100, 120)),
        );
    }

    pub fn set_cell(&mut self, row: usize, col: usize, id: u8) {
        if row < BOARD_HEIGHT && col < BOARD_WIDTH {
            self.cells[row][col].id = id;
        }
    }
    
    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        if row < BOARD_HEIGHT && col < BOARD_WIDTH {
            self.cells[row][col].id
        } else {
            0
        }
    }

    pub fn get_width() -> usize { BOARD_WIDTH }
    pub fn get_height() -> usize { BOARD_HEIGHT }

    pub fn is_valid_position(&self, blocks: &[(i32, i32)]) -> bool {
        blocks.iter().all(|(row, col)| {
            *row >= 0 && *row < BOARD_HEIGHT as i32 
            && *col >= 0 && *col < BOARD_WIDTH as i32
            && self.get_cell(*row as usize, *col as usize) == 0
        })
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut row = BOARD_HEIGHT - 1; // Start from bottom
        
        loop {
            // Check if this row is full
            if self.is_line_full(row) {
                // Remove this row
                self.remove_line(row);
                lines_cleared += 1;
                // Don't increment row, check same position again
            } else {
                // Move to next row up
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }
        
        lines_cleared
    }
    
    fn is_line_full(&self, row: usize) -> bool {
        self.cells[row].iter().all(|cell| cell.id != 0)
    }
    
    fn remove_line(&mut self, row: usize) {
        // Shift all rows above down by one
        for r in (1..=row).rev() {
            self.cells[r] = self.cells[r - 1];
        }
        // Add empty row at top
        self.cells[0] = [Cell { id: 0 }; BOARD_WIDTH];
    }
}