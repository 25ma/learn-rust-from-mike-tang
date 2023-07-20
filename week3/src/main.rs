
// 学生
#[derive(Debug, Clone)]
#[warn(dead_code)]
struct Student {
    name: String, // 姓名
    age: u32, // 年龄
    sex: bool, // 性别
    class: Option<Class>, // 班级
    lesson: Option<Lesson>, // 课程
    club: String, // 社团
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

// 班级
#[derive(Debug, Clone)]
#[warn(dead_code)]
enum Class {
    A,
    B,
    C,
    D,
}


// 目前的curd 都故意避开了生命周期的处理
fn main() {
    // 添加学生信息
    let username = String::from("zhangsan");
    let club_name = "dance".to_string();

    let mut student = Student::new(username, 12,true, Some(Class::A),  Some(Lesson::English), club_name);
    println!("添加学生信息：{:?}", student);

    let new_club_name = "music".to_string();
    student.edit(13, new_club_name, Some(Lesson::English));

    println!("修改学生信息1: {:?}", student);

    let learn_club = String::from("shared learning");
    student.edit(14, learn_club, Some(Lesson::Sports));

    println!("修改学生信息2: {:?}", student);
    println!("修改学生信息3");
    let s3_name = String::from("wangwu");
    let s3_age = 15;
    let s3_sex = false;
    let s3_lesson = Some(Lesson::Sports);
    let s3_club = String::from("music");
    let s3_class = Some(Class::C);
    let mut student3 = Student {
        name: s3_name,
        age: s3_age,
        sex: s3_sex,
        lesson: s3_lesson,
        class: s3_class,
        club: s3_club,
    };
    println!("student3 修改前：{:?}", student3);

    student3.age = 16;

    println!("student3 修改年龄：{:?}", student3);

    student3.class = Some(Class::D);

    println!("student3 修改班级： {:?}",student3);
 
    let username = String::from("lisi");
    let club_name = "dance".to_string();
    let student2 = Student::new(username, 17, false, Some(Class::B),Some(Lesson::Math),club_name);
    let student2_info = student2.find();
    println!("查询student2 信息: {:?}", student2_info);
}
