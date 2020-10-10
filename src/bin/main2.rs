
/**
*/
fn main() {

    let some_u8_value = 18u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => {
            println!("none:{:?}", some_u8_value)
        },
    }

}
