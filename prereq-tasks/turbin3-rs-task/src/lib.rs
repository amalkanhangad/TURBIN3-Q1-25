mod programs;

#[cfg(test)]
mod tests {

    use solana_sdk::{message::Message, signature::{read_keypair_file, Keypair, Signer}, signer, system_program, transaction::Transaction};
    use solana_sdk::bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use std::{io::{self, BufRead}, str::FromStr};
    use crate::programs::Turbin3_prereq::{ CompleteArgs, Turbin3PrereqProgram };

#[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());       
    }

#[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap(); 
    println!("{:?}", wallet);

    }

    #[test]
    fn wallet_to_base58() {
       println!("Input the private key a wallet file byte array:");
       let stdin = io::stdin();
       let wallet = stdin.lock().lines().next().unwrap().unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']').split(',')
            .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
       println!("Your private key is:");
       let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);
    } 

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64){
            Ok(s) =>{
                println!("Success! check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        }
    }

    #[test]
    fn transfer_sol() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        let to_pubkey = Pubkey::from_str("DaaBgiMjmMikJ6xtt9A3HmyrPd1g4FoYupkyniE9XvnY").unwrap();

        let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
        println!("transfer_sol() -> recent_blockhash: {}", recent_blockhash);
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash
        );

        let signature = client.send_and_confirm_transaction(&transaction)
                            .expect("Failed Transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",signature);
    }

    #[test]
    fn drain_wallet(){
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        let to_pubkey = Pubkey::from_str("DaaBgiMjmMikJ6xtt9A3HmyrPd1g4FoYupkyniE9XvnY").unwrap();

        //get balance
        let balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
        let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = client.get_fee_for_message(&message).expect("Failed get the gas fee");
        let transaction = Transaction::new_signed_with_payer(&[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

        let signature = client.send_and_confirm_transaction(&transaction).expect("Transaction failed");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    }

    #[test]
    fn enroll(){
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );

        let args = CompleteArgs{
            github: b"amalkanhangad".to_vec(),
        };

        let recent_blockhash = client.get_latest_blockhash().expect("Failed to fetch");

        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Transaction failed");
        println!("Enrollment complete! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

       
    }

}