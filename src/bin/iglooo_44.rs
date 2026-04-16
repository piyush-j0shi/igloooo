use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Payload {
    pub id: u32,
    pub data: String,
}

impl Payload {
    fn new(id : u32, data : String) -> Self {
        Payload {
            id : id,
            data : data,
        }
    }
}

#[derive(Debug)]
enum Status {
    Pending,
    Success,
}

type ApiSender = mpsc::Sender<Payload>;
type ApiReceiver = mpsc::Receiver<Payload>;
type StatusTracker = Arc<Mutex<HashMap<u32, Status>>>;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_ed : ApiSender = tx;
    let rx_ed : ApiReceiver = rx;

    let mut handles = vec![];
    let statustacker : StatusTracker = Arc::new(Mutex::new(HashMap::new()));
    let status_tracker = Arc::clone(&statustacker);

    for i in 0..4 {
        let cloned_sender = tx_ed.clone();
        let tracker_clone = Arc::clone(&statustacker);

        let handle = thread::spawn(move || {
            let random_payloads = Payload::new(i + 1, String::from("value"));
            tracker_clone.lock().unwrap().insert(random_payloads.id, Status::Pending);
            cloned_sender.send(random_payloads).unwrap();
        });
        handles.push(handle);
    }

    let handle = thread::spawn(move || {
        while let Ok(payload) = rx_ed.recv() {
            println!("Processing ID: {}", payload.id);
            if let Some(status) = status_tracker.lock().unwrap().get_mut(&payload.id) {
                *status = Status::Success;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    drop(tx_ed);
    for handle in handles{
        handle.join().unwrap();
    }
    handle.join().unwrap();
    println!("Final Status Tracker: {:#?}", statustacker.lock().unwrap());
}
