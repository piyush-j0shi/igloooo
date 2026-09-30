use std::{sync::Arc, thread::JoinHandle};

fn spawn_workers_with_sync<F>(n: usize, f: F) -> Vec<JoinHandle<()>>
where
    F: Fn(usize) + Send + Sync + 'static,
{
    let something = Arc::new(f);

    let mut vector = Vec::new();
    for i in 0..n {
        let somevariable = Arc::clone(&something);
        let handle = std::thread::spawn(move || {
            somevariable(i);
        });
        vector.push(handle);
    }
    vector
}

fn spawn_workers_with_clone<F>(n: usize, f: F) -> Vec<JoinHandle<()>>
where
    F: Fn(usize) + Send + Clone + 'static,
{
    let mut vector = Vec::new();
    for i in 0..n {
        let another_f = f.clone();

        let handle = std::thread::spawn(move || {
            another_f(i);
        });
        vector.push(handle);
    }
    vector
}

fn with_thread_scope<F>(n: usize, f: F)
where
    F: Fn(usize) + Sync,
{
    std::thread::scope(|s| {
        for i in 0..n {
            let ref_f = &f;
            s.spawn(move || {
                ref_f(i);
            });
        }
    })
}

fn main() {
    let vector: Vec<JoinHandle<()>> = spawn_workers_with_sync(8, |x| println!("{}", x));
    for i in vector {
        i.join().unwrap();
    }

    println!();

    let vector_1: Vec<JoinHandle<()>> = spawn_workers_with_clone(8, |x| println!("{}", x));
    for i in vector_1 {
        i.join().unwrap();
    }

    println!();
    with_thread_scope(5, |x| println!("{}", x));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_one() {
        let arc_mutex = Arc::new(Mutex::new(Vec::new()));
        let clone_above = Arc::clone(&arc_mutex);
        let results = spawn_workers_with_sync(8, move |x| clone_above.lock().unwrap().push(x));

        for result in results {
            result.join().unwrap();
        }

        let mut result_final = arc_mutex.lock().unwrap();
        result_final.sort();
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7], *result_final);
    }
}
