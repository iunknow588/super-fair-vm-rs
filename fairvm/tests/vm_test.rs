//! Integration tests for FairVM.

// #[tokio::test]
// async fn test_vm_initialization() {
//     let vm = FairVM::new();
//     assert!(!vm.is_bootstrapped().await);
// }

// #[tokio::test]
// async fn test_block_proposal() {
//     let vm = FairVM::new();
//     let data = vec![1, 2, 3, 4];
//     vm.propose_block(data.clone()).await.unwrap();
//
//     let mempool = vm.mempool.read().await;
//     assert_eq!(mempool.len(), 1);
//     assert_eq!(mempool[0], data);
// }

// #[tokio::test]
// async fn test_state_management() {
//     let vm = FairVM::new();
//     let mut state = vm.state.write().await;
//
//     let address = Address::random();
//     let mut account = state.get_account_mut(&address);
//     account.balance = U256::from(1000);
//
//     let retrieved = state.get_account(&address).unwrap();
//     assert_eq!(retrieved.balance, U256::from(1000));
// }

// #[tokio::test]
// async fn test_transfer() {
//     let vm = FairVM::new();
//     let mut state = vm.state.write().await;
//
//     let from = Address::random();
//     let to = Address::random();
//
//     // Setup initial balance
//     state.get_account_mut(&from).balance = U256::from(1000);
//
//     // Perform transfer
//     assert!(state.transfer(&from, &to, U256::from(500)));
//
//     // Verify balances
//     assert_eq!(state.get_account(&from).unwrap().balance, U256::from(500));
//     assert_eq!(state.get_account(&to).unwrap().balance, U256::from(500));
// }

// #[tokio::test]
// async fn test_block_creation() {
//     let parent_id = ids::Id::empty();
//     let timestamp = 12345;
//     let height = 1;
//     let transactions = vec![];
//
//     let block = fairvm::block::Block::new(parent_id, timestamp, height, transactions);
//
//     assert_eq!(block.height(), height);
//     assert_eq!(block.timestamp(), timestamp);
//     assert_eq!(block.parent_id(), parent_id);
// }
