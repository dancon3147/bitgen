use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        format!("{}->{}:{}", self.sender, self.recipient, self.amount)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl SignedTransaction {
    pub fn verify(&self) -> bool {
        use ed25519_dalek::{PublicKey, Signature, Verifier};
        use sha2::{Digest, Sha256};

        let public_key = match PublicKey::from_bytes(&self.public_key) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        let expected_address = {
            let hash = Sha256::digest(&self.public_key);
            bs58::encode(hash).into_string()
        };

        if expected_address != self.transaction.sender {
            return false;
        }

        let message = self.transaction.to_string();
        let signature = match Signature::from_bytes(&self.signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        public_key.verify(message.as_bytes(), &signature).is_ok()
    }
}