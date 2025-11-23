fn main() {
    let carry_weight: f32 = 57.5;
    let max_weight: f32 = 100.0;

    let percent_full: f32 = carry_weight/max_weight*100.0;
    let ranges = [0.0 .. 25.0, 25.0 .. 50.0, 50.0 .. 75.0, 75.0 .. 100.0 ];
    let state = if ranges[0].contains(&percent_full) {
        "Unburdened"
    } else if ranges[1].contains(&percent_full) {
        "Light Load"
    } else if ranges[2].contains(&percent_full) {
        "Encumbered"
    } else if ranges[3].contains(&percent_full) {
        "Overburdened"
    } else {
        "You cannot move! Drop some items immediately!"
    };

    //I wasn't able to use the ranges variables I created in the match. Can you explain why?
    /*let state = match percent_full {
        75.0 ..= 100.0 => "Overburdened",
        50.0 .. 75.0 => "Encumbered",
        25.0 .. 50.0 => "Light Load",
        0.0 .. 25.0 => "Unburdened",
        _ => "You cannot move! Drop some items immediately!"
    };*/

    println!("Carry Weight: {carry_weight:.2}/{max_weight:.2}({percent_full:.2}%)\nState: {state}");
}