struct Collector<T, E> {
    vecte: Vec<Result<T, E>>,
}

impl<T, E> Collector<T, E> {
    fn push(&mut self, result: Result<T, E>) {
        self.vecte.push(result);
    }

    fn successes(&self) -> Vec<&T> {
        let mut new_vec: Vec<&T> = Vec::new();

        for item in &self.vecte {
            match item {
                Ok(value) => new_vec.push(value),
                Err(error) => (),
            }
        }
        return new_vec;
    }

    fn errors(&self) -> Vec<&E> {
        let mut new_vec: Vec<&E> = Vec::new();

        for item in &self.vecte {
            match item {
                Ok(value) => (),
                Err(error) => new_vec.push(error),
            }
        }
        return new_vec;
    }

    fn all_ok(self) -> Result<Vec<T>, E> {
        self.vecte.into_iter().collect()
    }
}

fn main() {
    let mut coll = Collector { vecte: Vec::new() };

    coll.push(Ok(10));
    coll.push(Ok(20));
    coll.push(Err("oops"));
    coll.push(Ok(30));

    println!("Successes: {:?}", coll.successes());
    println!("Errors: {:?}", coll.errors());

    match coll.all_ok() {
        Ok(values) => println!("All ok: {:?}", values),
        Err(e) => println!("First error: {:?}", e),
    }
}
