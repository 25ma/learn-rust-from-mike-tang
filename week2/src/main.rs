fn main() {
        // 所有权规则

        // - 每一个值都有一个所有者 \r\n
        // - 值在任何时刻都只有一个所有者 \r\n
        // - 当所有者离开作用域的时候，这个值也会被销毁\r\n
        let a = String::from("hello world");
        let b = a;
    
        // 此时打印a 变量会提示 value borrowed here after move 这时候就相当于 a 变量的所有权已经转移到了b 变量
        // println!("{a}");

        // 不可变引用 \r\n
        // 不可变引用类似：我们生活中的借东西，别人有一件很好玩的东西，我们可以借来玩，但是我们没有这个东西的所有权，不能在这个东西上直接进行肆意破坏或者改造。\r\n
        let c = String::from("Immutable reference");
        let c2 = &c;

        // cannot assign twice to immutable variable
        // c2 = &String::from("mutable reference");

        // 可变引用 \r\n
        // 可变引用： 在同一时间只能有一个对某一特定数据的可变引用。如果同时会两个可变引用的代码，编译器将会报错。\r\n
        
        let mut d = String::from("mutable reference");
        let  e = &mut d;
        println!("{}", e);
        let f = &mut d;
        // 此时会出现 second mutable borrow occurs here ，如果不打印 变量 e 则不会，因为e 变量的作用域在 26行只有再无任何变量引用，则会直接释放，然而这时候 变量f 的可变引用是有效的
        // println!("{}", e);

        // 悬垂引用： 指向的内存可能已经被分配给其它持有者 \r\n
        // expected named lifetime parameter 或者 'static 生命周期标注 以及返回的数据类型还需要拥有数据的所有权
        danglingReferences();
}

// fn danglingReferences()->&String {  // 悬垂引用
fn danglingReferences()->&'static str {  // 改造后无悬垂引用
    // let string = String::from("hello world"); // 悬垂引用
    let string = "hello world"; // 改造后无悬垂引用
    &string
}

