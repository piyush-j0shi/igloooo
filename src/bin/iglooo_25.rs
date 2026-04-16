use std::collections::HashMap;

#[derive(Debug)]
struct KeyValue<K, V> {
    value: HashMap<K, V>,
}

impl<
        K: std::fmt::Display + std::fmt::Debug,
        V: std::hash::Hash + std::cmp::Eq + std::fmt::Display + std::fmt::Debug,
    > KeyValue<K, V>
{
    fn swap(self) -> KeyValue<V, K> {
        let mut new_hashmap = HashMap::new();

        for (key, values) in self.value.into_iter() {
            new_hashmap.insert(values, key);
        }
        return KeyValue { value: new_hashmap };
    }

    fn print_pair(&self) {
        println!("pair is : {:?}", &self);
    }
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let hash_struct = KeyValue { value: scores };
    println!("current hasmap : {:?}", hash_struct);

    let after_method = hash_struct.swap();
    println!("after swap : {:?}", after_method);
}
