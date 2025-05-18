FairVMError::ConsensusError(ConsensusError::from(consensus::basic::ConsensusError::from(err)))

latest_block_hash: H256::from_slice(&state.last_commit_hash.0.into()),

hash: H256::from_slice(&transaction.hash.0.into()), 