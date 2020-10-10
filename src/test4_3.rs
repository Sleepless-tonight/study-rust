
// 切片
pub(crate) fn test4_2() {

    let mut s = String::from("hello world");

    // 表示“字符串切片”的类型写为&str
    let word = first_word(&s);
    // 我们曾经讨论过将字符串文字存储在二进制文件中。现在我们了解了切片，我们可以正确理解字符串文字了：
    // s2 这里的类型是&str：它是指向二进制文件的特定点的切片。这也是字符串文字不可变的原因。&str是不可变的参考。
    let s2 = "Hello, world!";
    println!("s2 the first word is: {}", first_word2(&s2));

    println!("the first word is: {}", word);

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];


    s.clear();

}

fn first_word(s: &String) -> &str {


    // 我们使用以下iter方法在字节数组上创建一个迭代器：
    // 我们将在第13章中更详细地讨论迭代器。到目前为止，我们知道这iter 是一个返回集合中每个元素并enumerate 包装结果iter并将其作为元组的一部分返回的方法。返回的元组的第一个元素enumerate是索引，第二个元素是对该元素的引用。这比自己计算索引要方便一些。
    // 因为该enumerate方法返回一个元组，所以我们可以使用模式来破坏该元组，就像Rust中的其他地方一样。因此，在for 循环中，我们指定一个模式，该模式具有i元组中的索引和元组&item 中的单个字节。因为我们从获得对元素的引用，所以.iter().enumerate()我们&在模式中使用。
    // 在for循环内部，我们使用字节文字语法搜索表示空间的字节。如果找到空间，则返回位置。否则，我们使用以下命令返回字符串的长度s.len()：
    // 因为我们需要String逐个元素地检查并检查一个值是否为空格，所以我们将String使用以下as_bytes方法将其转换为字节数组 ：
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 字符串切片作为参数
fn first_word2(s: &str) -> &str {


    // 我们使用以下iter方法在字节数组上创建一个迭代器：
    // 我们将在第13章中更详细地讨论迭代器。到目前为止，我们知道这iter 是一个返回集合中每个元素并enumerate 包装结果iter并将其作为元组的一部分返回的方法。返回的元组的第一个元素enumerate是索引，第二个元素是对该元素的引用。这比自己计算索引要方便一些。
    // 因为该enumerate方法返回一个元组，所以我们可以使用模式来破坏该元组，就像Rust中的其他地方一样。因此，在for 循环中，我们指定一个模式，该模式具有i元组中的索引和元组&item 中的单个字节。因为我们从获得对元素的引用，所以.iter().enumerate()我们&在模式中使用。
    // 在for循环内部，我们使用字节文字语法搜索表示空间的字节。如果找到空间，则返回位置。否则，我们使用以下命令返回字符串的长度s.len()：
    // 因为我们需要String逐个元素地检查并检查一个值是否为空格，所以我们将String使用以下as_bytes方法将其转换为字节数组 ：
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}