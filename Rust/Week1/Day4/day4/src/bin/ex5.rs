use std::{any::{Any, TypeId, type_name, type_name_of_val}, collections::HashMap};

    const BUFF_NAMES: [&str;2] = ["Shield", "Giant Stength"];
    const DEBUFF_NAMES: [&str;3] = ["Paralized", "Frozen", "Burning"];

enum Event<'a> {
    Damage {origin: &'a str, target: &'a str, amount: i32},
    Heal   {origin: &'a str, target: &'a str, amount: i32},
    Buff   {origin: &'a str, target: &'a str, name: &'a str},
    Debuff {origin: &'a str, target: &'a str, name: &'a str},
    Death  {origin: &'a str, target: &'a str, message: &'a str}
}

fn simulate_battle<'a>(
    party: &mut HashMap<&str, (i32, i32, &'a str)>,
    enemy_party: &mut HashMap<&str, (i32, i32, &'a str)>,
    logs: &mut Vec<Event<'a>>,
) {
    if let Some(stats) = party.get_mut("Astarion") {
        stats.0 += 1;
    }

    logs.push(Event::Heal {
        origin: "Shadowheart",
        target: "Astarion",
        amount: 1,
    });

    if let Some(stats) = party.get_mut("Astarion") {
        stats.2 = BUFF_NAMES[0];
    }

    logs.push(Event::Buff {
        origin: "Shadowheart",
        target: "Astarion",
        name: BUFF_NAMES[0],
    });
}


fn print_logs(logs:&Vec<Event<'_>>, stop_at_first_death: bool) {
    for event in logs.iter() {
        match event {
            Event::Damage {origin, target, amount } => {
                println!("{origin} dealt {amount} damage to {target}!");
            }
            Event::Heal   {origin, target, amount } => {
                println!("{origin} healed {amount} health points on {target}!");
            }
            Event::Buff   {origin, target, name } => {
                println!("{origin} buffed {target} with {name}!");
            }
            Event::Debuff {origin, target, name } => {
                println!("{origin} caused {name} on {target}!");
            }
            Event::Death  {origin, target, message } => {
                println!("{origin} has killed {target}! {target}: \"{message}\"");

                if stop_at_first_death {
                    println!("Logs stopped prematurely!");
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut party = HashMap::from([
        ("Astarion",    (32, 40, "")),
        ("Shadowheart", (20, 45, "")),
        ("Karlach",     (55, 60, "")),
        ("Wyll",        (20, 50, "")),
        ("Lae'zel",     (47, 50, ""))
    ]);

    let mut enemy_party = HashMap::from([
        ("Goblin",     (40, 40, "")),
        ("Evil Druid", (45, 45, "")),
        ("Bear",       (80, 80, "")),
        ("Beholder",   (100, 100, "")),
        ("Wolf",       (47, 50, ""))
    ]);



    let mut log = vec![
        Event::Damage {origin: "Goblin", target: "Astarion", amount: 12 },
        Event::Heal   {origin: "Shadowheart", target: "Shadowheart", amount: 6 },
        Event::Buff   {origin: "Karlach", target: "Karlach", name: "Rage" },
        Event::Damage {origin: "Goblin", target: "Wyll", amount: 55 },
        Event::Debuff {origin: "Goblin", target: "Lae'zel", name: "Poison" },
        Event::Damage {origin: "Goblin", target: "Astarion", amount: 30 },
        Event::Death  {origin: "Goblin", target: "Astarion", message: "Astarion Death"}
    ];

    simulate_battle(&mut party, &mut enemy_party, &mut log);

    print_logs(&log, false);
}