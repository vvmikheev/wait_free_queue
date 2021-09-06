use std::sync::atomic::{AtomicUsize, AtomicIsize, AtomicBool, Ordering};
use std::sync::Arc;
use std::mem;
use std::ptr;


pub struct WaitFreeQueue<T>{
    header: Arc<AtomicIsize>,
    size: usize,
    ptr: *mut T,
    n_copies: Arc<AtomicUsize>,
    flag: Arc<AtomicBool>,
    pub id: usize
}


unsafe impl<T> Send for WaitFreeQueue<T> {}
unsafe impl<T> Sync for WaitFreeQueue<T> {}


impl<T> Clone for WaitFreeQueue<T>{
    fn clone(&self) -> WaitFreeQueue<T>{
        let a = self.n_copies.fetch_add(1, Ordering::Relaxed);
        WaitFreeQueue{
            header: self.header.clone(),
            size: self.size,
            ptr: self.ptr.clone(),
            n_copies: self.n_copies.clone(),
            flag: self.flag.clone(),
            id: a + 1
        }
    }
}


impl<T> WaitFreeQueue<T>{

    pub fn new(mut data: Vec<T>) -> Self {
        let size = data.len();
        let ptr = data.as_mut_ptr();
        mem::forget(data);

        WaitFreeQueue{
            header: Arc::new(AtomicIsize::new(0)),
            size,
            ptr,
            n_copies: Arc::new(AtomicUsize::new(0)),
            flag: Arc::new(AtomicBool::new(false)),
            id: 0
        }
    }

    pub fn get_next_mut(&mut self) -> Option<&mut T>{
        let offset = self.header.fetch_add(1, Ordering::Relaxed);
        if offset >= self.size as isize {
            None
        }
        else {
            unsafe {
                Some(& mut *self.ptr.offset(offset))
            }
        }
    }

    /// Returns the currents state of the data being stored. All elements T are copied
    /// regardless of whether T is Copy. If T is not Copy, access to both elements might lead to
    /// memory safety problems.
    pub unsafe fn get_current_state(&self) -> Vec<T> {
        let mut ans = Vec::with_capacity(self.size);

        for i in 0..self.size{
            let elem = ptr::read(self.ptr.offset(i as isize));
            ans.push(elem);
        }

        ans
    }
}

impl<T> Drop for WaitFreeQueue<T>{
    fn drop(&mut self){
        if self.n_copies.load(Ordering::Relaxed) != 0{
            self.n_copies.fetch_sub(1, Ordering::Relaxed);
        }
        else {
            unsafe {
                let _vec = Vec::from_raw_parts(self.ptr, self.size, self.size);}
        }
    }
}

