fn main() {
    let mut stamina: i32 = 32;

    loop {
        if stamina >= 20 {
            println!("Sprinting!");
            stamina -= 20;
        }
        else
        {
            println!("Too tired to sprint!");
            break;
        }
    }
}