/*
Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
*/

extern crate rand;
use rand::Rng;

fn main() {
    let k = rand::thread_rng().gen_range(1, 51); //random number from 1-50
    let h = rand::thread_rng().gen_range(2, 6); //array length 2-5
    let mut array: Vec<u8> = vec![];
    for i in 0..h { 
        let a = rand::thread_rng().gen_range(1, 26); //random values from 1-25
        array.push(a);
    }

    println!("{}", k);
    println!("{:?}", array);

    let mut c = 0;
    for i in 0..h {
        for j in 0..h {
            if array[i] + array[j] == k && i != j && i < j {
                println!("true");
                c += 1;
            } else if i == h - 1 && j == h - 1 {
                if c == 0 {
                    println!("false");
                }
            }
        }
    }
}