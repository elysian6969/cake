use cake::vec::FixedVec;

fn main() {
    let mut vec = FixedVec::<_, 3>::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
}
