use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use succinctactoe_lib::{play_tic_tac_toe, GameData};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SUCCINCTACTOE_ELF: &[u8] = include_elf!("succinctactoe-program");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    let game_data: GameData = play_tic_tac_toe();

    println!("Succintactoe SP1 Proof Generator");
    println!("-------------------------------");
    println!(
        "Game data: Username={}, Player Score={}, Computer Score={}",
        game_data.username, game_data.player_score, game_data.computer_score
    );

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&game_data.username);
    stdin.write(&game_data.player_score);
    stdin.write(&game_data.computer_score);

    // Execute the program
    let (mut public_values, report) = client.execute(SUCCINCTACTOE_ELF, &stdin).run().unwrap();

    // Record the number of cycles executed.
    println!(
        "Program executed successfully with number of cycles: {}",
        report.total_instruction_count()
    );

    // Read values
    let stored_username = public_values.read::<String>();
    let stored_user_score = public_values.read::<u32>();
    let stored_computer_score = public_values.read::<u32>();
    let result = public_values.read::<bool>();

    // Print results
    println!("Execution results:");
    println!("- Username: {}", stored_username);
    println!("- Player Score: {}", stored_user_score);
    println!("- Computer Score: {}", stored_computer_score);
    println!("- Result: {}", result);

    // Setup the program for proving.
    let (pk, vk) = client.setup(SUCCINCTACTOE_ELF);

    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");

    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    // Save proof
    let proof_path = "fibonacci_proof.bin";
    proof.save(proof_path).expect("Failed to save proof");
    println!("Proof saved to: {}", proof_path);

    println!("\nSP1 ZK Proof generation complete!");
}
