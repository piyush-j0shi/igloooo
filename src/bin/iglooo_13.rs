use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum GradingScale {
    Percentage,
    LetterGrade,
}

#[derive(Debug)]
struct Subject {
    grades: Vec<f64>,
    weight: f64,
    grading_scale: GradingScale,
}

#[derive(Debug)]
struct Student {
    name: String,
    subjects: HashMap<String, Subject>,
}

#[derive(Debug)]
struct StudentDatabase {
    students: HashMap<String, Student>,
}

impl StudentDatabase {
    fn new() -> Self {
        let studentdbnew: HashMap<String, Student> = HashMap::new();
        Self {
            students: studentdbnew,
        }
    }

    fn add_student(&mut self, name: &str, subject_name: &str, subject: Subject) {
        //  let english = Subject {
        //      grades: vec![89.0, 92.0, 39.0, 42.0],
        //      weight: 1.0,
        //      grading_scale: GradingScale::Percentage,
        //  };

        if let Some(previous_student) = self.students.get_mut(name) {
            previous_student
                .subjects
                .insert(subject_name.to_string(), subject);
        } else {
            let mut subject_map: HashMap<String, Subject> = HashMap::new();
            subject_map.insert(subject_name.to_string(), subject);

            let student = Student {
                name: name.to_string(),
                subjects: subject_map,
            };

            self.students.insert(name.to_string(), student);
        }
    }

    fn calculate_grade(&mut self, student_name: &str, subject_name: &str) {
        if let Some(student_details) = self.students.get(&student_name.to_string()) {
            if let Some(subject) = student_details.subjects.get(&subject_name.to_string()) {
                // println!("student_details : {:?}", subject);

                let mut total_marks = 0.0;
                let total = subject.grades.len() as f64;

                for marks in &subject.grades {
                    total_marks += marks;
                }
                let average = total_marks / total;

                match subject.grading_scale {
                    GradingScale::Percentage => println!("percentage : {}", average),
                    GradingScale::LetterGrade => {
                        if average >= 90.0 && average <= 100.0 {
                            println!("Grade : A");
                        } else if average < 90.0 && average >= 80.0 {
                            println!("Grade : B");
                        } else if average < 80.0 && average >= 70.0 {
                            println!("Grade : C");
                        } else if average < 70.0 && average >= 60.0 {
                            println!("Grade : D");
                        } else if average < 60.0 && average >= 40.0 {
                            println!("Grade : E");
                        } else {
                            println!("fail");
                        }
                    }
                }
            } else {
                println!("nothing found");
            }
        } else {
            println!("student does not exists");
        }
    }

    fn calculate_all_grades(&self) {
        for every_student in &self.students {
            // println!("every students : {:#?}", every_student.0);
            for every_subject in &every_student.1.subjects {
                // println!("every subjects are : {:#?}", every_subject);
                // println!("grades : {:?}", every_subject.1.grades);
                let mut total_sum = 0.0;
                for sum in &every_subject.1.grades {
                    total_sum += sum;
                }
                let length = every_subject.1.grades.len() as f64;
                // println!("total_sum : {}", total_sum);
                println!(
                    "name : {} | subject : {} | percentage : {}",
                    every_student.0,
                    every_subject.0,
                    total_sum / length
                );
                println!("======================================================");
            }
        }
    }
}
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    input.trim().to_string()
}

fn read_marks() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    let float64: f64 = input.trim().parse().unwrap();
    float64
}

fn get_details(grades: Vec<f64>) -> Subject {
    println!("enter method : grade or %");
    let method = read_input();

    if method.trim().to_lowercase() == "grade" {
        let subject = Subject {
            grades: grades,
            weight: 1.0,
            grading_scale: GradingScale::LetterGrade,
        };
        return subject;
    } else if method.trim().to_lowercase() == "%" {
        let subject = Subject {
            grades: grades,
            weight: 2.0,
            grading_scale: GradingScale::Percentage,
        };
        return subject;
    } else {
        println!("invalid choice, default is percentage");
        let subject = Subject {
            grades: grades,
            weight: 2.0,
            grading_scale: GradingScale::Percentage,
        };
        return subject;
    }
}

fn get_grades() -> Vec<f64> {
    println!("enter the no of grades");
    let no_of_grades = read_marks() as i32;

    let mut grades: Vec<f64> = Vec::new();
    for i in 0..no_of_grades {
        println!("enter the {}'st grade : ", i + 1);
        let marks = read_marks();

        if marks > 0.0 {
            grades.push(marks);
        } else {
            println!("marks can not be negative, enter again");
            let reenter_marks = read_marks();
            grades.push(reenter_marks);
        }
    }
    // println!("grades : {:?}", grades);
    return grades;
}

// fn main() {
//     let mut studentdb = StudentDatabase::new();
//
//     println!("enter student name");
//     let student_name = read_input();
//
//     println!("enter the subject name");
//     let subject_name = read_input();
//
//     //  studentdb.add_student("student1");
//     //  studentdb.add_student("student2");
//
//     let grades = get_grades();
//     let subject = get_details(grades);
//     studentdb.add_student(student_name.as_str(), subject_name.as_str(), subject);
//
//     // println!("new student : {:#?}", studentdb);
//
//     studentdb.calculate_grade(student_name.as_str(), subject_name.as_str());
//     studentdb.calculate_all_grades();
// }

fn main() {
    let mut studentdb = StudentDatabase::new();

    let alice_math = Subject {
        grades: vec![85.0, 92.0, 78.0, 88.0],
        weight: 1.0,
        grading_scale: GradingScale::Percentage,
    };

    let alice_english = Subject {
        grades: vec![91.0, 87.0, 94.0, 89.0],
        weight: 1.0,
        grading_scale: GradingScale::LetterGrade,
    };

    let alice_science = Subject {
        grades: vec![78.0, 82.0, 85.0, 80.0],
        weight: 1.5,
        grading_scale: GradingScale::Percentage,
    };

    studentdb.add_student("Alice", "Math", alice_math);
    studentdb.add_student("Alice", "English", alice_english);
    studentdb.add_student("Alice", "Science", alice_science);

    // println!("student db : {:#?}", studentdb);

    studentdb.calculate_grade("Alice", "Math");
    studentdb.calculate_grade("Alice", "English");

    println!("======================================================");
    println!("\n");

    studentdb.calculate_all_grades();
}
