use rand::Rng;
use std::cmp::Ordering;
use std::io; //stdライブラリの中のioモジュールの中のstdin関数

fn main() {
    println!("Guess the number!"); //数を当ててみ

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess."); //予想書いてねはあと

    let mut guess = String::new();

    io::stdin() //std::io::stdinでもおｋ
        .read_line(&mut guess) //.がメソッドを表す
        .expect("Failed to read line"); //行の読み込みに失敗しました。

    println!("You guessed: {}", guess); //次のように予想しました：{}

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
