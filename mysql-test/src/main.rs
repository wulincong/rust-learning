use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;

fn main() {
    let url = "mysql://root:wu188664@localhost:3306/rust";
    let pool = Pool::new(url).unwrap(); //获取连接池
    let mut conn = pool.get_conn().unwrap(); //获取链接
    conn.query_iter("select * from student").unwrap()
        .for_each(|row|{
            // let r:(i32, String, i32, String, NaiveDate) = from_row(row.unwrap());
            // println!("{}, {}, {}, {:?}", r.0, r.1, r.2, r.3);
        });
}

