use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{clock::Clock, instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_voting() {
    let program_id = voting::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/voting.so"));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let id = 1u64;
    let id_bytes = id.to_le_bytes();
    let (election_pda, election_bump) = Pubkey::find_program_address(
        &[voting::constants::ELECTION_SEED, id_bytes.as_ref()],
        &program_id,
    );
    let clock = svm.get_sysvar::<Clock>();
    let start_timestamp = clock.unix_timestamp;
    let end_timestamp = clock.unix_timestamp + 60 * 60 * 1000;
    let instruction = Instruction::new_with_bytes(
        program_id,
        &voting::instruction::Initialize {
            id,
            start_timestamp,
            end_timestamp,
            options: vec!["A".into(), "B".into()],
        }
        .data(),
        voting::accounts::Initialize {
            payer: payer.pubkey(),
            election: election_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    svm.send_transaction(tx).unwrap();
    let election_account = svm.get_account(&election_pda).unwrap();
    let mut election_data = election_account.data.as_slice();
    let election_state = voting::state::Election::try_deserialize(&mut election_data).unwrap();
    assert_eq!(election_state.bump, election_bump);
    assert_eq!(election_state.tallies, vec![0, 0]);

    let voter1 = Keypair::new();
    svm.airdrop(&voter1.pubkey(), 1_000_000_000).unwrap();
    let (voter1_pda, _voter1_bump) = Pubkey::find_program_address(
        &[
            voting::constants::VOTER_SEED,
            id_bytes.as_ref(),
            voter1.pubkey().as_ref(),
        ],
        &program_id,
    );
    let instruction = Instruction::new_with_bytes(
        program_id,
        &voting::instruction::Vote { _id: id, option: 0 }.data(),
        voting::accounts::Vote {
            voter: voter1.pubkey(),
            voter_pda: voter1_pda,
            election: election_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&voter1.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&voter1]).unwrap();
    svm.send_transaction(tx).unwrap();
    let election_account = svm.get_account(&election_pda).unwrap();
    let mut election_data = election_account.data.as_slice();
    let election_state = voting::state::Election::try_deserialize(&mut election_data).unwrap();
    assert_eq!(election_state.tallies, vec![1, 0]);

    // reject duplicate vote
    let instruction = Instruction::new_with_bytes(
        program_id,
        &voting::instruction::Vote { _id: id, option: 0 }.data(),
        voting::accounts::Vote {
            voter: voter1.pubkey(),
            voter_pda: voter1_pda,
            election: election_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&voter1.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&voter1]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_err());

    let voter2 = Keypair::new();
    svm.airdrop(&voter2.pubkey(), 1_000_000_000).unwrap();
    let (voter2_pda, _voter2_bump) = Pubkey::find_program_address(
        &[
            voting::constants::VOTER_SEED,
            id_bytes.as_ref(),
            voter2.pubkey().as_ref(),
        ],
        &program_id,
    );
    let instruction = Instruction::new_with_bytes(
        program_id,
        &voting::instruction::Vote { _id: id, option: 1 }.data(),
        voting::accounts::Vote {
            voter: voter2.pubkey(),
            voter_pda: voter2_pda,
            election: election_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&voter2.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&voter2]).unwrap();
    svm.send_transaction(tx).unwrap();
    let election_account = svm.get_account(&election_pda).unwrap();
    let mut election_data = election_account.data.as_slice();
    let election_state = voting::state::Election::try_deserialize(&mut election_data).unwrap();
    assert_eq!(election_state.tallies, vec![1, 1]);

    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = end_timestamp + 1;
    svm.set_sysvar(&clock);

    // reject vote after voting ended
    let voter3 = Keypair::new();
    svm.airdrop(&voter3.pubkey(), 1_000_000_000).unwrap();
    let (voter3_pda, _voter3_bump) = Pubkey::find_program_address(
        &[
            voting::constants::VOTER_SEED,
            id_bytes.as_ref(),
            voter3.pubkey().as_ref(),
        ],
        &program_id,
    );
    let instruction = Instruction::new_with_bytes(
        program_id,
        &voting::instruction::Vote { _id: id, option: 1 }.data(),
        voting::accounts::Vote {
            voter: voter3.pubkey(),
            voter_pda: voter3_pda,
            election: election_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&voter3.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&voter3]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_err())
}
