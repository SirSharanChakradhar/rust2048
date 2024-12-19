 // Initialize a 4x4 grid of zeros using Vec
fn grid()->  Vec<Vec<u64>> {

   vec! [
    vec![0 , 0 , 0 , 0 ],
    vec![0 , 0 , 0 , 0 ],
    vec![0 , 0 , 0 , 0 ],
    vec![0 , 0 , 0 , 0 ]
   ]
}

fn print_grid (grid: &Vec<Vec<u64>>) {

   for row in grid {
      for &cell in row {
         print!("{:2} ",cell);
      }
      println!(); 
   }
   println!();
}


fn main() {
   let mut my_grid = grid();
     
   my_grid[0][0] = 2;
   my_grid[1][3] = 2;
   my_grid[3][3] = 2;

   print_grid(&my_grid);

   }
   