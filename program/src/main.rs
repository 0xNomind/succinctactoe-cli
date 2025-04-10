#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read an input to the program.
    let username = sp1_zkvm::io::read::<String>();
    let player_score = sp1_zkvm::io::read::<u32>();
    let computer_score = sp1_zkvm::io::read::<u32>();

    // Game result
    let game_result = player_score > computer_score;

    // Commit to the public values of the program.
    sp1_zkvm::io::commit(&username);
    sp1_zkvm::io::commit(&player_score);
    sp1_zkvm::io::commit(&computer_score);
    sp1_zkvm::io::commit(&game_result);
}
