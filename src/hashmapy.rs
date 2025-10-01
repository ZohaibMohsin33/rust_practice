use std::collections::HashMap;



pub fn hasmap(){

    println!("Hello World");

    let mut heroes = HashMap::new();

    heroes.insert("Batman", "Bruce Waine");
    heroes.insert("Spiderman", "Peter Parker");

    for (k,v) in heroes.iter(){
        println!("The key is {} and the value is {}", k , v);
    }

    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Btman");
        match the_batman {
            Some(x) => print!("Batman is a hero"),
            None => print!("Batman is not a hero")
        };

    }
}