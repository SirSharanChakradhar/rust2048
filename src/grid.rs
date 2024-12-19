use rand::prelude::*; // For random number generation

// Define the size of the grid
const GRID_SIZE: usize = 4;

// Type alias for a 2D grid
type Grid = [[u32; GRID_SIZE]; GRID_SIZE];

// Function to initialize the grid with zeros
fn init_grid() -> Grid {
    [[0; GRID_SIZE]; GRID_SIZE]
}

// Function to spawn a random tile (2 or 4) in an empty cell
fn spawn_tile(grid: &mut Grid) {
    // Find all empty cells
    let mut empty_cells = vec![];
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if grid[row][col] == 0 {
                empty_cells.push((row, col));
            }
        }
    }

    // If no empty cells, do nothing
    if empty_cells.is_empty() {
        return;
    }

    // Choose a random empty cell
    let mut rng = thread_rng();
    let (row, col) = empty_cells[rng.gen_range(0..empty_cells.len())];

    // Assign a 2 or 4 to the chosen cell
    grid[row][col] = if rng.gen_bool(0.9) { 2 } else { 4 };
}

// Function to print the grid
fn print_grid(grid: &Grid) {
    for row in grid {
        for &cell in row {
            if cell == 0 {
                print!(" . "); // Empty cell
            } else {
                print!("{:2} ", cell); // Tile value
            }
        }
        println!(); // Newline for each row
    }
    println!(); // Extra newline for spacing
}

