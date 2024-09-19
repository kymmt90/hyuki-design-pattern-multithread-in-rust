mod data;
mod host;

use std::{thread, time::Duration};

fn main() {
    println!("main BEGIN");

    let data1 = host::request(10, 'A');
    let data2 = host::request(20, 'B');
    let data3 = host::request(30, 'C');

    println!("main otherJob BEGIN");
    thread::sleep(Duration::from_secs(2));
    println!("main otherJob END");

    println!("data1 = {}", data1.get_content());
    println!("data2 = {}", data2.get_content());
    println!("data3 = {}", data3.get_content());

    println!("main END");
}
