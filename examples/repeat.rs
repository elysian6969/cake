use cake::num;

fn main() {
    println!("{:0X?}", num::repeat_u8::<u64>(0x80));
    println!("{:0X?}", num::repeat_u16::<u64>(0x8080));
    println!("{:0X?}", num::repeat_u32::<u64>(0x80808080));
    println!("{:0X?}", num::repeat_u64::<u64>(0x8080808080808080));
}
