// How to publish to Crates.io and set everything up:
// https://youtu.be/4TI153PIEDQ?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&t=525
// Anything uploaded to Crates.io is permanent. But can e.g. make specific versions un-downloadable.

// use art::kinds::PrimaryColor;
// use art::utils::mix;

// We can import like below, instead of like above, because we re-exported these in lib.rs.
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
