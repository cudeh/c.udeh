fn main() {
	let p = 210000;
	let r = 5;
	let n = 3;

	//amount
	let amount = p* ((1 + (r/100))^n);
	println!("Amount is {}", amount );