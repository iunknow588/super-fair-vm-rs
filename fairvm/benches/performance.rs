//! Performance benchmarks for FairVM.

use criterion::{criterion_group, criterion_main, Criterion};
use ethers::types::{H256, U256};
use fairvm::transaction::TransactionType;
use fairvm::{Address, FairVM};
use tokio::runtime::Runtime;

/// 基准测试：提交交易
pub fn bench_submit_transaction(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("submit_transaction", |b| {
        b.iter(|| {
            rt.block_on(async {
                // 创建FairVM实例
                let fairvm = FairVM::new();
                let vm = fairvm.vm();
                let mut vm_instance = vm.write().await;

                // 创建测试交易
                let from = Address::new([1; 20]);
                let to = Address::new([2; 20]);

                // 直接构造 fairvm::Transaction
                let fairvm_tx = fairvm::Transaction::new(
                    H256::zero(),
                    from,
                    Some(to),
                    U256::from(100u64),
                    0,
                    21000u64,
                    Some(U256::from(1_000_000_000u64)),
                    vec![],
                    vec![],
                    TransactionType::Legacy,
                    1u64,
                    None,
                    None,
                );
                let _ = vm_instance.submit_transaction(fairvm_tx).await;
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
                // 创建FairVM实例
                let fairvm = FairVM::new();
                let vm = fairvm.vm();
                let mut vm_instance = vm.write().await;

                // 创建区块
                let parent_hash = H256::zero();
                let timestamp = chrono::Utc::now().timestamp() as u64;

                // 创建区块并忽略结果（仅测量性能）
                let _ = vm_instance.create_block(parent_hash, timestamp).await;
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
                // 创建FairVM实例
                let fairvm = FairVM::new();
                let vm = fairvm.vm();
                let vm_instance = vm.read().await;

                // 查询账户
                let address = Address::new([3; 20]);
                let _ = vm_instance.get_account(&address).await;
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
