/// 函数
/// example
/// let x = add(1, 2)

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main(){
    let a = 12; //不可变变量
    let mut b = 14; // 可变变量
    println!("a is {}", a);
    /* 复合类型 */
{
    let a = [1, 2, 3, 4, 5];
    let tup: (i32, f64, u8) = (800, 3.14, 1);
    let b = ["January", "February", "March"];
    let c: [i32;5] = [1, 2, 3, 4, 5];
    let d = [3;6];//6个3
    let first = a[0];
    let second = a[1];
}

/* 测试函数 */
{
    println!("1 + 1 = {}", add(1, 1));
}

/* 测试条件语句 */
{
    let num = 3;
    if num < 3{
        println!("True");
    }else{
        println!("False");
    }

    //三元表达式
    let a = if num > 0 {1} else{ 0 };
    println!("a = {}", a);

}

/* 循环 */
{
    /// while 循环
    let mut n = 0;
    while n < 4{
        println!("n = {}", n);
        n += 1;
    }
    /// for 循环
    let li = [10, 20, 30, 40, 50, 60];
    for i in li.iter() {
        println!("{}", i);
    }
    /// loop 循环
    loop {
        break;
    }

}





}



