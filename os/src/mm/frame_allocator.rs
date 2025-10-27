//! Implementation of [`FrameAllocator`] which
//! controls all the frames in the operating system.

use super::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use lazy_static::*;

/// tracker for physical page frame allocation and deallocation
pub struct FrameTracker {
    /// physical page number
    pub ppn: PhysPageNum,
}

// 这里使用FrameTracker来包装物理页面是为了在申请物理页面时插入清除代码，并实现Drop trait，实现自动回收页面
impl FrameTracker {
    /// Create a new FrameTracker
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {                  // 清空物理页号对应的页面
            *i = 0;
        }
        Self { ppn }                            
    }
}

// 实现Debug方便输出
impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

// 实现Drop，在FrameTracker生命周期结束后，将物理页面收回FrameAllocator
impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

// 定义号接口
trait FrameAllocator {
    fn new() -> Self;                           // 创建一个分配器
    fn alloc(&mut self) -> Option<PhysPageNum>; // 分配一个物理页面
    fn dealloc(&mut self, ppn: PhysPageNum);    // 释放一个物理页面
}
/// an implementation for frame allocator
pub struct StackFrameAllocator {
    current: usize,                     // [current, end)保存从未被分配出去的物理页面号
    end: usize,
    recycled: Vec<usize>,               // recycled保存被回收的物理页面号
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {    
        self.current = l.0;     // ekernel
        self.end = r.0;         // MEMORY_END
        // trace!("last {} Physical Frames.", self.end - self.current);
    }
}
impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {    // 先尝试从recycled中获取页面，如果recycled为空pop会返回None
            Some(ppn.into())
        } else if self.current == self.end {        // 如果区间为空，返回None
            None
        } else {
            self.current += 1;                      // 否则从未分配区间中取一个物理页面
            Some((self.current - 1).into())
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        // 该页面一定被分配出去过，并且还未被回收
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);        // 放入回收列表
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    /// frame allocator instance through lazy_static!
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}

// 真正对外的三个函数接口：初始化、获取物理页面、释放物理页面
/// initiate the frame allocator using `ekernel` and `MEMORY_END`
pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(ekernel as usize).ceil(),    // ekernel，内核的末尾，即可用内存的起始地址
        PhysAddr::from(MEMORY_END).floor(),         // 内存结尾地址
    );
}

/// Allocate a physical page frame in FrameTracker style
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)         // 获取的是PhysPageNum，这里将其转换为FrameTracker
}

/// Deallocate a physical page frame with a given ppn
pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
