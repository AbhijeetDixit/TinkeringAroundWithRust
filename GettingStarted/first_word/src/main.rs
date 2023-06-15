use std::io;
fn main() {
    //Get first word of a string
    let mut user_input = String::new();
    println!("Enter a string");
    io::stdin().read_line(&mut user_input).expect("Failure to read");
    let first_word = find_first_word(&user_input);
    println!("First word is : {}",first_word);
}

fn find_first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]
        }
    }
    return &s[..]
}
