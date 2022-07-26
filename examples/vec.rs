use cake::fixed::{FixedString, FixedVec};

fn main() {
    let mut vec = FixedVec::<_, 3>::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{vec:?}");
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{vec:?}");

    let mut string = FixedString::<30>::new();

    string.push('h');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');

    println!("{string}");
    println!("{string:?}");

    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());

    println!("{string}");
    println!("{string:?}");
}
