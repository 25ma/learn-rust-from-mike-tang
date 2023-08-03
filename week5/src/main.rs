// 声明一个add 宏
macro_rules! add {
    ($a: expr, $b: expr) => {
        $a + $b
    };
}

fn main() {
    let a = 30;
    let b = 55;

    let ab = add!(a, b);
    println!("{ab}");
}


// 1. 使用macro_rules! 声明宏，这个是rust 官方指定用于声明宏的 命令
// 2. 声明宏的方法一定要在使用宏（或者叫调用宏）之前，如果在使用宏（或者调用宏）之后声明会报错 cannot find macro `add` in this scope
// 3. macro_rules! 宏命令后面紧接这的是 当前需要声明宏的名字
// 4. ($a: expr, $b: expr) 其中 $a 和 $b 表示这个宏需要传递两个参数，  expr： 表示这是一个表达式
// 5. {} 大括号内就是这个声明宏的具体实现