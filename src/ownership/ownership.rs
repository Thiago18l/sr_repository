
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

    let (v1, len) = calculate_length(s3);

    let string_v2 = String::from("Version2");
    let len_v2 = calculate_length_v2(&string_v2);

    println!("{}, {}", string_v2, len_v2);
    println!("The length of '{}' is {}", v1, len);
    intern_ownership(s2);
    make_copy(x);


    let mut s4 = String::from("Hello");
    println!("{}", mutable_ref(&mut s4));


    // First word
    let word = String::from("Thiago Lopes");
    let result_w = first_word(&word);
    println!("{}", result_w);
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

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_v2(s: &String) -> usize {
    s.len()
}

fn mutable_ref(some_string: &mut String) -> &String {
    some_string.push_str(", sir");
    some_string
}

// First Word

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}