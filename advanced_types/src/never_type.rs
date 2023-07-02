/*
The never type '!'.

 */
pub fn main() {
    let game_in_progress = true;

    // We see that guess is of type u32... So how is the Err(_) arm allowed bellow?
    // Because 'continue' has a never type. Because of this, it can never return, 
    // therefore the only possible type returned is u32. So 'continue' here moves
    // the control back to the top of the loop.
    while game_in_progress {
        let guess: u32 = match "guess".to_string().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}