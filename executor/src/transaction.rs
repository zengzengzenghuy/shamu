use crypto::{Digest, Hash};
use ed25519_dalek::Digest as _;
use ed25519_dalek::Sha512;
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// The object's version number.
pub type ObjectVersion = u64;

/// A dumb object in the system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    /// The unique object's id.
    pub id: Digest,
    /// The object's content. This field is used to set the object's size.
    pub content: Vec<u8>,
    /// The object's version number.
    pub version: ObjectVersion,
}

impl Hash for Object {
    fn digest(&self) -> Digest {
        let mut hasher = Sha512::new();
        hasher.update(&self.content);
        hasher.update(self.version.to_le_bytes());
        Digest(hasher.finalize().as_slice()[..32].try_into().unwrap())
    }
}

impl Object {
    /// Create a new object with the specified content.
    #[allow(unused)]
    pub fn new(content: Vec<u8>) -> Self {
        let object = Self {
            id: Digest::default(),
            content,
            version: ObjectVersion::default(),
        };
        Self {
            id: object.digest(),
            ..object
        }
    }

    /// Create a new object with the specified content and a random id.
    #[allow(unused)]
    pub fn random<R: RngCore>(rng: &mut R, content: Vec<u8>) -> Self {
        let mut id = [0u8; 32];
        rng.fill_bytes(&mut id);
        Self {
            id: Digest(id),
            content,
            version: ObjectVersion::default(),
        }
    }
}

/// A transaction updating or creating objects.
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    /// The unique id of the transaction.
    pub id: Digest,
    /// The list of objects that this transaction reads or modifies.
    pub inputs: Vec<Object>,
    /// Represents the smart contract to execute. In this fake transaction,
    /// it determines the number of ms of CPU time needed to execute it.
    pub execution_time: u64,
}

impl Hash for Transaction {
    fn digest(&self) -> Digest {
        let mut hasher = Sha512::new();
        hasher.update(&self.execution_time.to_le_bytes());
        for object in &self.inputs {
            hasher.update(&object.id);
        }
        Digest(hasher.finalize().as_slice()[..32].try_into().unwrap())
    }
}

impl Transaction {
    /// Creates a transaction calling a contract with the specified objects.
    #[allow(unused)]
    pub fn new(inputs: Vec<Object>, execution_time: u64) -> Self {
        let transaction = Self {
            id: Digest::default(),
            execution_time,
            inputs,
        };
        Self {
            id: transaction.digest(),
            ..transaction
        }
    }
}
