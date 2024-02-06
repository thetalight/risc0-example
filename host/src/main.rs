use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// Compute the product a*b inside the zkVM
pub fn multiply(a: u64, b: u64) -> (Receipt, u64) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, METHOD_ELF).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    println!("I know the factors of {}, and I can prove it!", c);

    (receipt, c)
}

fn main() {
    // Pick two numbers
    let (receipt, _) = multiply(17, 23);

    // Verify receipt, panic if it's wrong
    receipt.verify(METHOD_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
