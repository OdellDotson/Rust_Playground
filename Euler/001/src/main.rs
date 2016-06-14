

fn main() {
	let mut sum = 0;
	for cnt in 1..1000 {
		if cnt %3 == 0 || cnt %5 == 0 {
			sum = sum + cnt;
		}
	}
println!("The sum is: {}", sum);
}
