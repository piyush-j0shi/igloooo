use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

#[derive(Clone)]
struct Storage{
    values : Arc<Mutex<HashMap<String, String>>>
}

impl Storage {
    fn new() -> Self {
        Storage { values : Arc::new(Mutex::new(HashMap::new())) }
    }

    fn set(&self, key : String, value : String){
        let mut map = self.values.lock().unwrap();
        map.insert(key, value);
    }

    fn get(&self, key : &str) -> Option<String>{
        let map = self.values.lock().unwrap();
        map.get(key).cloned()
    }
}


fn main() {
    let storage = Storage::new();
    let storage_thread = storage.clone();

    let handle = thread::spawn(move || {
        storage_thread.set(String::from("Okay"), String::from("Not Okay"));
    });
    handle.join().unwrap();
    println!("Value for 'Okay': {:?}", storage.get("Okay"));
}