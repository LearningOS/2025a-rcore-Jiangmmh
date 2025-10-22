//! SBI console driver, for text output
use crate::sbi::console_putchar;

// Write是一个trait，需要实现write_str方法，提供了write_char和write_fmt方法
use core::fmt::{self, Write};   

struct Stdout;

// 为Stdout结构体实现Write trait
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize); // 调用对sbi字符输出的封装
        }
        Ok(())
    }
}

// 注意write_fmt接收的参数是fmt::Arguments
pub fn print(args: fmt::Arguments) {
    // 遍历args中的格式字符串和参数，执行实际的格式化操作，并将格式化后的字符串写入底层目标
    Stdout.write_fmt(args).unwrap();
}

/// Print! to the host console using the format string and arguments.
#[macro_export]         // 用于将定义的宏导出到crate外部，让其它crate可以使用
macro_rules! print {    // 用于定义声明式宏，宏在编译时运行，将输入转换为正常的Rust代码
    // $fmt: literal：表示用变量fmt匹配一个字符串字面量
    // $(...)?：表示匹配一个可选组，?表示括号中的内容是可选的，例如print!("hello")就没有多余的参数
    // , $($arg: tt)+：表示用变量arg匹配逗号以及之后的重复组，+表示逗号后的内容可以出现一次或多次，用于匹配多个参数，tt匹配任意Rust语法片段
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // format_args!用于将格式字符串与后面的所有参数打包成一个fmt::Arguments实例
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

/// Println! to the host console using the format string and arguments.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}
