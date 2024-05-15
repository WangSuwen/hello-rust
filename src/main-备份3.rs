use enigo::{Enigo, Key};
use enigo::Keyboard;
//paste
fn main () {
    let enigo = Enigo::new(&Default::default());
    let kb = Enigo::key(&mut enigo, Key::Space, enigo::Direction::Press);
    /* if let Ok(val) = enigo {
        println!("{:?}", val);
        println!("{:?}", kb);
    } */
    // Keyboard::text(&mut kb, "Hello World");
    // enigo.key_down(Key::Space);
    // enigo.key_up(Key::Space);
    // enigo.key_click(Key::Layout('v'));
}
/* 
use udev::Device;
use std::fs::File;
use std::os::unix::io::AsRawFd;

//获取键码
const KEY_A: u32 = 30;

//模拟按下键
fn press_key(key: u32) {
    let device = Device::new_from_syspath(format!("/dev/input/event{}", 0)).unwrap();
    let mut file = File::open(device.devnode()).unwrap();
    let ev = input_event_new(EV_KEY, key, 1);
    let res = unsafe { write(file.as_raw_fd(), ev, size_of::<input_event>()) };
    if res < 0 {
        panic!("Error: {:?}", Error::last_os_error());
    }
}

//模拟释放键
fn release_key(key: u32) {
    let device = Device::new_from_syspath(format!("/dev/input/event{}", 0)).unwrap();
    let mut file = File::open(device.devnode()).unwrap();
    let ev = input_event_new(EV_KEY, key, 0);
    let res = unsafe { write(file.as_raw_fd(), ev, size_of::<input_event>()) };
    if res < 0 {
        panic!("Error: {:?}", Error::last_os_error());
    }
}
fn main () {
    press_key(KEY_A);
    release_key(KEY_A);
} */