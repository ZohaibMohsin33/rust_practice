
pub fn work_vectors () {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(34);
    v1.push(33);

    let mut v2  = vec![1,2,3,4,5];

    println!("1st {}",v2[0]);

    v2.push(6);

    let second : &i32 = &v2[1];

    match v2.get(1){
        second => println!("2nd {}",v2[1]),
        _ => println!("No line was there")
    };

    for val in &mut v2 {
        *val *= 2;
    }

    for val in &v2 {
        println!("{}",val);
    }
}