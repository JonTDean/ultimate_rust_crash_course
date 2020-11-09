const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

// const (STARTING_MISSILES, READY_AMOUNT): (i32, i32 ) = (8, 2)

fn main() {
    // // Regular Assignment
    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;
    
    // // Pattern Assignment
    // I'm not sure but it seems
    // that the variables are,
    // both mutable by default
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    
    println!("Firing {} of my {} missiles...", ready, missiles);

    // // Base mutable variable redeclaration of values.
    // missiles = missiles - ready;
    // println!("{} missiles left",  missiles);

    println!("{} missiles left",  missiles = missiles - ready);

    let _unused_variable = 0;
}
