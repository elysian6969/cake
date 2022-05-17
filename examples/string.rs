use frosting::str;

fn main() {
    // prints 'o'
    println!("as_ptr_end: {:?}", unsafe {
        *str::as_ptr_end("hello") as char
    });

    let string = "hello world!";

    // prints "world"
    println!("conjoin: {:?}", unsafe {
        let start = string.as_ptr().add(6);
        let end = string.as_ptr().add(11);

        str::conjoin(start, end)
    });
}
