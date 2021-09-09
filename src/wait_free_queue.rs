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
        data.shrink_to_fit();
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

    /// Returns the next element of the stored data according to the global header.
    /// Might be called concurrently from multiple threads.
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

    /// Returns a slice of internal data. Might be called inside a thread, safe operation.
    pub fn get_data(&self) -> &[T] {
        unsafe {
            let data = std::slice::from_raw_parts(self.ptr, self.size);
            data
        }
    }

    /// Reads internal data exhaustively. The struct example it has been called on is consumed.
    /// Returns Err if the struct example has living copies, otherwise returns Ok(data)
    pub fn exhaustive_read(mut self) -> Result<Vec<T>, String>{
        if self.n_copies.load(Ordering::Relaxed) != 0{
            Err("Exhaustive read has been called while other copies exist!".to_string())
        }
        else{
            let data = unsafe {Vec::from_raw_parts(self.ptr, self.size, self.size)};
            self.ptr = ptr::null_mut();
            Ok(data)
        }

    }
}

impl<T> Drop for WaitFreeQueue<T>{
    fn drop(&mut self){
        if self.n_copies.load(Ordering::Relaxed) != 0{
            self.n_copies.fetch_sub(1, Ordering::Relaxed);
        }
        else {
            if !self.ptr.is_null() {
                unsafe {
                    let _vec = Vec::from_raw_parts(self.ptr, self.size, self.size);}
            }
        }
    }
}

