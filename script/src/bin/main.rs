use clap::Parser;
use dilithium::{ml_dsa_44, ml_dsa_65, ml_dsa_87};
use rand::Rng;
use sp1_sdk::{include_elf, ProverClient, SP1ProofMode, SP1Stdin};

pub const MLDSA_ELF: &[u8] = include_elf!("mldsa-program");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "2")]
    variant: u8,

    #[arg(long, default_value = "groth16")]
    proof_mode: String,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let client = ProverClient::from_env();

    let entropy = rand::rng().random::<[u8; 32]>();

    let (public_key, signature, message) = match args.variant {
        2 => {
            let keypair = ml_dsa_44::Keypair::generate(Some(&entropy));
            let message = b"Hello, SP1 ML-DSA verification!".to_vec();
            let signature = keypair.sign(&message, None, false).unwrap();

            // Test verification here to make sure it works
            let is_valid_test = keypair.verify(&message, &signature, None);
            println!("Test verification with keypair.verify: {}", is_valid_test);

            // Test verification with the sign module function
            use dilithium::sign::ml_dsa_44 as sign_ml_dsa_44;
            let is_valid_sign =
                sign_ml_dsa_44::verify(&keypair.public.to_bytes(), &message, &signature);
            println!(
                "Test verification with sign::ml_dsa_44::verify: {}",
                is_valid_sign
            );

            (
                keypair.public.to_bytes().to_vec(),
                signature.to_vec(),
                message,
            )
        }
        3 => {
            let keypair = ml_dsa_65::Keypair::generate(Some(&entropy));
            let message = b"Hello, SP1 ML-DSA verification!".to_vec();
            let signature = keypair.sign(&message, None, false).unwrap();
            (
                keypair.public.to_bytes().to_vec(),
                signature.to_vec(),
                message,
            )
        }
        5 => {
            let keypair = ml_dsa_87::Keypair::generate(Some(&entropy));
            let message = b"Hello, SP1 ML-DSA verification!".to_vec();
            let signature = keypair.sign(&message, None, false).unwrap();
            (
                keypair.public.to_bytes().to_vec(),
                signature.to_vec(),
                message,
            )
        }
        _ => panic!("Invalid variant: {}", args.variant),
    };

    let mut stdin = SP1Stdin::new();
    stdin.write(&args.variant);
    stdin.write(&public_key);
    stdin.write(&signature);
    stdin.write(&message);

    if args.execute {
        let (output, report) = client.execute(MLDSA_ELF, &stdin).run().unwrap();
        let is_valid: bool = output.as_slice()[0] != 0;
        println!("Signature valid: {}", is_valid);
        println!("Cycles: {}", report.total_instruction_count());
    } else {
        let mode = match args.proof_mode.as_str() {
            "groth16" => SP1ProofMode::Groth16,
            "plonk" => SP1ProofMode::Plonk,
            _ => panic!("Invalid proof mode: {}", args.proof_mode),
        };

        let (pk, vk) = client.setup(MLDSA_ELF);
        let proof = client
            .prove(&pk, &stdin)
            .mode(mode)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated {} proof!", args.proof_mode);
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
