/*
Given an array of integers, find the first missing positive integer in linear time and constant space.
In other words, find the lowest positive integer that does not exist in the array.
The array can contain duplicates and negative numbers as well.
For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] should give 3.
You can modify the input array in-place.
*/

fn main() {
	let mut example1: [i32; 4] = [3, 4, -1, 1];
	let mut example2: [i32; 3] = [1, 2, 0];
	let mut example3: [i32; 5] = [-2, -1, -4, -5, 0];
	println!("Example1 = {:?}", example1);
	println!("Example2 = {:?}", example2);
	println!("Example3 = {:?}", example3);
	println!("");
	println!("OUTPUTS");
	example1.sort();
	example2.sort();
	example3.sort();

	//example1
	for i in 0..4 {
		if example1[i] > 0 {
			let mut j = example1[i] + 1;
			loop {
			if !example1.contains(&j) {
				println!("Example 1 -> {}", j);
				break;
			} else {
				j += 1;
			}
		}
			break;
		} else if i == 3 {
			println!("Example 1: -> 1");
		}
	}

	//example2
	for i in 0..3 {
		if example2[i] > 0 {
			let mut j = example2[i] + 1;
			loop {
			if !example2.contains(&j) {
				println!("Example 2 -> {}", j);
				break;
			} else {
				j += 1;
			}
		}
			break;
		} else if i == 2 {
			println!("Example 2 -> 1");
		}
	}

	//example3
	for i in 0..5 {
		if example3[i] > 0 {
			let mut j = example2[i] + 1;
			loop {
			if !example3.contains(&j) {
				println!("Example 3 -> {}", j);
				break;
			} else {
				j += 1;
			}
		}
			break;
		} else if i == 4 {
			println!("Example 3 -> 1");
		}

	}
}