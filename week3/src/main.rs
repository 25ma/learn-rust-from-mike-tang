
// 学生
#[derive(Debug, Clone)]
#[warn(dead_code)]
struct Student {
    name: String,
    age: u32,
    sex: bool,
    class: Option<Class>,
    lesson: Option<Lesson>,
    club: String,
}


impl Student {
    fn new(username: String, age: u32, sex: bool, class: Option<Class> , lesson: Option<Lesson>,  club: String) -> Self {
        Self {
            name: username,
            age: age,
            sex: sex,
            lesson: lesson,
            class: class,
            club: club,
        }
    }

    fn edit(self: &mut Self,age: u32, club: String, lesson: Option<Lesson>){
        self.age = age;
        self.club = club;
        self.lesson = lesson;
    }


    fn find(self: &Self)-> Self {
        Self {
            name: self.name.clone(),
            age: self.age,
            sex: self.sex,
            lesson: self.lesson.clone(),
            class: self.class.clone(),
            club: self.club.clone()
        }
    }
}

// 课程
#[derive(Debug, Clone)]
#[warn(dead_code)]
enum  Lesson {
    English,
    Math,
    Sports
}

// 社团
#[derive(Debug, Clone)]
#[warn(dead_code)]
struct Club {
    name: String,
}

// 班级
#[derive(Debug, Clone)]
#[warn(dead_code)]
enum Class {
    A,
    B,
    C,
    D,
}

fn main() {
    // 添加学生信息
    let username = String::from("zhangsan");
    let club_name = "dance".to_string();

    println!("添加学生信息：");
    let mut student = Student::new(username, 12,true, Some(Class::A),  Some(Lesson::English), club_name);
    println!("{:#?}", student);
    println!("修改学生信息");

    let new_club_name = "music".to_string();
    student.edit(13, new_club_name, Some(Lesson::English));

    println!("{:#?}", student);

    let username = String::from("lisi");
    let club_name = "dance".to_string();
    let student2 = Student::new(username, 13, false, Some(Class::B),Some(Lesson::Math),club_name);

    println!("查询student2 信息");
    let student2_info = student2.find();
    println!("{:#?}", student2_info);
}
