fn main() {
    let a = [1, 2, 3, 4, 5];


//Another way to have a collection of multiple values is with an array. 
//Unlike a tuple, every element of an array must have the same type. 
//Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

//Ex of a array you would need to leave fixed

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; //let a = [3, 3, 3, 3, 3];

//
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}