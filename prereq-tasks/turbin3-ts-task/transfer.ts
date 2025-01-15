import {
    Transaction,
    SystemProgram,
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction,
    PublicKey,
  } from "@solana/web3.js";
  import wallet from "./dev-wallet.json";
  
  const from = Keypair.fromSecretKey(new Uint8Array(wallet));
  const to = new PublicKey("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ");
  const connection = new Connection("https://api.devnet.solana.com");
  
  (async () => {
    try {
      const balance = await connection.getBalance(from.publicKey);
  
      const transaction = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance,
        })
      );
  
      const { blockhash } = await connection.getLatestBlockhash("confirmed");
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = from.publicKey;
  
      const message = transaction.compileMessage();
      const fee =
        (await connection.getFeeForMessage(message, "confirmed"))?.value || 0;
  
      if (balance <= fee) {
        throw new Error("Insufficient balance to cover the transaction fee.");
      }
  
      transaction.instructions.pop();
      transaction.add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance - fee,
        })
      );
  
      const signature = await sendAndConfirmTransaction(connection, transaction, [
        from,
      ]);
      console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
      if (e instanceof Error) {
        console.error(`Oops, something went wrong: ${e.message}`);
      } else {
        console.error(`Oops, something went wrong: ${e}`);
      }
    }
  })();