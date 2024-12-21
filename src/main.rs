use std::io;
use std::str;

fn main() {
    let mut s: String = String::new();
    let mut n: String = String::new();
    println!("Please type a line: ");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    println!("Please type a nuber of rows: ");
    io::stdin().read_line(&mut n).expect("Failed to read number of rows");
    let s: String = String::from(s.trim());
    let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => return println!("Rows must be integer!"),
    };
    println!("{}", convert(s, n))
}

fn convert(s: String, n: i32) -> String {
    if n == 1 {
        return s
    }

    let pattern: i32 = (2 * n) - 2;
    let mut fin_str: String = String::new();
    for i in 1..n+1 {
        let mut interval: i32 = if i == n {pattern} else {2 * n - 2 * i};
        let mut idx: i32 = i-1;
        while idx < s.len() as i32 {
            let chr = s.chars().nth(idx as usize).expect("Failed to get char");
            fin_str.push(chr);
            idx += interval;
            interval = if interval != pattern {pattern - interval} else {interval};
        }
    }
    return fin_str
}