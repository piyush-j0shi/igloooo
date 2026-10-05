use core::fmt;
use thiserror::Error;

#[derive(Debug)]
enum CustomError {
    BadJson(String),
    Io(std::io::Error),
}

#[derive(Debug, Error)]
enum SomeError {
    #[error("invalid input: {0}")]
    BadJson(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl std::error::Error for CustomError {}

// We need this because the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
impl From<std::io::Error> for CustomError {
    fn from(value: std::io::Error) -> Self {
        CustomError::Io(value)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::BadJson(msg) => write!(f, "invalid input : {}", msg),
            CustomError::Io(err) => write!(f, "i/o error : {}", err),
        }
    }
}

fn first_function(input: &str) -> Result<i32, String> {
    input
        .parse::<i32>()
        .map_err(|_| "could not parse number".to_string())
        .and_then(|n| {
            if n > 0 {
                Ok(n)
            } else {
                Err("Number must be positive".to_string())
            }
        })
}

fn second_function(path: &str) -> Result<String, CustomError> {
    let recieved = std::fs::read_to_string(path)?.trim().to_string();
    if recieved.starts_with("}") {
        Err(CustomError::BadJson("Invalid JSON Start".to_string()))
    } else {
        Ok(recieved)
    }
}

fn third_function(path: &str) -> Result<String, SomeError> {
    let recieved = std::fs::read_to_string(path)?.trim().to_string();
    if recieved.starts_with("}") {
        Err(SomeError::BadJson("Invalid JSON Start".to_string()))
    } else {
        Ok(recieved)
    }
}

fn main() {
    let value = first_function("2").unwrap_or_else(|err| {
        println!("Error : {}", err);
        0
    });

    let good = first_function("2");
    let bad = first_function("bad");

    println!("{}", value);
    println!("{:?}", good.ok());
    println!("{:?}", bad.err());

    let result_of_secondfunction_0 = second_function("somefile.somefile");
    println!("{:?}", result_of_secondfunction_0);

    let result_of_secondfunction_1 = second_function("badstart.badstart");
    println!("{:?}", result_of_secondfunction_1);

    let result_of_secondfunction_2 = second_function("notfound.notfound");
    println!("{:?}", result_of_secondfunction_2);

    let result_of_thirdfunction_0 = third_function("somefile.somefile");
    println!("{:?}", result_of_thirdfunction_0);

    let result_of_thirdfunction_1 = third_function("badstart.badstart");
    println!("{:?}", result_of_thirdfunction_1);

    let result_of_thirdfunction_2 = third_function("notfound.notfound");
    println!("{:?}", result_of_thirdfunction_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("iglooo_59_{}", name));
        let mut f = std::fs::File::create(&path).expect("could not create temp file");
        f.write_all(contents.as_bytes())
            .expect("could not write temp file");
        path
    }

    #[test]
    fn missing_file_gives_io_variant() {
        let result = second_function("definitely_not_a_real_file.nope");
        assert!(matches!(result, Err(CustomError::Io(_))));
    }

    #[test]
    fn bad_contents_give_badjson_variant() {
        let path = temp_file("bad", "}\n  \"ok\": true\n");
        let result = second_function(path.to_str().unwrap());
        assert!(matches!(result, Err(CustomError::BadJson(_))));
    }

    #[test]
    fn good_contents_give_ok_and_are_trimmed() {
        let path = temp_file("good", "  Hello  \n");
        let result = second_function(path.to_str().unwrap());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn thiserror_version_behaves_identically() {
        let bad = temp_file("te_bad", "}\n");
        let good = temp_file("te_good", "Hello\n");

        assert!(matches!(
            third_function("definitely_not_a_real_file.nope"),
            Err(SomeError::Io(_))
        ));
        assert!(matches!(
            third_function(bad.to_str().unwrap()),
            Err(SomeError::BadJson(_))
        ));
        assert_eq!(third_function(good.to_str().unwrap()).unwrap(), "Hello");
    }
}
