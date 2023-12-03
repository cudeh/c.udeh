
use std::io;



 fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();


    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("error");
    let name:String = input1.trim().parse().expect("invalid");

    println!("Enter email: ");
    io::stdin().read_line(&mut input2).expect("error");
    let email:String = input2.trim().parse().expect("invalid");

    println!("Department: ");
    io::stdin().read_line(&mut input3).expect("error");
    let department:String = input3.trim().parse().expect("invalid");

    println!("State of origin: ");
    io::stdin().read_line(&mut input4).expect("error");
    let state_of_origin:String = input4.trim().parse().expect("invalid");

    println!("Are you a class rep 'Course rep' or 'Not Course Rep' ");
    io::stdin().read_line(&mut input5).expect("error");
    let are_you_a_class_rep:String = input5.trim().parse().expect("invalid");


    println!("What level are you in?: ");
    io::stdin().read_line(&mut input6).expect("error");
    let are_you_are_level:f32 = input6.trim().parse().expect("invalid");


    println!("Whats your CGPA?");
    io::stdin().read_line(&mut input7).expect("error");
    let cgpa:f64 = input7.trim().parse().expect("invalid");



//Validation of the eligibility to vote

if cgpa >= 4.0 && are_you_a_class_rep == "Course Rep" && are_you_are_level >= 100.0 {
println!("You Are Qualified to vote");
println!("{}",name);
println!("{}",email);
println!("{}",department);
println!("{}",state_of_origin);

} else{
    println!("You are not qualified to vote");





 }





}
