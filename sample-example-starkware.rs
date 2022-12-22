use starkware_rs::{
    proof_system::{
        create_random_proof,
        prepare_verifying_key,
        verify_proof,
    },
    Hash256,
    MerkleTree,
    MessageDigest,
    Stark,
};
use rand::rngs::OsRng;

fn main() {
    // Define the statement we want to prove
    let statement = b"This is the statement we want to prove";

    // Define the circuit that represents the statement
    let mut stark = Stark::new();
    stark.add_input(statement);
    stark.add_constraints();

    // Generate a random proof
    let mut rng = OsRng::new().unwrap();
    let (proof, vk) = create_random_proof(stark, &mut rng).unwrap();

    // Verify the proof
    let pvk = prepare_verifying_key(&vk);
    assert!(verify_proof(&pvk, &proof, &[]).unwrap());
}
