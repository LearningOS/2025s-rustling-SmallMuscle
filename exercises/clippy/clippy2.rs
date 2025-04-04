// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res:u64 = 42;
    let mut option = Some(12);
    while let Some(x) = option.take() {
        res += x;
    }
    println!("{}", res);
}
