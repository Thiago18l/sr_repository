
pub fn ownership() {
    let mut s = String::from("Hello");
    s.push_str(", world from ownership");
    println!("{}", s);

    // Ways that variables and data interact: move
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
    //String version
    let s1 = String::from("Maybe");
    let s2 = s1;
    println!("{}", s2);
    // Ways that variables and data interact: clone
    let string_1 = String::from("Whatever");
    let string_2 = string_1.clone();
    println!("string 1 = {}, string 2 = {}", string_1, string_2);


    let string_value_one = gives_ownership();
    let string_value = String::from("Hello");
    let s3 = takes_and_gives_back(string_value);
    println!("{}, {}", s3, string_value_one);
    intern_ownership(s2);
    make_copy(x);
}


// Ownership and functions
fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(value_string: String) -> String {
    value_string
}


fn intern_ownership(value: String) {
    println!("{}", value);
}

fn make_copy(value: i32) {
    println!("{}", value);
}