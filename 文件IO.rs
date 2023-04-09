use std::io::stdin;
use std::fs;
use std::io::prelude::*;

fn main(){
    let args = std::env::args();
    for arg in args{
        println!("{}", arg);
    }
    // {
    // let mut str_buf = String::new();
    // stdin().read_line(&mut str_buf).expect("faild to read line.");
    // println!("Your input line is \n{}", str_buf);
    // }
//  文件读取
    {
    let text = fs::read_to_string("./test.txt").unwrap();
    println!("{}", text);
    }
    {
        
    }
}


