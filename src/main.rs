fn main() {
    println!("Game of Life, Rust version!");

    const WORLD_DIMENSION: usize = 30;
    
    let mut _world: [i32; WORLD_DIMENSION*WORLD_DIMENSION] = [0; WORLD_DIMENSION*WORLD_DIMENSION];
    printWorld(_world, WORLD_DIMENSION);

    println!("Closing...");
}

fn printWorld(world: [i32; 30*30], dimension: usize) {
    for x in 0..dimension {
        for y in 0..dimension {
            print!("{} ", world[x*dimension+y]);
        }
        println!();
    }
}
