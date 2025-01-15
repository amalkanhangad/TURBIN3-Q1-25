import bs58 from "bs58";

function base58ToWallet(): void {
  console.log("Enter your base58 private key:");
  const base58 = "";

  try {
    const wallet = bs58.decode(base58);
    console.log("Wallet bytes:", Array.from(wallet));
  } catch (error) {
    console.error("Invalid base58 string:", error);
  }
}

base58ToWallet();