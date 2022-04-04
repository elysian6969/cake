#[derive(Debug)]
#[repr(C)]
pub struct VPad<const N: usize>([usize; N]);

impl<const N: usize> VPad<N> {
    pub const fn new() -> Self {
        Self([0; N])
    }
}

extern "C" fn classes(this: *const ()) {
    debug!();
}

extern "C" fn hud_process_input(this: *const ()) {
    debug!();
}

extern "C" fn hud_update(this: *const ()) {
    debug!();
}

extern "C" fn activate_mouse(this: *const ()) {
    debug!();
}

extern "C" fn frame_stage_notify(this: *const ()) {
    debug!();
}

#[derive(Debug)]
#[repr(C)]
pub struct VTable {
    _pad0: VPad<8>,
    // offset 8
    classes: extern "C" fn(this: *const ()),
    _pad1: VPad<1>,
    // offset 10
    hud_process_input: extern "C" fn(this: *const ()),
    hud_update: extern "C" fn(this: *const ()),
    _pad2: VPad<4>,
    // offset 16
    activate_mouse: extern "C" fn(this: *const ()),
    _pad3: VPad<20>,
    // offset 37
    frame_stage_notify: extern "C" fn(this: *const ()),
}

impl VTable {
    pub const fn new() -> Self {
        Self {
            _pad0: VPad::new(),
            classes,
            _pad1: VPad::new(),
            hud_process_input,
            hud_update,
            _pad2: VPad::new(),
            activate_mouse,
            _pad3: VPad::new(),
            frame_stage_notify,
        }
    }
}

fn main() {
    println!("{:?}", function!());

    //println!("{:?}", from_ref(&5_u32));
    println!("{:?}", from_ref(&(5_u32,)));

    let vtable = VTable::new();
    let vtable_addr = ptr::addr_of!(vtable) as *const u8;
    let activate_mouse_addr = ptr::addr_of!(vtable.activate_mouse) as *const u8;

    println!("activate_mouse = {}", unsafe {
        activate_mouse_addr.offset_from(vtable_addr) / 8
    });

    let off = const {
        unsafe {
            let vtable = MaybeUninit::<VTable>::uninit();
            let vtable_addr = ptr::addr_of!(vtable) as *const u8;
            let activate_mouse_addr =
                ptr::addr_of!(vtable.assume_init_ref().activate_mouse) as *const u8;

            activate_mouse_addr.offset_from(vtable_addr) / 8
        }
    };

    println!("activate_mouse = {}", off);
}
