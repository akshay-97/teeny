pub struct Post{
    state : Option<Box<dyn State>>,
    content : String,
}

impl Post{
    pub fn new() -> Post{
        Post {
            state : Some(Box::new(Draft {})),
            content : String::new(),
        }
    }
    
    pub fn add_text(&mut self, content : &str) -> (){
        self.content.push_str(content);
    }
    
    pub fn get_content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) -> (){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) -> (){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self : Box<Self>) -> Box<dyn State>;
    fn approve(self : Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, content: &'a Post) -> &'a str; // deref coercion, why cant we be explicit here
}
struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
    fn request_review(self : Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
    
    fn content<'a>(&self, content:&'a Post) -> &'a str{
        ""
    }
}

impl State for PendingReview {
    fn request_review(self : Box<Self>) -> Box<dyn State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State>{
        Box::new(Published{})
    }

    fn content<'a>(&self, content:&'a Post) -> &'a str{
        ""
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, content:&'a Post) -> &'a str{
        &content.content
    }
}