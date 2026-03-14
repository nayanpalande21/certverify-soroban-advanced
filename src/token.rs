use soroban_sdk::{contract, contractimpl, Env, BytesN};

#[contract]
pub struct CertificateToken;

#[contractimpl]
impl CertificateToken {

    pub fn mint(env: Env, cert_hash: BytesN<32>) {

        // store minted token
        env.storage().persistent().set(&cert_hash, &true);

        // emit mint event
        env.events().publish(
            ("certificate_token", "minted"),
            cert_hash
        );
    }
}