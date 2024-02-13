use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod q;
mod tp;
pub fn main(){
    // print!("hello am main\n");
    // let msg_impl = MessageTracker::new();
    // let mut limit_impl = LimitTracker::new(&msg_impl, 32);
    // limit_impl.set_value(28);
    //print!("{}", msg_impl.sentMessages.borrow()[0])
    // let mut qe = q::Queue{endNode : None};
    // qe.add(123);
    // qe.add(234);
    // qe.add(456);
    // qe.display();

    // let r = qe.remove();
    // if let Some(w) = r{
    //     print!("Removed element is {} \n\n", w);
    // }

    // qe.add(001);
    // qe.display();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10000));
        let val = String::from("hi");
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(10000));
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let rec_again = rx.recv();
    match rec_again {
        Err(err) => print!("no messages or closed tx {} \n", err),
        Ok(_) => print!("there is some message")
    }

}
pub trait Messenger {
    fn send(&self, msg : &str) -> ();
}

pub struct MessageTracker {
    sentMessages : RefCell<Vec<String>>,
}

impl MessageTracker {
    fn new() -> MessageTracker{
        MessageTracker { sentMessages: RefCell::new(vec![]), }
    }
}
impl Messenger for MessageTracker{
    fn send(&self, msg : &str){
        let copymut = self.sentMessages.borrow_mut();
        self.sentMessages.borrow_mut().push(String::from(msg))
    }
}
// Limit Tracker implementation
pub struct LimitTracker <'a, T : Messenger> {
    messenger : &'a T,
    maxL : usize,
    currL : usize,
}

impl <'a, T> LimitTracker<'a, T>
where T : Messenger,
{
    pub fn new(msgg : &T, max : usize) -> LimitTracker<T> {
        LimitTracker {
            currL : 0,
            messenger : msgg,
            maxL : max,
        }
    }
    
    pub fn set_value(&mut self, currL : usize) -> () {
        self.currL = currL;

        if self.currL > self.maxL{
            print!("overflow !!!\n");
            self.messenger.send("overflow alert\n")
        }
        else {
            print!("limits held\n");
            self.messenger.send("within limits\n")
        }
    }
}


