mod monitor;
mod db;

pub use db::DB;
use std::env;
use std::time;
use web3::{
    contract::{Contract, Options},
    futures::{future, StreamExt},
    types::{FilterBuilder, Address},
};
use std::str::FromStr;
use web3::types::{BlockNumber, U64, H256};
use arrayref::array_ref;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    dotenv::dotenv().ok();
    let web3 = web3::Web3::new(web3::transports::WebSocket::new(&env::var("MAINNET_WSS").unwrap()).await?);
    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Number(U64::from(16690066)))
        .address(vec![Address::from_str("0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc").unwrap()])
        .topics(
            // this is 'Transfer (index_topic_1 address from, index_topic_2 address to, uint256 value)' event
            // use https://emn178.github.io/online-tools/keccak_256.html, and type in 'Transfer(address,address,uint256)'
            // it will return result hash as used in next line
            Some(vec![array_ref!("0xe9e508bad6d4c3227e881ca19068f099da81b5164dd6d62b2eaf1e8bc6c34931".as_bytes()[0..32], 0, 32).into()]),
            None,
            None,
            None,
        )
    //    .topics(
    //         Some(vec![array_ref!("0xa945e51eec50ab98c161376f0db4cf2aeba3ec92755fe2fcd388bdbbb80ff196".as_bytes()[0..32], 0, 32).into()]),
    //         None,
    //         None,
    //         None,
    //     )
        .build();

    let sub = web3.eth_subscribe().subscribe_logs(filter).await?;

    sub.for_each(|log| {
        println!("{:?}", log);
        future::ready(())
    }).await;

    Ok(())
}





// use std::env;
// use std::str::FromStr;

// use web3::contract::{tokens, Contract, Options};
// use web3::types::{Address, U64, H160, U256};

// #[tokio::main]
// async fn main() -> web3::Result<()> {
//     dotenv::dotenv().ok();
//     let http = web3::transports::Http::new(&env::var("MAINNET_HTTP").unwrap())?;
//     let web3h = web3::Web3::new(http);
//     let mut accounts = web3h.eth().accounts().await?;
//     accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
//     println!("Accounts: {:?}", accounts);
//     let wei_conv: U256 = U256::exp10(18);
//     for account in accounts {
//         let balance = web3h.eth().balance(account, None).await?;
//         println!(
//             "Eth balance of {:?}: {}",
//             account,
//             balance.checked_div(wei_conv).unwrap()
//         );
//     }

//     let aave_addr = Address::from_str("0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc").unwrap();
//     let tornado_contract =
//         Contract::from_json(web3h.eth(), aave_addr, include_bytes!("tornado_abi.json")).unwrap();
//     // let token_name: String = token_contract
//     //     .query("name", (), None, Options::default(), None)
//     //     .await
//     //     .unwrap();
//     // let total_supply: U256 = token_contract
//     //     .query("totalSupply", (), None, Options::default(), None)
//     //     .await
//     //     .unwrap();
//     // println!("Token name: {}, total supply: {}", token_name, total_supply);

//     // token_contract.events()
//     let result: Vec<(U256,U256)> = tornado_contract.events("deposit", (), (), ())
//         .await
//         .unwrap();
//     for (count, v) in result.iter().enumerate() {
//         let result: &(U256,U256) = v;
//         println!("Event result: {:#?}", result);
//     }
//     //println!("Event result: {:#?}", result);
//     Ok(())
// }
