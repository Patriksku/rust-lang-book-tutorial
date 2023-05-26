/*
This state pattern is easy to extend. You can e.g. add another method reject() which will take a post
that is under review, back to a draft state.

A downside of this pattern is that some states are coupled to each other. If we for e.g. want to add a new state
"schedule" between PendingReview and Publish, we would need to update PendingReview, so that we switch to the new state
instead.
Another downside is duplication.

So by implementing the State pattern exactly as in OOP, we are not taking full advantage of Rust.
 */
use state_design_pattern_oop::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
