fn main() {
    let int16 : u16 =65535;
    let int8 : u8 =255;

    let result:u16 = int16 / (int8 as u16);
    println!("{}",result);
}
