use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("yellow"), 50);

    //let team_name = String::from("Blue");
    //let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    //let teams = vec![String::from("Blue"), String::from("Yellow")];
    //let initial_scores = vec![10,50];

    //let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}
