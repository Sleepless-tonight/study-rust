// mod test_gtk;
// mod test1_1;
// mod GPUI_Component_Test;
// mod test2_1;
// mod test2_2;
// mod test2_3;
// mod test4_3;
// use test2_2 as test;
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}
fn main() {
    hello_world();
    test_println();
    // test1_1::test1();
    // test2_1::test1();
    // test::test2();
    // test2_3::test3();
    // test_gtk::test_gtk();
    // GPUI_Component_Test::main();
}
fn hello_world() {
    println!("Hello, World!");
    println!("{} days", "31");
}
fn test_println() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:?}", peter);
    // Pretty print
    println!("{:#?}", peter);
    // 访问字段
    println!("Name: {}, Age: {}", peter.name, peter.age);
}