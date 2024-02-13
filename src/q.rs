#[derive(Debug)]
pub struct OurQ<T>{
    value : T,
    next : Option<Box<OurQ<T>>>
}

impl <T> OurQ<T>{
    pub fn new(val :T) -> OurQ<T>{
        OurQ {
            value : val,
            next : None
        }

    }
}
#[derive(Debug)]
pub struct Queue<T> {
    pub endNode : Option<Box<OurQ<T>>>
}

impl <T: std::fmt::Debug> Queue<T>{
    pub fn add(&mut self, val :T) -> (){
        let mut ptr = &mut self.endNode;
        match ptr {
            None => {
                let new_node = OurQ::new(val);
                self.endNode = Some(Box::new(new_node));
            },
            Some(boxV) => {
                let mut start = boxV;
                loop {
                    if let Some(_) = &(*start).next{
                        start = (*start).next.as_mut().unwrap();
                    }
                    else{
                        break;
                    }
                }
                let new_node = OurQ::new(val);
                (*start).next= Some(Box::new(new_node));
            }
        }
    }

    pub fn remove(&mut self) -> Option<T>{
        let ptr = std::mem::take(&mut self.endNode);
        match ptr {
            None => None,
            Some (boxV) => {
                let node = *boxV;
                self.endNode = node.next;
                Some(node.value)
            }
        }
    }

    pub fn display(&self) -> (){
        let mut ptr = &self.endNode;
        let mut count = 0;
        loop{
            match ptr{
                None => {        
                    print!("is my q empty\n");
                    break
                },
                Some (boxV) => {
                    count = count +1;
                    print!("value indexes : {} \n", count);
                    print!("Value is : {:?} \n", (*boxV).value);
                    ptr = &(*boxV).next
                }
            }
        }
    }
}