#[cfg(test)]
mod tests {
    use std::task::Context;

    use casper_contract;
    use casper_engine_test_support::{self, AccountHash, Code, Hash, SessionBuilder, TestContext};
    use casper_types::{self, PublicKey, RuntimeArgs, SecretKey};

    struct Account {
        secret_key: SecretKey,
        public_key: PublicKey,
        addre: AccountHash,
    }
    impl Account {
        fn new_session_run(&self, code: Code, args: RuntimeArgs, context: TestContext) {
            context.run(
                SessionBuilder::new(code, args)
                    .with_address(self.addre)
                    .with_authorization_keys(&[self.addre]).build()
            )
        }
    }
    struct Contract {
        account: Account,
        session_code: Code,
        hash: Option<Hash>,
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
