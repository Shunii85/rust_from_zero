fn main() {
    println!("Hello, world!");

    // sample codes
    let x: i32 = 10;
    let y = 20;

    let z = mul(x, y);
    println!("z = {z}");
}

fn mul(x: i32, y: i32) -> i32 {
    x*y
}
