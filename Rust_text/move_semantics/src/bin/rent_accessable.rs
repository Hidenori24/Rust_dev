fn main() {
    let important_data = "Hello, World!".to_string();

    calc_data(&important_data); //値参照
    println!("{}", important_data)
}

fn calc_data(data: &String) {
    println!("{}", data);
}
