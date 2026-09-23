//1. we are mutating the captures so we do not want to use the Fn() and then closures are going to
//   be used more than one so FnOnce() is also out
//2. When attempt is 0 then it will give the E::deafult() this is kept for now it will be changed
//   in the future
//3. If I want to return every error then maybe closures bound is not going to change the function
//   return signature is going to change to  a Vec<E>

#[allow(dead_code)]
fn retry<T, E, F>(n: i32, mut action: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: Default,
{
    let mut last_item: Result<T, E> = Err(E::default());
    for _ in 0..n {
        let result = action();
        if result.is_ok() {
            return result;
        }
        last_item = result;
    }
    last_item
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_1() {
        let result: Result<i32, String> = retry(3, || Ok(42));
        assert_eq!(Ok(42), result);
    }

    #[test]
    fn test_retry_2() {
        let mut attempts = 0;
        let result: Result<&str, String> = retry(3, || {
            attempts += 1;
            if attempts < 3 {
                Err(format!("boom {}", attempts))
            } else {
                Ok("worked")
            }
        });

        assert_eq!(Ok("worked"), result);
    }

    #[test]
    fn test_retry_3() {
        let result: Result<i32, &str> = retry(2, || Err("nope"));
        assert_eq!(Err("nope"), result);
    }
}
