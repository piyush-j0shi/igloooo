use std::sync::{Arc, Mutex, mpsc};
use std::{thread, vec};

type RawDocuments = Vec<String>;
type SharedReceiver = Arc<Mutex<mpsc::Receiver<String>>>;
type VectorStore = Arc<Mutex<Vec<String>>>;

fn main() {
    let raw_data: RawDocuments = vec![
        String::from("THis"),
        String::from("is"),
        String::from("a"),
        String::from("document"),
    ];
    let storage: VectorStore = Arc::new(Mutex::new(Vec::new()));
    let (tx, raw_rx) = mpsc::channel();
    let rx: SharedReceiver = Arc::new(Mutex::new(raw_rx));

    let mut handles = vec![];

    thread::spawn(move || {
        for value in raw_data {
            tx.send(value).unwrap();
        }
    });

    for _ in 0..4 {
        let cloned_rx = Arc::clone(&rx);
        let storage_cloned = Arc::clone(&storage);

        let handle = thread::spawn(move || {
            loop {
                let recieved = match cloned_rx.lock().unwrap().recv() {
                    Ok(doc) => doc,
                    Err(_) => break,
                };

                storage_cloned.lock().unwrap().push(recieved);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("final result : {:?}", storage.lock().unwrap());
}
