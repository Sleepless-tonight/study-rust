use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

// 获取输入并输出
pub(crate) fn test1() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}