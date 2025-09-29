use std::char;


pub fn work(){
    
    let mut str1 = String::new();

    str1.push('A');
    str1.push_str(" letter");
    str1.push_str(" of");
    str1.push_str(" work");

    for word in str1.split_whitespace() {
        println!("{}", word);
    }


    let str2 = String::from("a f t e s b g r e s l i n");
    let mut v1: Vec<char> = str2.chars().collect();

    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}",char);
    }

    let str3 : &str = "Random String";
    let str4 = str3.to_string();

    println!("{}", str4);

    let byte_arr : &[u8] = str4.as_bytes();

    let st5: &str = &str4[0..6];

    println!("The string is : {} and it's length is {} ",st5, st5.len());
    

}
