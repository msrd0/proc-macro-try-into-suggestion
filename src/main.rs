use derive::DeriveMacro;

#[derive(DeriveMacro)]
#[custom = "{ -1 as i32 }"]
struct FooBar;

fn main() {}
