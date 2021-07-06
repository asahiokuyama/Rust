use std::io;

fn main() {
    // let x = plus(9);

    // println!("value is {}", x);

    // if x == 10 {
    // 	println!("x = 10");
    // }

    // let a = [10,20,30,40,50];

    // for element in a.iter() {
    // 	println!("value = {}", element);
    // }

    // for number in (1..4).rev() {
    // 	println!("{}", number);
    // }
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("err");
    let c: f64 = temp.trim().parse().expect("err");
    let f = trans(c);

    println!("{}", c);
    println!("{}", f);

}

// fn plus(x: i32) -> i32{
// 	x+1
// }

fn trans(x: f64) -> f64{
	(x*1.8) + 32.0
}