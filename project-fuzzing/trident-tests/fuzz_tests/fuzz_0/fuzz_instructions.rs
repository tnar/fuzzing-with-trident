pub mod basic_5_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Create(Create),
        Walk(Walk),
        Run(Run),
        Jump(Jump),
        Reset(Reset),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Create {
        pub accounts: CreateAccounts,
        pub data: CreateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAccounts {
        pub action_state: AccountId,
        pub user: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateData {}
    #[derive(Arbitrary, Debug)]
    pub struct Walk {
        pub accounts: WalkAccounts,
        pub data: WalkData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WalkAccounts {
        pub action_state: AccountId,
        pub user: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WalkData {}
    #[derive(Arbitrary, Debug)]
    pub struct Run {
        pub accounts: RunAccounts,
        pub data: RunData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RunAccounts {
        pub action_state: AccountId,
        pub user: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RunData {}
    #[derive(Arbitrary, Debug)]
    pub struct Jump {
        pub accounts: JumpAccounts,
        pub data: JumpData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct JumpAccounts {
        pub action_state: AccountId,
        pub user: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct JumpData {}
    #[derive(Arbitrary, Debug)]
    pub struct Reset {
        pub accounts: ResetAccounts,
        pub data: ResetData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ResetAccounts {
        pub action_state: AccountId,
        pub user: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ResetData {}
    impl<'info> IxOps<'info> for Create {
        type IxData = basic_5::instruction::Create;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = basic_5::instruction::Create {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.user.get_or_create_account(
                self.accounts.user,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let action_state = fuzz_accounts
            .action_state
            .get_or_create_account(
                self.accounts.action_state,
                &[b"action-state", signer.pubkey().as_ref()],
                &basic_5::ID,
            )
            .unwrap();

            let signers = vec![signer.clone()];

            let acc_meta = basic_5::accounts::Create {
                action_state: action_state.pubkey,
                user: signer.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Walk {
        type IxData = basic_5::instruction::Walk;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WalkSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = basic_5::instruction::Walk {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.user.get_or_create_account(
                self.accounts.user,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let action_state = fuzz_accounts
            .action_state
            .get_or_create_account(
                self.accounts.action_state,
                &[b"action-state", signer.pubkey().as_ref()],
                &basic_5::ID,
            )
            .unwrap();

            let signers = vec![signer.clone()];

            let acc_meta = basic_5::accounts::Walk {
                action_state: action_state.pubkey,
                user: signer.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Run {
        type IxData = basic_5::instruction::Run;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = RunSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = basic_5::instruction::Run {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.user.get_or_create_account(
                self.accounts.user,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let action_state = fuzz_accounts
            .action_state
            .get_or_create_account(
                self.accounts.action_state,
                &[b"action-state", signer.pubkey().as_ref()],
                &basic_5::ID,
            )
            .unwrap();

            let signers = vec![signer.clone()];

            let acc_meta = basic_5::accounts::Run {
                action_state: action_state.pubkey,
                user: signer.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Jump {
        type IxData = basic_5::instruction::Jump;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = JumpSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = basic_5::instruction::Jump {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.user.get_or_create_account(
                self.accounts.user,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let action_state = fuzz_accounts
            .action_state
            .get_or_create_account(
                self.accounts.action_state,
                &[b"action-state", signer.pubkey().as_ref()],
                &basic_5::ID,
            )
            .unwrap();

            let signers = vec![signer.clone()];

            let acc_meta = basic_5::accounts::Jump {
                action_state: action_state.pubkey,
                user: signer.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Reset {
        type IxData = basic_5::instruction::Reset;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ResetSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = basic_5::instruction::Reset {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.user.get_or_create_account(
                self.accounts.user,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let action_state = fuzz_accounts
            .action_state
            .get_or_create_account(
                self.accounts.action_state,
                &[b"action-state", signer.pubkey().as_ref()],
                &basic_5::ID,
            )
            .unwrap();

            let signers = vec![signer.clone()];

            let acc_meta = basic_5::accounts::Reset {
                action_state: action_state.pubkey,
                user: signer.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        action_state: AccountsStorage<PdaStore>,
        // system_program: AccountsStorage<todo!()>,
        user: AccountsStorage<Keypair>,
    }
}
