const STARTING_AMOUNT: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_AMOUNT, READY_AMOUNT);
    // let hi = 2;
    // READY_AMOUNT = 4;
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
