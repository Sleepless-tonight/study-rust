use std::io;

// 1、您只能对一个特定范围内的特定数据进行一个可变引用。
// 2、当我们拥有不变的借用时，我们也不能拥有可变的借用。
// 3、请注意，引用的范围从引入它的地方开始，一直持续到最后一次使用该引用。
pub(crate) fn test4_2() {

    let mut s = String::from("hello");
    let r1 = &mut s;
    s.push_str(",hello1");
    s.push_str(",hello2");
    println!("{}", s);
    // 只要注掉下面对 r1 的使用，此上下5行的代码就是可运行的。
    println!("{}", r1);

}