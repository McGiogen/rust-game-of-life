use rand::Rng;

const WORLD_DIMENSION: usize = 10;
const WORLD_LENGTH: usize = WORLD_DIMENSION * WORLD_DIMENSION;

fn main() {
    println!("Game of Life, Rust version!");

    let seed = generate_seed();
    println!("Seed: {}", seed);

    let mut world: Vec<bool> = build_world(WORLD_LENGTH, seed);
    print_world(&world, WORLD_DIMENSION);

    for _ in 0..10 {
        println!("--------------------");
        world = tick(&world, WORLD_DIMENSION);
        print_world(&world, WORLD_DIMENSION);
    }

    println!("Closing...");
}

fn generate_seed() -> i32 {
    return rand::thread_rng().gen::<i32>();
}

fn build_world(size: usize, seed: i32) -> Vec<bool> {
    const SEED_BITS: usize = 32;
    let mut shifted_seed: i32 = seed;
    let mut world: Vec<bool> = vec![false; size];

    for index in 0..size {
        // Prepare seed
        if index % SEED_BITS == SEED_BITS - 1 {
            // Seed restart
            shifted_seed = seed;
        } else {
            shifted_seed >>= 1;
        };

        if 1 & shifted_seed == 1 {
            world[index] = true;
        }
    }
    return world;
}

fn tick(world: &[bool], dimension: usize) -> Vec<bool> {
    let mut new_world = vec![false; dimension * dimension];
    for i in 0..WORLD_LENGTH {
        let n_alives = neighbours(&world, i).iter().filter(|&&n| n).count();
        new_world[i] = n_alives == 3 || (world[i] && n_alives == 2);
    }
    return new_world;
}

fn print_world(world: &[bool], dimension: usize) {
    for i in 0..world.len() {
        if i % dimension == 0 && i != 0 {
            println!();
        }
        print!("{}", if world[i] { "██" } else { "░░" });
    }
    println!();
}

fn neighbours(world: &[bool], index: usize) -> Vec<bool> {
    const MUTATORS: [[i32; 2]; 8] = [
        [-1, -1],
        [0, -1],
        [1, -1],
        [-1, 0],
        [1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
    ];
    let coordinates = vec![
        (index % WORLD_DIMENSION) as i32, // x
        (index / WORLD_DIMENSION) as i32, // y
    ];
    return MUTATORS
        .iter()
        .map(|&[mx, my]| vec![mx + coordinates[0], my + coordinates[1]])
        .filter(|coords| coords.iter().all(|&c| c >= 0 && c < WORLD_DIMENSION as i32))
        .map(|coords| coords[1] * WORLD_DIMENSION as i32 + coords[0])
        .map(|i| world[i as usize])
        .collect();
}
