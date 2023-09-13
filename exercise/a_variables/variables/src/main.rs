const STARTING_MISSILES: i32 = 8;
const READY_MISSILES: i32 = 2;
fn main() {
    // let (missiles, ready) = (8, 2);
    // let mut missiles = STARTING_MISSILES;
    // let ready:i32 = READY_MISSILES;
    let (missiles, ready) = (STARTING_MISSILES, READY_MISSILES);
    println!("Firing {} of my {} missiles...", ready, missiles);

    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
