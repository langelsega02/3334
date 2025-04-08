// In class Assignment
fn main() {
// Create a struct Student (major)
struct Student {
    major:String,
}
// Higher order functions update majors

fn update_majors(mut collection: Vec<Student>,behavior:fn(&mut Student,String)) -> Vec<Student>{
    for item in &mut collection{
        behavior(item, "CompSci".to_string());
    }
    collection
}
// First Order functions, assign_major(student,major_declared)

fn assign_major(s:&mut Student,major:String){
    s.major = major;
}
// create a vector of students1,2,3 and update all students major
let student1 = Student {major:"Aerospace Engineering".to_string()};
let student2 = Student {major:"Graphic Design".to_string()};
let student3 = Student {major:"Medicine".to_string()};

let str_vec = vec![student1, student2, student3];

let x = update_majors(str_vec, assign_major);

for s in &x {
    println!("Student major: {}", s.major);
}
}
