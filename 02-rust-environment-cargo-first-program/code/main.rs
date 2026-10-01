use std::io;

fn main() {
    println!("========================================");
    println!("    Student Grade Calculator PPL");
    println!("========================================");

    let mut name = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).unwrap();

    let mut midterm = String::new();
    println!("Enter Midterm score (35):");
    io::stdin().read_line(&mut midterm).unwrap();
    let midterm: f64 = midterm.trim().parse().unwrap();

    let mut final_exam = String::new();
    println!("Enter Final score (35):");
    io::stdin().read_line(&mut final_exam).unwrap();
    let final_exam: f64 = final_exam.trim().parse().unwrap();

    let mut kahoot = String::new();
    println!("Enter Kahoot score (12):");
    io::stdin().read_line(&mut kahoot).unwrap();
    let kahoot: f64 = kahoot.trim().parse().unwrap();

    let mut project = String::new();
    println!("Enter Project score (12):");
    io::stdin().read_line(&mut project).unwrap();
    let project: f64 = project.trim().parse().unwrap();

    let mut attendance = String::new();
    println!("Enter Attendance score (3):");
    io::stdin().read_line(&mut attendance).unwrap();
    let attendance: f64 = attendance.trim().parse().unwrap();

    let mut typing = String::new();
    println!("Enter Typing score (3):");
    io::stdin().read_line(&mut typing).unwrap();
    let typing: f64 = typing.trim().parse().unwrap();

    let total = midterm
        + final_exam
        + kahoot
        + project
        + attendance
        + typing;

    let grade = if total >= 80.0 {
        "A"
    } else if total >= 75.0 {
        "B+"
    } else if total >= 70.0 {
        "B"
    } else if total >= 60.0 {
        "C+"
    } else if total >= 50.0 {
        "C"
    } else if total >= 45.0 {
        "D+"
    } else if total >= 40.0 {
        "D"
    } else {
        "F"
    };

    println!();
    println!("========================================");
    println!("Name       : {}", name.trim());
    println!("Midterm    : {:.2}/35", midterm);
    println!("Final      : {:.2}/35", final_exam);
    println!("Kahoot     : {:.2}/12", kahoot);
    println!("Project    : {:.2}/12", project);
    println!("Attendance : {:.2}/3", attendance);
    println!("Typing     : {:.2}/3", typing);
    println!("----------------------------------------");
    println!("Total      : {:.2}/100", total);
    println!("Grade      : {}", grade);
    println!("========================================");
}

