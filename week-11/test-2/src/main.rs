fn main() {
    
     let v = vec!["Cadbury", "Champion", "Dangote Sugar", "Flour Mills", "Nestle", "Unilever", "Honeywell", "Nigerian Breweries"];
}

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter a username: ");
    std::io::stdin().(&mut input1).expect("Failed to read input");
    //username is not the first four letters of the vec
    let username:usize = input1.trim().parse().expect("Invalid input");

    //getting letters at given username letters
    let ch: &str = v[username];

    print!("{} is the character for username [{}]\n",ch, username);


    println!("Enter password: ", );
    i8::stdin().read_line(&mut input2).expect("Not a valid string");

    let password:&str = "letters between a-z";
    let password:&str = "numbers between 0-9";
    let password:&str = "lowercase letters";
    let password:&str = "mo chaacters from $,#,@"; 
    let password:&str = "minimum of 3";
    let password:&str = "maximum of 8"
    
    else {
        println!("Invalid password");
    };

    use std::fs::file;
    //Create a new file
    let mut file = File::create("Company information");

    use std::io::Write;

    fn main() {

        let file name = "Company Information";

        let mut file = std::fs::file::create("data.text").expect("create failed");
        file.write_all("Cadbury Nigeria Plc");
        file.write_all("founded in 1965");
        file.write_all("share of 15000000");
        file.write_all("liability of 5500000");
        file.write_all("leverage of 63%");

        file.write_all("Champion Breweries Plc");
        file.write_all("founded in 1974");
        file.write_all("share of 25000000");
        file.write_all("liability of 8000000");
        file.write_all("leverage of 68%");

        file.write_all("Dangote sugar refinery Plc");
        file.write_all("founded in 1970");
        file.write_all("share of 18000000");
        file.write_all("liability of 10000000");
        file.write_all("leverage of 44%");

        file.write_all("Flour Mills Nigeria Plc");
        file.write_all("founded in 1960");
        file.write_all("share of 32000000");
        file.write_all("liability of 4000000");
        file.write_all("leverage of 87%");

        file.write_all("Nestle Nigeria Plc");
        file.write_all("founded in 1961");
        file.write_all("share of 8000000");
        file.write_all("liability of 1500000");
        file.write_all("leverage of 81%");

        file.write_all("Unilever Nigeria Plc");
        file.write_all("founded in 1923");
        file.write_all("share of 37000000");
        file.write_all("liability of 11000000");
        file.write_all("leverage of 70%");

        file.write_all("Honeywell Nigeria Plc");
        file.write_all("founded in 1906");
        file.write_all("share of 34000000");
        file.write_all("liability of 9000000");
        file.write_all("leverage of 74%");

        file.write_all("Nigerian Breweries Plc");
        file.write_all("founded in 1946");
        file.write_all("share of 30000000");
        file.write_all("liability of 12000000");
        file.write_all("leverage of 60%");

        .as_bytes().expect("write failed");
file.write_all(file name.as_bytes()).expect("write failed");
println!("\nData written to file." );

use std::fs::file;
//Create new file
let mut file = file::create("File of company's share greater than 20,000,000");

if share > 20,000,000;{
    file.write_all("Companies leverage")
    println!("\nCompanies percentage leverages used by each company");
    as_bytes().expect("write failed");
file.write_all(file name.as_bytes()).expect("write failed");
    println!("\nData written to file." );
}

    if liability < 10000000; {
        let String = 5% * percentage leverage
        println!("\n5% of percentage leverages");
        as_bytes().expect("write failed");
file.write_all(file name.as_bytes()).expect("write failed");
    println!("\nData written to file." );

    
    
}




}

























