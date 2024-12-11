mod a;

// This Tuple struct has a private inner field
pub struct TupleStruct(String);

pub fn the_example() {
    let TupleStruct(s) = a::run();
    println!("result:  {s}");
}

