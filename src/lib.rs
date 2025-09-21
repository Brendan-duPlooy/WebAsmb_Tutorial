use wasm_bindgen::prelude::*;
use web_sys::console;

// Macro for logging to browser console
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// Import the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, which alerts a hello message
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! Welcome to WebAssembly with Rust!", name));
}

// Define Cell enum with explicit memory representation
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

// Main Universe struct representing the Game of Life world
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// Methods callable from JavaScript
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn set_pattern(&mut self, pattern: &str, start_row: u32, start_col: u32) {
        match pattern {
            "glider" => {
                let pattern = [
                    (0, 1), (1, 2), (2, 0), (2, 1), (2, 2)
                ];
                self.clear_area(start_row, start_col, 3, 3);
                for &(row, col) in &pattern {
                    let r = (start_row + row) % self.height;
                    let c = (start_col + col) % self.width;
                    let idx = self.get_index(r, c);
                    self.cells[idx] = Cell::Alive;
                }
            }
            "pulsar" => {
                let pattern = [
                    // Top part
                    (2, 4), (2, 5), (2, 6), (2, 10), (2, 11), (2, 12),
                    (4, 2), (4, 7), (4, 9), (4, 14),
                    (5, 2), (5, 7), (5, 9), (5, 14),
                    (6, 2), (6, 7), (6, 9), (6, 14),
                    (7, 4), (7, 5), (7, 6), (7, 10), (7, 11), (7, 12),
                    // Bottom part
                    (9, 4), (9, 5), (9, 6), (9, 10), (9, 11), (9, 12),
                    (10, 2), (10, 7), (10, 9), (10, 14),
                    (11, 2), (11, 7), (11, 9), (11, 14),
                    (12, 2), (12, 7), (12, 9), (12, 14),
                    (14, 4), (14, 5), (14, 6), (14, 10), (14, 11), (14, 12),
                ];
                self.clear_area(start_row, start_col, 17, 17);
                for &(row, col) in &pattern {
                    let r = (start_row + row) % self.height;
                    let c = (start_col + col) % self.width;
                    let idx = self.get_index(r, c);
                    self.cells[idx] = Cell::Alive;
                }
            }
            "gosper_glider_gun" => {
                let pattern = [
                    (1, 25),
                    (2, 23), (2, 25),
                    (3, 13), (3, 14), (3, 21), (3, 22), (3, 35), (3, 36),
                    (4, 12), (4, 16), (4, 21), (4, 22), (4, 35), (4, 36),
                    (5, 1), (5, 2), (5, 11), (5, 17), (5, 21), (5, 22),
                    (6, 1), (6, 2), (6, 11), (6, 15), (6, 17), (6, 18), (6, 23), (6, 25),
                    (7, 11), (7, 17), (7, 25),
                    (8, 12), (8, 16),
                    (9, 13), (9, 14),
                ];
                self.clear_area(start_row, start_col, 11, 38);
                for &(row, col) in &pattern {
                    let r = (start_row + row) % self.height;
                    let c = (start_col + col) % self.width;
                    let idx = self.get_index(r, c);
                    self.cells[idx] = Cell::Alive;
                }
            }
            _ => log!("Unknown pattern: {}", pattern),
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
    }

    pub fn randomize(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|_| {
                if js_sys::Math::random() < 0.3 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Private methods (not exported to JavaScript)
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        
        // Use modular arithmetic to handle wrapping at edges
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn clear_area(&mut self, start_row: u32, start_col: u32, height: u32, width: u32) {
        for row in 0..height {
            for col in 0..width {
                let r = (start_row + row) % self.height;
                let c = (start_col + col) % self.width;
                let idx = self.get_index(r, c);
                self.cells[idx] = Cell::Dead;
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// Performance timing utilities
pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

// Utility functions for better debugging
pub mod utils {
    use wasm_bindgen::prelude::*;

    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    pub fn set_panic_hook() {
        console_error_panic_hook::set_once();
    }

    #[cfg(not(feature = "console_error_panic_hook"))]
    pub fn set_panic_hook() {}
}