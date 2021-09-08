pub mod wait_free_queue;

#[cfg(test)]
mod tests {
    use crate::wait_free_queue::WaitFreeQueue;
    use std::thread;

    #[test]
    fn data_read_test(){
        #[derive(Debug, PartialEq)]
        struct Test;  // trying to send smth which is not Copy or Clone

        let mut mydata = Vec::with_capacity(100);
        for _i in 0..100{
            let elem = Test;
            mydata.push(elem)
        }

        let n_threads: usize = 4;
        let mut handler = Vec::with_capacity(n_threads);

        let queue = WaitFreeQueue::new(mydata);

        for _i in 0..n_threads{

            let mut thread_queue = queue.clone();

            handler.push(
            thread::spawn(move || {
                loop{
                    let elem = thread_queue.get_next_mut();
                    if elem.is_some(){
                        assert_eq!(*elem.unwrap(), Test);
                    }
                    else {
                        break
                    }
                }

            }))
        }

        for handle in handler {
            match handle.join(){
                Ok(_) => {},
                Err(e) => {panic!(e)}
            };
        }

    }

    #[test]
    fn data_write_test() {
        let mydata = vec![1; 100];
        let n_threads: usize = 4;
        let mut handler = Vec::with_capacity(n_threads);

        let queue = WaitFreeQueue::new(mydata);

        for _i in 0..n_threads {
            let mut thread_queue = queue.clone();

            handler.push(
                thread::spawn(move || {
                    loop {
                        let elem = thread_queue.get_next_mut();
                        if elem.is_some() {
                            *(elem.unwrap()) += 1;
                        } else {
                            break
                        }
                    }
                }))
        }

        for handle in handler {
            match handle.join(){
                Ok(_) => {},
                Err(e) => {panic!(e)}
            };
        }
        let new_data = queue.get_data();

        assert_eq!(new_data, vec![2; 100].as_slice());

    }

}
