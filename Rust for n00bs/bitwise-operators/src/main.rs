fn main() {
    let int1 = 20;
    let int2 = 15;

    let and = int1 & int2;
    println!("{}", and);

    let or = int1 | int2;
    println!("{}", or); 

    let xor = int1 ^ int2;
    println!("{}", xor);

    let ls = int1 << int2;
    println!("{}", ls);

    let rs = int1 >> int2;
    println!("{}", rs);
}
