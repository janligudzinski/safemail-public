use base64::Engine;
use bcrypt::{hash, verify, DEFAULT_COST};
use domain::{crypto::CryptographyService, user::PasswordService};
use openssl::{
    hash::MessageDigest,
    pkey::PKey,
    rsa::{Padding, Rsa},
    sign::{Signer, Verifier},
};
#[derive(Clone)]
pub struct BcryptPasswordService;

impl PasswordService for BcryptPasswordService {
    fn hash_password(&self, password: String) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }

    fn verify_password(&self, password: String, hashed_password: &str) -> bool {
        verify(password, hashed_password).unwrap()
    }
}

#[derive(Clone)]
pub struct OpensslCryptographyService;

impl CryptographyService for OpensslCryptographyService {
    fn validate_public_key(&self, public_key: &str) -> bool {
        let engine = base64::engine::general_purpose::STANDARD;
        let bytes = match engine.decode(public_key) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
        Rsa::public_key_from_der(&bytes).is_ok()
    }
    fn validate_signature(
        &self,
        plaintext: &str,
        signature_base64: &str,
        public_key: &str,
    ) -> bool {
        let engine = base64::engine::general_purpose::STANDARD;
        let plaintext_bytes = plaintext.as_bytes();
        let signature = engine
            .decode(signature_base64)
            .expect("Expected base64 encoded signature");
        let public_key = engine
            .decode(public_key)
            .expect("Expected base64 encoded public key");
        let key =
            PKey::public_key_from_der(&public_key).expect("Public key reading from DER failed");
        let mut verifier =
            Verifier::new(MessageDigest::sha256(), &key).expect("Verifier creation failed");
        verifier.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
        verifier
            .set_rsa_pss_saltlen(openssl::sign::RsaPssSaltlen::DIGEST_LENGTH)
            .unwrap();
        verifier
            .verify_oneshot(&signature, plaintext_bytes)
            .unwrap_or(false)
    }

    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        let rsa = Rsa::generate(2048)?;
        let private_key = rsa.private_key_to_pem()?;
        let public_key = rsa.public_key_to_pem()?;

        let engine = base64::engine::general_purpose::STANDARD;
        let private_key_base64 = engine.encode(&private_key);
        let public_key_base64 = engine.encode(&public_key);

        Ok((public_key_base64, private_key_base64))
    }

    fn produce_signature(
        &self,
        plaintext: &str,
        private_key: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let engine = base64::engine::general_purpose::STANDARD;
        let private_key_der = engine.decode(private_key)?;
        let key = PKey::private_key_from_pem(&private_key_der)?;

        let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
        signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        signer.set_rsa_pss_saltlen(openssl::sign::RsaPssSaltlen::DIGEST_LENGTH)?;
        signer.update(plaintext.as_bytes())?;
        let signature = signer.sign_to_vec()?;

        Ok(engine.encode(signature))
    }
}
