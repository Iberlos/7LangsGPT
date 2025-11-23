use rand::Rng;
//NOTE: I added the dependency rand = "0.8" to cargo toml after some research.

fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let mut grid:[[usize;WIDTH];HEIGHT]= [[0; 10]; 10];
    let boss_threat = 50;
    let tile_with: [(&str, i32); 10] = [
        ("Nothing", 0), 
        ("Slime", 1),
        ("Goblin",3),
        ("Bandit",4),
        ("Wolf",5),
        ("Weak Orc",7),
        ("Orc",8),
        ("Dark Wizard",12),
        ("Troll",15),
        ("Dragon",boss_threat)
    ];

    tile_with[0].0;

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            grid[x][y] = rand::thread_rng().gen_range(0..10);
        };
    };

    let mut threat_count = 0;
    println!("Scanning area...");
    'scan: for (l_index, line) in grid.iter().enumerate() {
        for (c_index, cell) in line.iter().enumerate() {
            print!("Scaning cell ({l_index},{c_index})...");
            let present = tile_with[*cell].0;
            let threat = tile_with[*cell].1;
            //Note: I did the alocations above because string formatting doesn;t seem to like the tipple indexing as mentioned in ex 3
            threat_count += threat;
            if threat == boss_threat {
                println!("Boss found! It is a {present}! Its threat is {threat}! Ending search!");
                break 'scan;
            }
            println!("Therre is a {present}. Its threat is {threat}.");
        }
    }
    println!("Total threat count: {threat_count}");
}