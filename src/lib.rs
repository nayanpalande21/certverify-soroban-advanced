#![no_std]

mod token;

use soroban_sdk::{contract, contractimpl, Env, BytesN};

#[contract]
pub struct CertificateVerifier;

#[contractimpl]
impl CertificateVerifier {

    // Register certificate hash
    pub fn register_hash(env: Env, hash: BytesN<32>) {

        // store certificate hash
        env.storage().instance().set(&hash, &true);

        // emit registration event
        env.events().publish(
            ("certificate", "registered"),
            hash.clone()
        );

        // call token contract (simulated inter-contract logic)
        token::CertificateToken::mint(env.clone(), hash.clone());
    }

    // Verify certificate hash
    pub fn verify_hash(env: Env, hash: BytesN<32>) -> bool {

        let result = env.storage().instance().get(&hash).unwrap_or(false);

        // emit verification event
        env.events().publish(
            ("certificate", "verified"),
            (hash.clone(), result)
        );

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, BytesN};

    #[test]
    fn test_register_hash() {
        let env = Env::default();
        let contract_id = env.register(CertificateVerifier, ());
        let client = CertificateVerifierClient::new(&env, &contract_id);

        let hash = BytesN::from_array(&env, &[1; 32]);

        client.register_hash(&hash);
        let result = client.verify_hash(&hash);

        assert_eq!(result, true);
    }

    #[test]
    fn test_verify_non_existing_hash() {
        let env = Env::default();
        let contract_id = env.register(CertificateVerifier, ());
        let client = CertificateVerifierClient::new(&env, &contract_id);

        let hash = BytesN::from_array(&env, &[2; 32]);

        let result = client.verify_hash(&hash);

        assert_eq!(result, false);
    }

    #[test]
    fn test_multiple_hashes() {
        let env = Env::default();
        let contract_id = env.register(CertificateVerifier, ());
        let client = CertificateVerifierClient::new(&env, &contract_id);

        let hash1 = BytesN::from_array(&env, &[3; 32]);
        let hash2 = BytesN::from_array(&env, &[4; 32]);

        client.register_hash(&hash1);

        assert_eq!(client.verify_hash(&hash1), true);
        assert_eq!(client.verify_hash(&hash2), false);
    }
}