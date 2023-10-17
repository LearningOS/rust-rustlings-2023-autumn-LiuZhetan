// clippy2.rs
// 
// Execute `q` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
