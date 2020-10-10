use std::io;
use rand::Rng;
/*
    1、添加为依赖
    2、Cargo.lock文件，它创建的第一次运行cargo build，现在是你的guessing_game目录。首次构建项目时，Cargo会找出符合条件的所有依赖项版本，然后将它们写入Cargo.lock文件。将来在构建项目时，Cargo将看到Cargo.lock文件存在并使用那里指定的版本，而不是再次进行所有确定版本的工作。这使您可以自动生成可复制的版本。

*/
pub fn test2() {
    println!("Guess the number!");

    // use rand::Rng。该Rng特征定义方法是随机数生成器
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}