struct SomeStruct<F>
where
    F: Fn() -> i32,
{
    filed: F,
}

// The reason for choosing box here is we can get diffrent get diffrent type of closures due to box
// and dyn because the compiler can not know until the runtime that's why and box helps us to make
// things comfortable for size
struct AnotherStruct {
    element: Box<dyn Fn() -> i32>,
}

struct OneMoreStruct {
    onefield: Box<dyn FnMut(&str) -> usize>,
}

#[allow(dead_code)]
impl<F> SomeStruct<F>
where
    F: Fn() -> i32,
{
    fn new(args: F) -> Self {
        Self { filed: args }
    }

    fn fired(&self) -> i32 {
        (self.filed)()
    }
}

#[allow(dead_code)]
impl AnotherStruct {
    fn new(args: Box<dyn Fn() -> i32>) -> Self {
        Self { element: args }
    }

    fn fired(&self) -> i32 {
        (self.element)()
    }
}

#[allow(dead_code)]
impl OneMoreStruct {
    fn new(args: Box<dyn FnMut(&str) -> usize>) -> Self {
        Self { onefield: args }
    }

    // can not use &self because it can not borrow mutable, as out closure is FnMut() so rust need
    // it as mutable
    fn fired(&mut self, some: &str) -> usize {
        (self.onefield)(some)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_somestruct() {
        let some_struct = SomeStruct::new(|| 5);
        assert_eq!(some_struct.fired(), 5);
    }

    #[test]
    fn test_anotherstruct() {
        let vector_2 = vec![
            AnotherStruct::new(Box::new(|| 2)),
            AnotherStruct::new(Box::new(|| -2)),
        ];

        let mut vector_4 = Vec::new();
        for item in vector_2 {
            vector_4.push(item.fired());
        }

        assert_eq!(vector_4, [2, -2]);
    }

    #[test]
    fn test_onemorestruct() {
        let mut vector_3: Vec<String> = vec![];

        let mut onemorestruct = OneMoreStruct::new(Box::new(move |event| {
            vector_3.push(event.to_string());
            vector_3.len()
        }));

        assert_eq!(
            vec![
                onemorestruct.fired("one"),
                onemorestruct.fired("two"),
                onemorestruct.fired("three"),
            ],
            [1, 2, 3]
        );
    }
}
