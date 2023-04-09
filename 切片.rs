fn main(){
    let s = String::from("boardcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{} = {}+{}", s, part1, part2);
    
}