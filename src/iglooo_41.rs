use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();

    let handle = thread::spawn(move || {
        tx.send(5).unwrap();

        let recieved = rx1.recv().unwrap();
        println!("recieved : {}", recieved);
    });
    handles.push(handle);
    let handle = thread::spawn(move || {
        let value = rx.recv().unwrap();
        println!("recieved : {}", value);
        tx1.send(value * value).unwrap();
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

}
