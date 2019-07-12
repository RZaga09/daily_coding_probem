/*
Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
*/

let result = false;
let k = (Math.floor(Math.random() * 49) + 1); //random number between 1-50
let h = (Math.floor(Math.random() * 4) + 1); //random number between 1-5 (num of values in array)
let list = []

for (let i = 0; i < h; i++) {
    list.push(Math.floor(Math.random() * 25)) //add values to list
}

for (i in list) {
    for (j in list) {
        if (list[i] + list[j] === k && i != j) {
            result = true;
        }
    }
}

console.log(k);
console.dir(list);
console.log(result);