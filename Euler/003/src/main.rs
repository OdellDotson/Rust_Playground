fn main() {
	let mut factoring: i64 =  600851475143;
	let mut largestFactor: i64 = 0;
	let mut currFacTest: i64 = 2;

	while factoring > 2 {
		while factoring%currFacTest == 0 {
			largestFactor = currFacTest;
			factoring = factoring / currFacTest;
		}
	currFacTest = currFacTest + 1;
	}
	println!("{}", largestFactor);
}
