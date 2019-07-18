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
	let mut i = 1;

	loop {
		if !example1.contains(&i) {
			println!("Example 1 -> {}", i); //expect 2
			break;
		} else {
			i += 1;
		}
	}

	let mut i = 1;
	loop {
		if !example2.contains(&i) {
			println!("Example 2 -> {}", i); //expect 3
			break;
		} else {
			i += 1;
		}
	}

	let mut i = 1;
	loop {
		if !example3.contains(&i) {
			println!("Example 3 -> {}", i); //expect 1
			break;
		} else {
			i += 1;
		}
	}
}
