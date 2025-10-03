use std::collections::BTreeMap;

fn main() {
    let mut map: BTreeMap<u8, &str> = BTreeMap::new();

    map.insert(1,"Number 1");
    map.insert(2,"Number 2");
    map.insert(3,"Number 2");
    map.insert(4,"Number 4");
    map.insert(5,"Number 5");

    for kvp in map.iter() {
        println!("Key: {}, Value: {}", kvp.0, kvp.1);
    }
}
