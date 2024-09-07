pub trait CryptographyService {
    fn validate_public_key(&self, public_key: &str) -> bool;
    fn validate_signature(&self, plaintext: &str, signature_base64: &str, public_key: &str)
        -> bool;
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>>;
    fn produce_signature(
        &self,
        plaintext: &str,
        private_key: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
