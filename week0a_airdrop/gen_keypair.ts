import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';
import * as prompt from 'prompt-sync';
import * as fs from 'fs';

// Generate a new keypair
let kp = Keypair.generate();

// Output the public key (wallet address) and secret key (private key)
console.log(`You've generated a new Solana wallet: 
${kp.publicKey.toBase58()}`);

console.log(`To save your wallet, copy and paste the output of the following into a JSON file: 
[${kp.secretKey.toString()}]`);

// Function to convert base58 string to wallet (byte array)
function base58_to_wallet() {
  const userInput = prompt()('Enter your base58 private key: ');
  const decodedWallet = bs58.decode(userInput);
  console.log('Decoded wallet (byte array):', decodedWallet);

  // Convert to JSON format and save it
  const jsonKeyFormat = JSON.stringify([ ...decodedWallet ]);
  fs.writeFileSync('wallet_key.json', jsonKeyFormat, 'utf8');
  console.log('Your private key has been saved in JSON format as wallet_key.json');
}

// Function to convert wallet (byte array) to base58
function wallet_to_base58() {
  const wallet: number[] = [227,234,145,148,23,198,159,154,155,214,139,175,5,54,45,175,236,105,57,173,111,91,88,50,244,32,240,26,134,60,201,135,196,122,128,22,246,146,132,38,17,149,1,157,1,252,239,75,249,224,142,249,58,140,161,246,154,134,24,89,87,219,158,61];
  const base58Encoded = bs58.encode(Buffer.from(wallet));
  console.log('Base58 encoded private key:', base58Encoded);
}

// You can call these functions as needed:
base58_to_wallet(); // Call this to convert from base58 to wallet
wallet_to_base58(); // Call this to convert from wallet (byte array) to base58
