fn main() {
    let party = vec![
        ("Astarion", 32, 40),
        ("Shadowheart", 20, 45),
        ("Karlach", 55, 60),
        ("Wyll", 0, 50),
        ("Lae'zel", 47, 50)
    ];
    
    let mut message = String::from("");
    for character in party.iter() {
        let health_percentage = 100.0*(character.1 as f32)/(character.2 as f32);
        let character_status = match health_percentage {
            75.0 ..= 100.0 => "Healthy",
            50.0 .. 75.0 => "Hurt",
            25.0 .. 50.0 => "Wounded",
            1.0 .. 25.0 => "Critical",
            _ => "Dead"
        };
        message.push_str("{charcter.0}: character.1/character.2 ({health_percentage:.2}%) -> {character_status}\n");
        //Question: Isn't there a way to make the line above work? there is no formatting. I wanted to reduce the number of print calls to one by apending the messages.
        print!("{}: {}/{} ({:.2}%) -> {}\n", character.0, character.1, character.2, health_percentage, character_status)
        //Question: The following case seems to be invalid "{character.0}" is there no simpler way to use the tupples in the string?
    }
    
    print!("{message}");
    //Question: The above is quite dumb and unintuitive, isn't there a better way to print a String?
}