struct Pipeline<T> {
    struct_element: Vec<Box<dyn Fn(T) -> T>>,
}

struct AnotherPipeline<T, E> {
    struct_element: Vec<Box<dyn Fn(T) -> Result<T, E>>>,
}

impl<T> Pipeline<T> {
    fn new() -> Self {
        Self {
            struct_element: Vec::new(),
        }
    }

    fn add(mut self, input: impl Fn(T) -> T + 'static) -> Self {
        self.struct_element.push(Box::new(input));
        self
    }

    fn run(&self, some_input: T) -> T {
        let mut value = some_input;
        for stage in &self.struct_element {
            value = stage(value);
        }
        value
    }
}

impl<T, E> AnotherPipeline<T, E> {
    fn new() -> Self {
        Self {
            struct_element: Vec::new(),
        }
    }

    fn add(mut self, input: impl Fn(T) -> Result<T, E> + 'static) -> Self {
        self.struct_element.push(Box::new(input));
        self
    }

    fn run(&self, some_input: T) -> Result<T, E> {
        let mut value = some_input;

        for stage in &self.struct_element {
            match stage(value) {
                Ok(v) => value = v,
                Err(e) => return Err(e),
            }
        }
        Ok(value)
    }
}

fn main() {
    let pipeline = Pipeline::new()
        .add(|s: String| s.trim().to_lowercase())
        .add(|s: String| s.split_whitespace().collect::<Vec<_>>().join(" "));

    println!("{}", pipeline.run("  Hello WOrld  ".to_string()));

    let validator: AnotherPipeline<String, String> = AnotherPipeline::new()
        .add(|s: String| Ok(s.trim().to_string()))
        .add(|s: String| {
            if s.is_empty() {
                Err("input was empty".to_string())
            } else {
                Ok(s)
            }
        })
        .add(|s: String| {
            if s.len() > 20 {
                Err(format!("too long: {} chars", s.len()))
            } else {
                Ok(s)
            }
        })
        .add(|s: String| Ok(s.to_uppercase()));

    for input in [
        "  hello  ",
        "   ",
        "this string is definitely longer than twenty characters",
        "ok",
    ] {
        println!("{:?} -> {:?}", input, validator.run(input.to_string()));
    }
}
