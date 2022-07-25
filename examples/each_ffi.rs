use cake::mem::UninitArray;

extern "C" {
    fn scanf(fmt: *const u8, ...) -> i32;
}

fn main() {
    let mut vec = UninitArray::<i32, 3>::uninit();
    let [x, y, z] = UninitArray::each_mut_ptr(&mut vec);

    unsafe {
        scanf("%d %d %d\0".as_ptr(), x, y, z);
    }

    let vec = unsafe { UninitArray::assume_init(vec) };

    println!("{vec:?}");
}
