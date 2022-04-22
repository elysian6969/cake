use frosting::tuple;
use frosting::tuple::TupleIndex;

fn foo<T>(tuple: T)
where
    T: TupleIndex<1, Element = bool>,
{
    println!("foo: {:?}", tuple::get(&tuple));
}

fn main() {
    foo((1, true, "hi"));
    foo((false, false, 69.420));
}
