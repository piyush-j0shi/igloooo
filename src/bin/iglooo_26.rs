struct Summary<T> {
    vect: Vec<T>,
}

impl<T: std::cmp::PartialOrd> Summary<T> {
    fn largest(&self) -> Option<&T> {
        let mut largest = &self.vect[0];
        for i in &self.vect {
            if i > largest {
                largest = i;
            }
        }
        Some(largest)
    }
}

fn main() {
    let some_vec = Summary {
        vect: vec![1, 2, 3, 4],
    };
    let largest = some_vec.largest();
    println!("largest_value : {:?}", largest);
}
