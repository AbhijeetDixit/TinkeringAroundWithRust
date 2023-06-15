use std::io;
fn get_user_choice() -> u32{
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("Enter a choice (1-4)");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failure to read the choice");
    let choice : u32 = choice.trim().parse().expect("Failure to parse");
    choice
}

fn read_two_values() -> (u32, u32){
    let mut tup:(u32, u32) = (0,0);
    println!("Enter first number");
    let mut num1 = String::new();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num1).expect("Failure to read number 1");
    io::stdin().read_line(&mut num2).expect("Failure to read number 2");
    tup.0 = num1.trim().parse().expect("Failure to parse number 1");
    tup.1 = num2.trim().parse().expect("Failure to parse number 2");
    tup
}

fn perform_addition(){
    //println!("I am addition");
    let tup1 = read_two_values();
    //println!("Two numbers are : {}, {}",tup1.0, tup1.1);
    println!("Sum of two numbers is : {}", tup1.0 + tup1.1);
}

fn perform_subtraction(){
    //println!("I am subtraction");
    let tup1 = read_two_values();
    //println!("Two numbers are : {}, {}",tup1.0, tup1.1);
    println!("Difference of two numbers is : {}", tup1.0 - tup1.1);
}

fn perform_multiplication(){
    //println!("I am multiplication");
    let tup1 = read_two_values();
    //println!("Two numbers are : {}, {}",tup1.0, tup1.1);
    println!("Product of two numbers is : {}", tup1.0 * tup1.1);
}

fn perform_division(){
    //println!("I am division");
    let tup1 = read_two_values();
    //println!("Two numbers are : {}, {}",tup1.0, tup1.1);
    if tup1.1 == 0{
        println!("Cannot divide by 0");
        return;
    }
    println!("Sum of two numbers is : {}", tup1.0 / tup1.1);
}

fn main() {
    println!("Welcome to a basic calculator");
    let choice = get_user_choice();
    println!("User choice is {choice}");
    if choice == 1{
        perform_addition()
    }else if choice == 2{
        perform_subtraction()
    }else if choice == 3{
        perform_multiplication()
    }else if choice == 4{
        perform_division()
    }else{
        println!("Invalid choice.")
    }
}
