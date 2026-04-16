use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct XY {
    x: i32,
    y: *const i32,
    _pin: PhantomPinned,
}

impl XY {
    fn new() -> Pin<Box<Self>> {
        let mut boxed_struct = Box::pin(XY {
            x: 5,
            y: std::ptr::null(),
            _pin: PhantomPinned,
        });

        let ptr: *const i32 = &boxed_struct.x;

        unsafe {
            let mut_data = Pin::as_mut(&mut boxed_struct);
            Pin::get_unchecked_mut(mut_data).y = ptr;
        }

        boxed_struct
    }
}

fn main() {
    let struct_1 = XY::new();
    println!("struct 1 : {:?}", struct_1);

    let struct_2 = *struct_1;
    println!("struct_2 : {:?}", struct_2);
}
