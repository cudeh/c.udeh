use std::io;

 fn main() {
    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("error");
    let name:String = input1.trim().parse().expect("invalid");

    println!("Enter email: ");
    io::stdin().read_line(&mut input2)expect("error");
    let email:String = input2.trim().parse().expect("invalid");

    println!("Department: ");
    io::stdin().read_line(&mut input3)expect("error");
    let department:String = input3.trim().parse().expect("invalid");

    println!("State of origin: ");
    io::stdin().read_line(&mut input4)expect("error");
    let state of origin:String = input4.trim().parse().expect("invalid");

    println!("Are you a class rep?: ");
    io::stdin().read_line(&mut input5)expect("error");
    let Are you a class rep:String = input5.trim().parse().expect("invalid");

    if class_rep = "yes"

    println!("Are you in 100 level?: ");
    io::stdin().read_line(&mut input6)expect("error");
    let Are you in 100 level:String = input6.trim().parse().expect("invalid");

    if department = "no"

    println!("Whats your CGPA?");
    io::stdin().read_line(&mut input7)expect("error");
    let CGPA:String = input7.trim().parse().expect("invalid");

    if CGPA = >4.0

    println!("You can vote");






}
