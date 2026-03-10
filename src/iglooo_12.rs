#[derive(Debug)]
enum HistoryOperations {
    Push(String),
    Append(String),
    Prepend(String),
    Replace { from: String, to: String },
}

#[derive(Debug)]
struct CustomString {
    currentvalue: String,
    history: Vec<HistoryOperations>,
}

impl CustomString {
    fn new(input_string: String) -> Self {
        let mut history: Vec<HistoryOperations> = Vec::new();
        history.push(HistoryOperations::Push(input_string.clone()));

        Self {
            currentvalue: input_string,
            history: history,
        }
    }

    fn append(&mut self, value: String) {
        self.currentvalue.push_str(value.as_str());
        self.history.push(HistoryOperations::Append(value));
    }

    fn prepend(&mut self, value: String) {
        self.currentvalue = value.clone() + self.currentvalue.as_str();
        self.history.push(HistoryOperations::Prepend(value));
    }

    fn replace(&mut self, from_str: &str, to_str: &str) {
        self.currentvalue = self.currentvalue.replace(from_str, to_str);
        self.history.push(HistoryOperations::Replace {
            from: from_str.to_string(),
            to: to_str.to_string(),
        })
    }
}

fn main() {
    let somestring = "somestring".to_string();
    println!("somestring is : {}", somestring);

    let mut somenewstring = CustomString::new("something".to_string());
    println!("somenewstring is : {:?}", somenewstring);

    somenewstring.append("trying".to_string());
    println!("somenewstring is : {:?}", somenewstring);

    somenewstring.prepend("trying".to_string());
    println!("somenewstring is : {:?}", somenewstring);

    somenewstring.replace("trying", "nottrying");
    println!("somenewstring is : {:?}", somenewstring);
}
