fn main() {

    
// task 1

// 使用枚举包裹三个不同的类型，并放入一个Vec中，然后对Vec进行遍历，调用三种不同类型的各自的方法
    let name = Person::Name(String::from("zhangsan"));
    let age = Person::Age(18);
    let six = Person::Six(true);

    let persion = vec![
        name,
        age,
        six,
    ];

    println!("task 1: ");
    for row in persion.iter() {
        match row {
            Person::Name(ref name) => println!("{name}"),
            Person::Age(age) => println!("{age}" ),
            Person::Six(bool) => println!("{bool}"),
        }
    }


    // task 2

    // 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法

    let apple = Fruits {
        name: "apple".to_string(),
        color: (255,0,0),
        price: 3.99,
    };

    let orange = Fruits {
        name: "orange".to_string(),
        color: (153,204,0),
        price: 1.68,
    };

    let banner =  Fruits {
        name: "banana".to_string(),
        color: (255,255,0),
        price: 2.88,
    };

    let fruits = vec![
        apple,
        orange,
        banner,
    ];

    println!("task 2: ");
    for row in fruits.iter() {
         row.print_attr();
    }


    // 以上两种方式的区别是：
    //  1. enum 的方式在数组中进行遍历的时候需要使用match 匹配模式来进行不同类型的处理
    //  2. 而trait Object 的方式的话 是可以很方便在顶层trait 中定义通用的方法，遍历的时候也不需要做额外的处理，因为结构体的数据类型都是一致的
    //  3. trait Object 还可以针对结构体中不同的类型去处理数据，enum却没办法做到这一点
    //  4. 但是还有需要注意的就是如果 一个结构体实现了某个trait, 同时也需要实现这个trait 中的所有定义的方法


    // task 3 实现 Add trait  
    // 目前的问题是 FooBar 的name 值传递 数值类型 u32 i32  f32 f64 等数值都是可以正常编译的，传递String char &str 类型进去都编译不过，这块还在寻找解决办法
    let sum = FooBar{name: 1} + FooBar{name: 2};
    println!("FooBar + FooBar = {:?}",sum );

}

// task 1
#[derive(Debug, Clone)]
enum Person {
    Name(String),
    Age(u32),
    Six(bool)
}


trait GreenFood {
    fn get_name(&self) -> String;
    fn print_attr(&self);
}


// task 2 
#[derive(Debug)]
#[warn(dead_code)]
struct Fruits {
    name: String,
    color: (u32,u32,u32),
    price:f32,
}

impl  GreenFood for Fruits {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn print_attr(&self) {
        println!("name: {} , color: {:?}, price: {} 元/斤", self.name.clone(), self.color, self.price);
    }
}

impl  std::fmt::Display for Fruits {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        write!(f, "{:?}", self.color)
    }
}

// task 3  实现 Add trait 

use core::ops::Add;

#[derive(Debug)]
struct FooBar<T> {
    name: T,
}

impl<T: std::fmt::Debug> std::fmt::Display for FooBar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result{
        write!(f, "{:?}", self.name)
    }
}

impl<T> Add for FooBar<T>
where 
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        FooBar{ 
            name: self.name + rhs.name,
        }
    }
}