#[derive(Debug)]

struct Site{
    domain: String,
    name: String,
    nation: String,
    found: u32
}

fn main(){
    let runoob = Site{
    domain: String::from("www.bing.com"),
    name: String::from("RUNOOB"),
    nation: String::from("China"),
    found: 2013
    };
    let site = Site{
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };

    //  元组结构体
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0); 
    let origin = Point(0.0, 0.0);

    {   //结构体输出
        struct Rectangle{
            width:u32,
            height: u32
        }

        
        let rect1 = Rectangle{width: 30, height: 50};
        println!("rect1 is {:?}", rect1);
        

    }


}

