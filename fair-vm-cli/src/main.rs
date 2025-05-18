use clap::{Parser, Subcommand};
use ethers::providers::{Http, Provider};
use ethers::types::{Address, Bytes, U256};
// use fairvm_sdk::{client::Client, wallet::Wallet};
use fair_vm_sdk::wallet::FairWallet;
// 请根据实际类型导入 FairWallet 或 HardwareWallet，如果需要
// use fairvm_sdk::wallet::HardwareWallet;
use bytes::Bytes as BytesType;
use rand::rngs::OsRng;
use std::str::FromStr;
/// 默认链 ID
const CHAIN_ID: u64 = 1337;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 钱包相关操作
    Wallet {
        #[command(subcommand)]
        action: WalletCommands,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// 创建新钱包
    New {
        /// 是否使用助记词
        #[arg(long)]
        mnemonic: bool,
    },

    /// 从助记词导入钱包
    ImportMnemonic {
        /// 助记词
        phrase: String,
    },

    /// 从私钥导入钱包
    Import {
        /// 私钥
        private_key: String,
    },

    /// 导出私钥
    ExportKey {
        /// 助记词或私钥
        key: String,
    },

    /// 保存到密钥库
    SaveKeystore {
        /// 私钥或助记词
        key: String,
        /// 密码
        password: String,
        /// 保存路径
        path: String,
    },

    /// 从密钥库加载
    LoadKeystore {
        /// 密钥库文件路径
        path: String,
        /// 密码
        password: String,
    },

    /// 连接 Ledger 钱包
    ConnectLedger {
        /// 派生路径（可选）
        #[arg(long)]
        path: Option<String>,
    },

    /// 从 Ledger 获取地址
    GetLedgerAddress {
        /// 派生路径（可选）
        #[arg(long)]
        path: Option<String>,
    },

    /// 使用 Ledger 发送交易
    SendFromLedger {
        /// 接收地址
        to: String,
        /// 发送金额(wei)
        value: String,
        /// RPC URL
        rpc_url: String,
        /// 派生路径（可选）
        #[arg(long)]
        path: Option<String>,
    },

    /// 发送交易
    Send {
        /// 接收地址
        to: String,

        /// 发送金额(wei)
        value: String,

        /// 私钥或助记词
        key: String,

        /// RPC URL
        rpc_url: String,
    },

    /// 估算交易 gas
    EstimateGas {
        /// 接收地址
        to: String,

        /// 发送金额(wei)
        value: String,

        /// 数据(可选)
        #[arg(long)]
        data: Option<String>,

        /// RPC URL
        rpc_url: String,
    },

    /// 获取当前网络的费用建议
    GetFees {
        /// RPC URL
        rpc_url: String,
    },

    /// 获取账户 nonce
    GetNonce {
        /// 账户地址
        address: String,

        /// RPC URL
        rpc_url: String,
    },
}

fn generate_random_private_key() -> String {
    let mut rng = OsRng;
    let secret_key = secp256k1::SecretKey::new(&mut rng);
    hex::encode(secret_key.secret_bytes())
}

async fn handle_wallet_command(cmd: WalletCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        WalletCommands::New { mnemonic } => {
            if mnemonic {
                let wallet = FairWallet::generate_new(CHAIN_ID)?;
                if let Some(phrase) = wallet.get_mnemonic() {
                    println!("新钱包已创建");
                    println!("地址: {:?}", wallet.address().await?);
                    println!("助记词: {}", phrase);
                    println!("请安全保存助记词！");
                }
            } else {
                let private_key = generate_random_private_key();
                let wallet = FairWallet::from_private_key(&private_key, CHAIN_ID)?;
                println!("新钱包已创建");
                println!("地址: {:?}", wallet.address().await?);
                println!("私钥: {}", private_key);
                println!("请安全保存私钥！");
            }
        }

        WalletCommands::ImportMnemonic { phrase } => {
            let wallet = FairWallet::from_mnemonic(&phrase, CHAIN_ID)?;
            println!("钱包已导入");
            println!("地址: {:?}", wallet.address().await?);
            println!("私钥: {}", wallet.export_private_key());
        }

        WalletCommands::Import { private_key } => {
            let wallet = FairWallet::from_private_key(&private_key, CHAIN_ID)?;
            println!("钱包已导入");
            println!("地址: {:?}", wallet.address().await?);
        }

        WalletCommands::ExportKey { key } => {
            let wallet = if key.contains(" ") {
                FairWallet::from_mnemonic(&key, CHAIN_ID)?
            } else {
                FairWallet::from_private_key(&key, CHAIN_ID)?
            };
            println!("私钥: {}", wallet.export_private_key());
        }

        WalletCommands::SaveKeystore {
            key,
            password,
            path,
        } => {
            let wallet = if key.contains(" ") {
                FairWallet::from_mnemonic(&key, CHAIN_ID)?
            } else {
                FairWallet::from_private_key(&key, CHAIN_ID)?
            };
            wallet.save_to_keystore(&path, &password)?;
            println!("密钥库已保存到: {}", path);
        }

        WalletCommands::LoadKeystore { path, password } => {
            let wallet = FairWallet::load_from_keystore(&path, &password, CHAIN_ID)?;
            println!("钱包已从密钥库加载");
            println!("地址: {:?}", wallet.address().await?);
        }

        WalletCommands::ConnectLedger { path } => {
            let wallet = FairWallet::connect_ledger(path, CHAIN_ID).await?;
            println!("Ledger 钱包已连接");
            println!("地址: {:?}", wallet.address().await?);
        }

        WalletCommands::GetLedgerAddress { path } => {
            let wallet = FairWallet::connect_ledger(path, CHAIN_ID).await?;
            println!("Ledger 地址: {:?}", wallet.address().await?);
        }

        WalletCommands::SendFromLedger {
            to,
            value,
            rpc_url,
            path,
        } => {
            let provider = Provider::<Http>::try_from(&rpc_url)?;
            let wallet = FairWallet::connect_ledger(path, CHAIN_ID).await?;

            let to = Address::from_str(&to)?;
            let value = U256::from_str(&value)?;

            let tx = ethers::types::TransactionRequest {
                to: Some(ethers::types::NameOrAddress::Address(to)),
                value: Some(value),
                ..Default::default()
            };

            let tx_hash = wallet.send_transaction(&provider, tx).await?;
            println!("交易已发送: {:?}", tx_hash);
        }

        WalletCommands::Send {
            to,
            value,
            key,
            rpc_url,
        } => {
            let provider = Provider::<Http>::try_from(&rpc_url)?;
            let wallet = if key.contains(" ") {
                FairWallet::from_mnemonic(&key, CHAIN_ID)?
            } else {
                FairWallet::from_private_key(&key, CHAIN_ID)?
            };

            let to = Address::from_str(&to)?;
            let value = U256::from_str(&value)?;

            let tx = ethers::types::TransactionRequest {
                to: Some(ethers::types::NameOrAddress::Address(to)),
                value: Some(value),
                ..Default::default()
            };

            let tx_hash = wallet.send_transaction(&provider, tx).await?;
            println!("交易已发送: {:?}", tx_hash);
        }

        WalletCommands::EstimateGas {
            to,
            value,
            data,
            rpc_url,
        } => {
            let provider = Provider::<Http>::try_from(&rpc_url)?;
            let to = Address::from_str(&to)?;
            let value = U256::from_str(&value)?;
            let data = match data {
                Some(d) => {
                    let decoded = hex::decode(d).unwrap();
                    Bytes::from(BytesType::from(decoded))
                }
                None => Bytes::default(),
            };

            let wallet = FairWallet::from_private_key(
                "0000000000000000000000000000000000000000000000000000000000000001",
                CHAIN_ID,
            )?;
            let gas = wallet
                .estimate_gas(&provider, Some(to), value, data)
                .await?;
            println!("估算的 gas: {}", gas);
        }

        WalletCommands::GetFees { rpc_url } => {
            let provider = Provider::<Http>::try_from(&rpc_url)?;
            let wallet = FairWallet::from_private_key(
                "0000000000000000000000000000000000000000000000000000000000000001",
                CHAIN_ID,
            )?;
            let fees = wallet.get_fees(&provider).await?;
            println!("{}", fees);
        }

        WalletCommands::GetNonce { address, rpc_url } => {
            let provider = Provider::<Http>::try_from(&rpc_url)?;
            let address = Address::from_str(&address)?;
            let wallet = FairWallet::from_private_key(
                "0000000000000000000000000000000000000000000000000000000000000001",
                CHAIN_ID,
            )?;
            let nonce = wallet.get_nonce(&provider, address).await?;
            println!("账户 nonce: {}", nonce);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Wallet { action } => handle_wallet_command(action).await?,
    }

    Ok(())
}
