from: CoreAddress::from(fair_vm_core::Address(H160::from_slice(&tx.from.0))),
            .map(|addr| CoreAddress::from(fair_vm_core::Address(H160::from_slice(&addr.0)))),
        hash: CoreHash::from(fair_vm_core::Hash(H256::from_slice(&tx.hash.0))), 