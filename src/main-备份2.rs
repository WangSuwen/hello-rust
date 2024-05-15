
extern crate winapi;
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput, KEYEVENTF_KEYUP};
const VK_A: u16 = 0x41;  // "A"键的虚拟键码

// 定义一个按下键的函数
fn press_key(vk: u16) {
    // 创建一个INPUT结构体来存储键盘输入数据
    let mut input = INPUT {
        // 设置输入类型为键盘输入
        type_: INPUT_KEYBOARD,
        // 将联合体初始化为零
        u: unsafe { std::mem::zeroed() },
    };

    // 使用不安全代码来操作KEYBDINPUT结构体
    unsafe {
        // 将虚拟键码（vk）设置为输入参数
        *input.u.ki_mut() = KEYBDINPUT {
            // 设置虚拟键码（vk）
            wVk: vk,
            // 将扫描码设置为0（不使用）
            wScan: 0,
            // 将标志设置为0（无标志）
            dwFlags: 0,
            // 将时间设置为0（不使用）
            time: 0,
            // 将额外信息设置为0（不使用）
            dwExtraInfo: 0usize,
        };
        // 将输入事件发送到系统
        SendInput(
            // cInputs：发送的输入事件数量（这里是1）
            1,
            // lpInputs：INPUT结构体数组的指针（这里是一个单独的INPUT结构体）
            &mut input,
            // cbSize：INPUT结构体的字节数（转换为i32）
            std::mem::size_of::<INPUT>() as i32
        );
    }
}

// 定义一个释放键的函数
fn release_key(vk: u16) {
    // 创建一个INPUT结构体来存储键盘输入数据
    let mut input = INPUT {
        // 设置输入类型为键盘输入
        type_: INPUT_KEYBOARD,
        // 将联合体初始化为零
        u: unsafe { std::mem::zeroed() },
    };

    // 使用不安全代码来操作KEYBDINPUT结构体
    unsafe {
        // 将虚拟键码（vk）设置为输入参数
        *input.u.ki_mut() = KEYBDINPUT {
            // 设置虚拟键码（vk）
            wVk: vk,
            // 将扫描码设置为0（不使用）
            wScan: 0,
            // 将标志设置为KEYEVENTF_KEYUP（键释放）
            dwFlags: KEYEVENTF_KEYUP,
            // 将时间设置为0（不使用）
            time: 0,
            // 将额外信息设置为0（不使用）
            dwExtraInfo: 0usize,
        };
        // 将输入事件发送到系统
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}
// #[test]
fn main() {
    press_key(VK_A);
    release_key(VK_A);
}