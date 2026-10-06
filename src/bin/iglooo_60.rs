use std::num::ParseIntError;

fn someithrirtytwoerror(input: Vec<&str>) -> Result<Vec<i32>, ParseIntError> {
    let someoutput = input.iter().map(|s| s.parse::<i32>()).collect();
    someoutput
}

fn main() {
    let somelist_0 = vec!["1", "2", "igloo", "3", "nothing"];
    let somelist_1 = vec!["1", "2", "3"];

    let someanotherresult_0: Result<Vec<i32>, _> = someithrirtytwoerror(somelist_0);
    let someanotherresult_1: Result<Vec<i32>, _> = someithrirtytwoerror(somelist_1);

    println!("{:?}", someanotherresult_0);
    println!("{:?}", someanotherresult_1);
}
