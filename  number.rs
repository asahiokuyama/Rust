use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let seacret_number = rand::thread_rng().gen_range(1,101);//乱数生成
    println!("The seacret number is: {}", seacret_number);

    loop {
    	println!("Please input your guess");

	    let mut guess = String::new();//可変変数をString型で定義

	    io::stdin().read_line(&mut guess).expect("error");//guessに入力

	    // let guess: u32 = guess.trim().parse().expect("type number");
	    //trimで両端の文字を削除
	    //parseで型推論により型変形

	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_)	=> continue,
	    };
	    //parseは型列挙でOKとErrを返す。Okにnumを入れ、guessに返す

	    println!("you guess: {}",guess);

	    match guess.cmp(&seacret_number) {//比較する
	    	Ordering::Less => println!("too small"),
	    	Ordering::Greater => println!("too big"),
	    	Ordering::Equal =>{
	    		println!("you win");
	    		break;
	    	}
    }				
    }
    
}
