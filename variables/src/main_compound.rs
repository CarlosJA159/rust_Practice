fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //A tuple is a general way of grouping together a number of 
    //values with a variety of types into one compound type. Tuples have a fixed length: 
    //Once declared, they cannot grow or shrink in size.

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

}