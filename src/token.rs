use soroban_sdk::{contract, contractimpl, Env, BytesN};

#[contract]
pub struct CertificateToken;

#[contractimpl]
impl CertificateToken {

    pub fn mint(env: Env, cert_hash: BytesN<32>) {

        // store token
        env.storage().persistent().set(&cert_hash, &true);

        // improved event data
        env.events().publish(
            ("certificate_token", "minted"),
            (cert_hash.clone(), true)
        );
    }
}