use alloc::collections::vec_deque::VecDeque;
use alloc::sync::Arc;
use core::fmt::{self, Write};
use spin::mutex::Mutex;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

// 输出缓冲区的大小
const CONSOLE_BUFFER_SIZE: usize = 256 * 10;


// 使用user_lib中的函数
use super::{read, write};
use lazy_static::*;

struct ConsoleBuffer(VecDeque<u8>);

// 这段代码的核心思想是创建一个全局可访问、线程安全的控制台缓冲区
// lazy_static是一个宏，使得静态变量可以在首次访问时初始化，而不是在编译时
lazy_static! {
    // static ref用于声明lazy_static变量
    // CONSOLE_BUFFER是一共全局可访问的静态变量，
    static ref CONSOLE_BUFFER: Arc<Mutex<ConsoleBuffer>> = {
        // 创建一个指定容量的双端队列，用于存储字节数据
        let buffer = VecDeque::<u8>::with_capacity(CONSOLE_BUFFER_SIZE);
        // 使用Mutex来保护缓冲区的并发安全
        // Arc<Mutex<ConsoleBuffer>>：ConsoleBuffer->Mutex->Arc
        Arc::new(Mutex::new(ConsoleBuffer(buffer)))
    };
}

// 为ConsoleBuffer提供方法实现
impl ConsoleBuffer {
    // 通过write将缓冲区中的数据一次性写入标准输出
    fn flush(&mut self) -> isize {
        // self.0引用了ConsoleBuffer中的VecDeque<u8>
        // make_contiguous将双端队列中的数据移动到一个连续块中并返回一个指向该块的切片
        let s: &[u8] = self.0.make_contiguous();

        // 调用write，输出字节
        let ret = write(STDOUT, s);

        // 清空缓冲区
        self.0.clear();
        ret
    }
}

impl Write for ConsoleBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes().iter() {
            self.0.push_back(*c);
            // 只有当遇到换行符或缓冲区满时，flush才会被调用，将内容输出到屏幕
            if (*c == b'\n' || self.0.len() == CONSOLE_BUFFER_SIZE) && -1 == self.flush() {
                return Err(fmt::Error);
            }
        }
        Ok(())
    }
}

#[allow(unused)]
pub fn print(args: fmt::Arguments) {
    let mut buf = CONSOLE_BUFFER.lock();
    // buf.write_fmt(args).unwrap();
    // BUG FIX: 关闭 stdout 后，本函数不能触发 panic，否则会造成死锁
    buf.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}

pub fn flush() {
    let mut buf = CONSOLE_BUFFER.lock();
    buf.flush();
}
