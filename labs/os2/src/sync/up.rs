//！ 这里主要是为了实现成员变量的可变
//！（不对整个类实例进行操作，而是单个成员变量）


use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T>{
    /// inner data
    inner: RefCell<T>
}

unsafe impl<T> Sync for UPSafeCell<T> {
    
}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self{
        Self { 
            inner: RefCell::new(value), 
        }
    }

    pub fn execlusive_access(&self) -> RefMut<'_, T>{
        // 显示的违背了Rust所有权
        // 获取了变量的可变引用
        self.inner.borrow_mut()
    }
}