// 1. The closure needs the variable to mutable at the call site because it can not mutate the
//    immutable variabl and tried to mutably borrow a non-mutable variablee
//2. Two counters from the same factory would be diffrent because they both have diffrent varibales
//3. impl Fn can not work as return type because our closure body mutates the capture.
//4. it lives inside the variable we assigned it to

#[allow(dead_code)]
fn counter_factory() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

#[allow(dead_code)]
fn accumalte_factory(mut starting_value: i32) -> impl FnMut(i32) -> i32 {
    move |x| {
        starting_value += x;
        starting_value
    }
}

#[allow(dead_code)]
fn rate_limiter(n: i32) -> impl FnMut() -> bool {
    let mut count = 0;

    move || {
        count += 1;
        count % n == 0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_factory() {
        let mut count = counter_factory();
        assert_eq!([count(), count(), count()], [1, 2, 3]);
    }

    #[test]
    fn test_accumalte_factory() {
        let mut something = accumalte_factory(0);
        assert_eq!([something(10), something(5), something(3)], [10, 15, 18]);
    }

    #[test]
    fn test_ratelimiter() {
        let mut ratelimiter = rate_limiter(3);
        assert_eq!(
            [
                ratelimiter(),
                ratelimiter(),
                ratelimiter(),
                ratelimiter(),
                ratelimiter(),
                ratelimiter(),
                ratelimiter(),
                ratelimiter()
            ],
            [false, false, true, false, false, true, false, false]
        )
    }
}
