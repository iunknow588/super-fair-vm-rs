//! Performance benchmarks for FairVM.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ethers::types::U256;
use fair_vm::{Address, FairVM, Transaction, TransactionType};
use tokio::runtime::Runtime;

/// 基准测试：提交交易
pub fn bench_submit_transaction(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("submit_transaction", |b| {
        b.iter(|| {
            rt.block_on(async {
                let fairvm = FairVM::new();
                let tx = Transaction {
                    hash: Default::default(),
                    from: Address([0u8; 20]),
                    to: Some(Address([1u8; 20])),
                    value: U256::from(100),
                    nonce: 0,
                    gas_limit: 21000,
                    gas_price: Some(U256::from(1)),
                    data: vec![],
                    signature: vec![],
                    transaction_type: TransactionType::Legacy,
                    chain_id: 1,
                    max_fee_per_gas: Some(U256::from(2)),
                    max_priority_fee_per_gas: Some(U256::from(1)),
                };
                let _ = fairvm.submit_transaction(tx).await;
            })
        })
    });
}

/// 基准测试：创建区块
pub fn bench_create_block(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("create_block", |b| {
        b.iter(|| {
            rt.block_on(async {
                let fairvm = FairVM::new();
                let tx = Transaction {
                    hash: Default::default(),
                    from: Address([0u8; 20]),
                    to: Some(Address([1u8; 20])),
                    value: U256::from(100),
                    nonce: 0,
                    gas_limit: 21000,
                    gas_price: Some(U256::from(1)),
                    data: vec![],
                    signature: vec![],
                    transaction_type: TransactionType::Legacy,
                    chain_id: 1,
                    max_fee_per_gas: Some(U256::from(2)),
                    max_priority_fee_per_gas: Some(U256::from(1)),
                };
                let _ = fairvm.submit_transaction(tx).await;
            })
        })
    });
}

/// 基准测试：账户查询
pub fn bench_get_account(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("get_account", |b| {
        b.iter(|| {
            rt.block_on(async {
                let fairvm = FairVM::new();
                let address = Address([0u8; 20]);
                black_box(fairvm.get_account(&address).await);
            })
        })
    });
}

criterion_group!(
    benches,
    bench_submit_transaction,
    bench_create_block,
    bench_get_account
);
criterion_main!(benches);
