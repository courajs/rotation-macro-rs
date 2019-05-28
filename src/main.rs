macro_rules! rotations {
    ($( $x:expr ),*) => {
        $( println!("{:?}", $x); )*
    }
}
fn main() {
    rotations!(1, "hey");
}
