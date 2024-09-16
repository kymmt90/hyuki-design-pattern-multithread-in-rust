mod helper;
mod host;

fn main() {
    println!("main BEGIN");

    let handles = [(10, 'A'), (20, 'B'), (30, 'C')]
        .into_iter()
        .map(|(count, c)| host::request(count, c))
        .collect::<Vec<_>>();

    println!("main END");

    for h in handles {
        h.join().unwrap();
    }
}
