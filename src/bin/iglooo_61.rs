use std::collections::HashMap;

fn main() {
    let mut somemap: HashMap<&str, usize> = HashMap::new();
    somemap.insert("apple", 10);

    println!("map : {:?}", somemap);

    somemap.entry("apple").insert_entry(20);
    println!("map : {:?}", somemap);

    somemap.entry("apple").and_modify(|value| *value += 1);
    println!("map : {:?}", somemap);

    somemap.entry("banana").or_insert_with(|| 30);
    println!("map : {:?}", somemap);

    somemap.entry("mango").or_default();
    println!("map : {:?}", somemap);

    let mut hashmap: HashMap<&str, usize> = HashMap::new();
    hashmap
        .entry("key1")
        .and_modify(|value| *value += 1)
        .or_insert(10);
    println!("{:?}", hashmap);

    hashmap
        .entry("key1")
        .and_modify(|value| *value += 1)
        .or_insert(10);
    println!("{:?}", hashmap);

    let mut onemorehashmap: HashMap<&str, Vec<i32>> = HashMap::new();
    onemorehashmap
        .entry("first_vec")
        .and_modify(|value| value.iter_mut().map(|x| *x += 1).collect())
        .or_insert_with(|| vec![1, 2, 3]);
    println!("{:?}", onemorehashmap);

    onemorehashmap
        .entry("first_vec")
        .and_modify(|value| value.iter_mut().map(|x| *x += 1).collect())
        .or_insert_with(|| vec![1, 2, 3]);
    println!("{:?}", onemorehashmap);
}
