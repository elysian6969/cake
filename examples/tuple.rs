use cake::tuple;

fn foo<T>(tuple: T)
where
    T: tuple::Index<1, Element = bool>,
{
    println!("foo: {:?}", tuple::get(&tuple));
}

fn main() {
    foo((1, true, "hi"));
    foo((false, false, 69.420));
}
