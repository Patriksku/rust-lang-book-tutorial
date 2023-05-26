pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // This function is not part of the state pattern -> We do not care about the state
    // when calling this function.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // We want a reference to the state object. We do not need to own it as below, as we are not going
        // to change it. Also, with unwrap(), we get back a &Box<Option<dyn State>>, but because of
        // deref coercion, we will be able to call .content() on the state.
        self.state.as_ref().unwrap().content(self)
    }

    // As we can see, this method is always going to be the same, no matter what state we currently are in,
    // because each state is reponsible for its own rules, that govern what happens when we call request_review()
    // on that particular state object.
    pub fn request_review(&mut self) {
        // This is why we needed to use Option<> for the state. We take ownership of the value (move it)
        // from the Option<>, and set our state to the result of the request_review() call.
        // So we are moving the state outside of our Post struct, and Rust does not allow empty fields in Structs.
        // That is why we need to use Option<>.
        // The call will be different, based on whatever the state currently is.
        //
        // Therefore, self.state is None. So we reassign it to Some().
        // request_review() will consume the old state, invalidating it, and return a new state.
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review())
        }
    }

    // Same as above
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }
}

pub trait State {
    // We take ownership of a Box containing self, but we don't use self.
    // So we are essentially invalidating the old state, and returning a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // If we call request_review() while already being in PendingReview state,
    // then we do not want to do anything, so we just return the state we are in.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Function takes two references, and returns a reference, so we need to tell the compiler
    // the relationship between these 3 references. Here, the lifetime of our return type, is tied
    // to the lifetime of post, so we tell the compiler this.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
