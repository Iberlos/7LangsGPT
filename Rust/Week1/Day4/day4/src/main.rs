fn main() {
    //Task 1 - simple variables and branching
    let health: i32 = 32;
    let max_health: i32 = 40;

    let percent = health as f32 / max_health as f32 * 100.0;

    print!("Health: {health}/{max_health}({percent}%)\nStatus: ");
    if percent >= 50.0 {
        println!("Healthy")
    }
    else
    {
        println!("Wounded")
    }

    //Task 2 - match instead of if/else
    let status = match percent {
        0.0 => "Dead",
        0.01..=0.25 => "Critical",
        0.25..0.5 => "Injured",
        _ => "Healthy"
    };
    println!("Health: {health}/{max_health}({percent}%)\nStatus: {status}");

    //Task 3 - loop
    println!("Regenerating health...");
    let mut regen_health = health;
    loop {
        regen_health += 1;
        println!("Current health: {regen_health}");
        if regen_health == max_health {break};
    }

    //Task 4 - while
    let mut countdown: i32 = 5;
    while countdown >= 0 {
        match countdown {
            0 => println!("Liftoff!"),
            _ => println!("{countdown}!")
        }
        countdown -= 1;
    }

    //Task 5 - for
    println!("For Loop:");
    let min_range: i32 = 0;
    let max_range: i32 = 10;
    for i in min_range..=max_range {
        println!("Step: {i}");
        if i == max_range {println!("Done Itterating!")};
    }

    //Task 6 - for with borrowing
    let mut players = vec!["Astarion", "Shadowheart", "Karlach", "Wyll", "Lae'zel"];
    println!("Player List:");
    for name in players.iter() {
        println!("Player: {name}");
    }
    //Added the logic below to justify the use of a vector because clippy pointed out an array would have sufficed otherwise
    players.push("Iberlos");
    println!("A new player joined!\nPlayer List:");
    for name in players.iter() {
        println!("Player: {name}");
    }

    //Task 7 - itter with enumerate
    println!("Indexed Player List:");
    for (index, name) in players.iter().enumerate() {
        println!("{index}: {name}");
    }

    //task 8 - nested for loops with lable breaking
    let enemies = ["Goblin", "Orc", "Troll", "Dragon"];
    'search: for enemy in enemies.iter() {
        for segment in 1..=5 {
            println!("Checking ({enemy},{segment})...");
            if *enemy == "Orc" && segment == 3 {
                println!("Found critical enemy segment - stopping search!");
                break 'search;
            }
        }
    }
    println!("Search finished");
}
