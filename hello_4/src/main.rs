
fn main() {
	let mut inc: int = 1;

	print_number(inc);
	print_sum(12, inc);

	inc_one(&mut inc);

	print_number(inc);
	print_sum(12, inc);
}

fn print_number(x: int) {
	println!("Number is: {}", x);
}

fn print_sum(x: int, y:int) {
	println!("Sum of arguments is: {}", x + y);
}

fn inc_one(x: &mut int) {
	*x += 1;
}
