// In class Assignment

// Create a struct Student (major)
struct Student {
    major:String,
}
// Higher order functions update majors

fn update_majors(collection: Vec<Student>,behavior:fn(&mut Student,String))
// First Order functions, assign_major(student,major_declared)

fn assign_major(& mut s:Student,major:String){

}
// create a vector of students1,2,3 and update all students major
let str_vec = vec![students1, students2, students3];