fn main() {
    let mut important_data = "Hello, World!".to_string();

    important_data = calc_data(important_data); //値の所有権を渡して返却．
    println!("{}", important_data)
}

fn calc_data(data: String) -> String {
    println!("{}", data);
    data
}
