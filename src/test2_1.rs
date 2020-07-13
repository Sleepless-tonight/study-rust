use std::io;

// 获取输入并输出
pub(crate) fn test1() {
    println!("Guess the nuber!");
    println!("Please input your guess!");

    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed：{}", guess);
}