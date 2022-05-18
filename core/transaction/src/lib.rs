#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub trait Serializable {
    fn serialize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Serializable for transaction::Transaction {
    fn serialize(&self) -> String {
        String::from(serde_json::to_string(&self).unwrap())
    }
}

extern crate hex;
pub mod transaction {
    use serde::{Deserialize, Serialize};
    use sha3::{Digest, Sha3_384}; //To hash transaction struct

    #[derive(Serialize, Deserialize)]
    pub struct NameValue {
        pub name: String,
        pub value: u64,
    }
    #[derive(Serialize, Deserialize)]
    pub struct Transaction {
        pub hash: String,
        pub data: u128,
        pub from: String,
        pub to: String,
        pub amount: u128,
        pub tx_type: u64,
        pub typevalue: NameValue,
        pub sign: String,
        pub previous: String,
        pub blockid: u128,
        pub epoch: u64,
    }
    impl Transaction {
        pub fn new(
            hash: String,
            data: u128,
            from: String,
            to: String,
            amount: u128,
            tx_type: u64,
            typevalue: NameValue,
            sign: String,
            previous: String,
            blockid: u128,
            epoch: u64
        ) -> Transaction {
            Transaction {
                hash,
                data,
                from,
                to,
                amount,
                tx_type,
                typevalue,
                sign,
                previous,
                blockid,
                epoch
            }
        }
        pub fn hash(&self) -> String {
            let mut hasher = Sha3_384::new();

            // write input message
            hasher.update(format!(
                "{}{}{}{}{}{}{}{}{}{}",
                self.amount,
                self.data,
                self.from,
                self.hash,
                self.previous,
                self.sign,
                self.to,
                self.tx_type,
                self.typevalue.name,
                self.typevalue.value,
            ));
            // read hash digest
            format!("hash {:?}", hex::encode(hasher.finalize()))
        }

        // fn sign(&self, )
    }
    pub fn deserialize_transaction(transaction: String) -> Transaction {
        let transaction: Transaction = serde_json::from_str(&transaction).unwrap();
        transaction
    }

    // pub fn serializeE(var: Serializable) String {
    //     match var {
    //         Serializable::Transaction => Transaction.serialize()
    //     }
    // }
}
