use std::println;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    println!("HELLO WORLD!!!!")
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    println!("I'm a module... {}", dt);
}
