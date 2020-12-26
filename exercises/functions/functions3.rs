// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    let x: i32 = 5;
    let maybe: bool = true;
    call_me(x, maybe);
}

fn call_me(num: i32, maybe: bool) {
    if maybe {
        for i in 0..num {
            println!("Ring! Call number {}", i + 1);
        }
    } else {
        println!("Maybe not");
    }

}
