pub mod grid;

pub(crate) fn main() {
    // Step 1: Initialize the grid
    let mut grid = init_grid();

    // Step 2: Spawn two initial tiles
    spawn_tile(&mut grid);
    spawn_tile(&mut grid);

    // Step 3: Print the grid to check the initialization
    print_grid(&grid);
}

