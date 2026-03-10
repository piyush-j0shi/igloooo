use std::io;

fn read_input() -> f64 {
    let mut marks = String::new();
    io::stdin().read_line(&mut marks).unwrap();
    let parsed: f64 = marks.trim().parse().unwrap();
    return parsed;
}

fn read_no_of_subjects() -> i32 {
    let mut marks = String::new();
    io::stdin().read_line(&mut marks).unwrap();
    let parsed: i32 = marks.trim().parse().unwrap();
    return parsed;
}

fn average(numbers: &Vec<f64>) -> f64 {
    let total_subjects = numbers.len() as f64;
    if numbers.is_empty() {
        println!("total marks can not be empty");
        return 0.0;
    } else {
        let mut sum = 0.0;
        for i in numbers {
            sum += i;
        }
        return sum / total_subjects;
    }
}

fn main() {
    let mut total_marks: Vec<f64> = Vec::new();

    println!("enter the total no of subjects you want");
    let total_no_of_subjects = read_no_of_subjects();

    for i in 0..total_no_of_subjects {
        println!("enter marks for subject {}", i + 1);
        let mark = read_input();

        if mark > 0.0 {
            total_marks.push(mark);
        } else {
            println!("marks can not be negative");
        }
    }

    //    if total_marks.is_empty() {
    //        println!("total marks can not be empty");
    //    } else {
    //        println!("marks : {:?}", total_marks);
    //    }

    let average = average(&total_marks);
    println!("average is : {}", average);
}
