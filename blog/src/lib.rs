pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let current = self.content.clone();
        println!("{}", current);

        if let Some(s) = self.state.take() {
            let text: &str = s.update_content(text, &current[..]);
            self.state = Some(s);
            self.content = String::from(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn reject(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn update_content<'a>(&self, content: &'a str, current: &'a str) -> &'a str {
        current
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }

    fn update_content<'a>(&self, content: &'a str, current: &'a str) -> &'a str {
        content
    }
}

struct PendingReview {
    approvals: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {

        match self.approvals {
            0 => Box::new(PendingReview {approvals:1}),
            1 => Box::new(Published {}),
            _ => Box::new(Published {}),
        }
    }

    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}
