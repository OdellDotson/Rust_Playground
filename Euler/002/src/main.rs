fn main() {
	let mut prev2 = 1;
	let mut prev1 = 2;
	let mut sum = 2; //Because I'm lazy

	while prev1 + prev2 < 4000000{
		let val = prev1 + prev2;
		prev2 = prev1;
		prev1 = val;
		if val%2 == 0 {
			sum  = sum + val;
		}
	}
	println!("The sum is: {}", sum);
}
