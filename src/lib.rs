use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plant_map: HashMap<char, &str> = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_index = match students.iter().position(|&s| s == student) {
        Some(index) => index,
        None => return vec![],
    };

    let rows: Vec<&str> = diagram.split('\n').collect();

    if rows.len() != 2 {
        return vec![];
    }

    let cups_per_student = 2;
    let start = student_index * cups_per_student;
    let end = start + cups_per_student;

    rows.iter()
        .flat_map(|row| row[start..end].chars())
        .filter_map(|c| plant_map.get(&c).copied())
        .collect()
}
