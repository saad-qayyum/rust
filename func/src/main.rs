fn main() {
    let x = get_fibonacci(20);
    println!("Hello, world! {x}");
}

fn get_fibonacci(num: usize)->i32{
    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);

    for elem in 2..num {
        vec.push(vec[elem-1]+vec[elem-2])
    }
    return vec[num-1];
}
