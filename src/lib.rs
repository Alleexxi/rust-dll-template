unsafe fn main(_hinst: usize) {
    println!("Hello, world!");
}

#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32, _reserved: *mut ()) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || unsafe { main(hinst) });
    }

    1
}
