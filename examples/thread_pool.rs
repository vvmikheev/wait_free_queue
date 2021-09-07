extern crate atomic_queue;

use atomic_queue::wait_free_queue::WaitFreeQueue;
use std::thread;

fn main(){

    let mut workload: Vec<Box<dyn Fn() -> isize>> = Vec::new();

    fn single_job(x: isize) -> Box<dyn Fn() -> isize>{
        let single_calculation= move || x.pow(2) - 2 * x + 1 ;
        Box::new(single_calculation)
    }

    let xs = vec![1; 1000];
    for x in xs{
        workload.push(single_job(x as isize))
    }

    let queue = WaitFreeQueue::new(workload);

    let n_threads = 4;
    let mut handler = Vec::with_capacity(n_threads);

    for _i in 0..n_threads{
        let mut thread_queue = queue.clone();
        handler.push(thread::spawn(move || {
            loop{
                let elem = thread_queue.get_next_mut();
                if elem.is_some(){
                    let job = elem.unwrap();
                    let result = job();
                    assert_eq!(result, 0);
                }
                else {
                    break
                }
            }
        }));
    }

    for handle in handler {
            match handle.join(){
                Ok(_) => {},
                Err(e) => {panic!(e)}
            };
        }
}