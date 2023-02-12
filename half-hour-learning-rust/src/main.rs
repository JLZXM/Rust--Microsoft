// This prints "in", then "out"
fn main() {
    use std::cmp::min;
    let least = min(7,1);
    println!("least = {}", least);
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
}