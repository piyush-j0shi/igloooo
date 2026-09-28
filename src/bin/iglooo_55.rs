struct SomeStruct {
    a_string: String,
    a_vec: Vec<i32>,
}

struct SmallStruct {
    something_readable: String,
    something_mutable: i32,
}

impl SomeStruct {
    fn new(some_string: String, some_vec: Vec<i32>) -> Self {
        Self {
            a_string: some_string,
            a_vec: some_vec,
        }
    }
}

impl SmallStruct {
    fn new(first_value: String, second_value: i32) -> Self {
        Self {
            something_readable: first_value,
            something_mutable: second_value,
        }
    }

    fn new_method(&mut self) {
        self.something_mutable += 1;
    }
}

fn main() {
    let mut somevec: Vec<i32> = vec![1, 2, 3];
    let element_one = &somevec[0];
    println!("{}", element_one);

    // we can not use it this way because we have an immutable borrow already and now we can not
    // borrow it as mutable because either there can be a lot of immutable borrow or mutable borrow
    // we can not use both for same variable.
    //
    // But there is a way to do this by printing the value before the mutable borrow because
    // a borrow doesn't last until the end of the scope — it ends after its last use

    // let mut some_closure = || {
    // somevec.push(1);
    // };
    // some_closure();
    // println!("{}", element_one);

    let mut some_closure = || {
        somevec.push(1);
    };
    some_closure();
    println!("{:?}", somevec);

    // This wouldn't have compiled in the previous rust versions.
    let mut somestruct = SomeStruct::new("String".to_string(), vec![1, 2, 3]);
    let someclosure = || {
        println!("a_string : {}", somestruct.a_string);
    };
    somestruct.a_vec.push(1);
    someclosure();

    println!(
        "a_string : {}, a_vec : {:?}",
        somestruct.a_string, somestruct.a_vec
    );

    // Same borrowing error here if we do it this way because the closure does the first immutable
    // borrow or read only and after we are firing the new_method() which mutates the struct inside
    // variable so we need to be careful here.

    // let mut smallstructvariable = SmallStruct::new("String".to_string(), 1);
    // let smallclosure = || println!("reads this : {}", smallstructvariable.something_readable);
    // smallstructvariable.new_method();
    // smallclosure();

    let mut smallstructvariable = SmallStruct::new("String".to_string(), 1);
    let smallclosure = || println!("reads this : {}", smallstructvariable.something_readable);
    smallclosure();
    smallstructvariable.new_method();
}
