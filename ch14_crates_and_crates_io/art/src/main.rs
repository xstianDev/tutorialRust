// Sin re-export
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// Con re-export
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}