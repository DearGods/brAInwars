use solana_client::rpc_client::RpcClient;
// use solana_program::keccak::{hashv, Hasher};
// use solana_program::system_instruction;
// use solana_sdk::commitment_config::CommitmentConfig;
// use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{read_keypair_file, Signer};
use solana_sdk::transaction::Transaction;
// use solana_zk_token_sdk::instruction::range_proof::RangeProofU64Data;
// use solana_zk_token_sdk::zk_token_proof_instruction::{
//     verify_ciphertext_ciphertext_equality, verify_range_proof_u64,
//     GroupedCiphertext2HandlesValidityProofData,
// };
use solana_zk_token_sdk::zk_token_proof_instruction::verify_ciphertext_commitment_equality;
// use solana_zk_token_sdk::zk_token_proof_program::ID as ZK_TOKEN_PROOF_PROGRAM_ID;
// use solana_zk_token_sdk::zk_token_proof_state::ProofContextState;
// use solana_zk_token_sdk::zk_token_proof_state::ProofContextStateMeta;

use solana_zk_token_sdk::encryption::elgamal::ElGamalKeypair;
// use solana_zk_token_sdk::encryption::grouped_elgamal::GroupedElGamal;
use solana_zk_token_sdk::encryption::pedersen::Pedersen;
use solana_zk_token_sdk::instruction::{CiphertextCommitmentEqualityProofData, PubkeyValidityData};
use solana_zk_token_sdk::{
    instruction::ZkProofData,
    // zk_token_proof_instruction::ProofInstruction, zk_token_proof_program,
};

pub fn main() {
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let connection = RpcClient::new(url);
    let wallet = read_keypair_file("/Users/ohaddahan/.config/solana/id.json").unwrap();
    println!("wallet: {:?}\n", wallet.pubkey().to_string());
    let keypair = ElGamalKeypair::new_rand();
    println!("keypair.pubkey: {:?}\n", keypair.pubkey().to_string());
    let amount: u64 = 55;
    let ciphertext = keypair.pubkey().encrypt(amount);
    println!("ciphertext.string: {:?}\n", ciphertext.to_string());
    let (commitment, opening) = Pedersen::new(amount);

    let _proof_data = CiphertextCommitmentEqualityProofData::new(
        &keypair,
        &ciphertext,
        &commitment,
        &opening,
        amount,
    )
    .unwrap();

    let pubkey_validity_data = PubkeyValidityData::new(&keypair).unwrap();
    pubkey_validity_data.verify_proof().unwrap();

    let latest_blockhash = connection.get_latest_blockhash().unwrap();

    let instruction = verify_ciphertext_commitment_equality(None, &pubkey_validity_data);
    let txn = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&wallet.pubkey()),
        &[&wallet],
        latest_blockhash,
    );

    match connection.send_and_confirm_transaction(&txn) {
        Ok(sig) => loop {
            if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => println!("Error transferring Sol:, {}", e),
    }

    // let amount = std::u64::MAX;
    // let (commitment, opening) = Pedersen::new(amount);
    // let proof_data = RangeProofU64Data::new(&commitment, amount, &opening).unwrap();
}
