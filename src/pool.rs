use std::cell::RefCell;
use::std::thread;
use::std::sync::Mutex;
use std::sync::mpsc;
use::std::sync::Arc;

/*
thread pool creates channel
thread pool pushed FnOnce to sender

worker spawns thread
worker thread spawns infinite loop
{
     tries to read from receiver
}
*/
pub struct ThreadPool<F>
where
    F : FnOnce() + Send
{
    ts : Vec<Worker>,
    sender : Option<mpsc::Sender<F>>
}

struct Worker{
    id : usize,
    wt : Option<thread::JoinHandle<()>>,
}
impl Worker{
    pub fn new<F : FnOnce() + Send + 'static> (id : usize, rxc : Arc<Mutex<mpsc::Receiver<F>>>) -> Worker{
        let result = thread::spawn(move ||{
            loop{
                match (*rxc).lock() {
                    Ok(recv) =>{
                        match(recv.try_recv()){
                            // check for broken channel error
                            Err(mpsc::TryRecvError::Disconnected) => {print!("pipeline broken"); break;},
                            Err(mpsc::TryRecvError::Empty) => {},//print!("empty channel ... trying again"),
                            Ok(job) => {
                                print!("task {} picked up \n", id);
                                drop(recv);
                                job();
                                print!("task {} done \n", id);
                            }
                        }
                    }
                    Err(e)=> print!("lock Busy")
                }
            }
        });
        Worker { id, wt: Some(result)}
    }

}
impl <F> ThreadPool<F>
where
    F: FnOnce() + Send + 'static
{
    pub fn new(size : usize) -> ThreadPool<F>{
        let (tx, rx) = mpsc::channel();
        let mut ts = Vec::with_capacity(size);
        let recv = Arc::new(Mutex::new(rx));
        for i in 0..size{
            ts.push(Worker::new(i, recv.clone()));
        }
        
        ThreadPool{
            ts,
            sender : Some(tx)
        }
    }

    pub fn execute(&mut self, f: F) -> Result<(), mpsc::SendError<F>>
    {
        self.sender.as_ref().unwrap().send(f)
    }
}


impl<F: Send + FnOnce()> Drop for ThreadPool<F> {
    fn drop(&mut self) {
        print!("shutting down server\n");
        drop(self.sender.take());
        for (wt) in &mut self.ts  {
            print!("shutting worker {} \n", wt.id);
            if let Some(a) = wt.wt.take(){
                a.join().unwrap();
            }
            
        }
    }
}