import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../Turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { updateAsDataDelegateV2 } from "@metaplex-foundation/mpl-token-metadata";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("85jq2RGcXqJ91eayNtvWXpTryCbRnS1RZWS3dBc2LhPG");

// Recipient address
const to = new PublicKey("5sus6wj7rbUSEhoEZ364ESszPNee11JNkcuH1mW93WGG");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        // Get the token account of the toWallet address, and if it does not exist, create it
        const ata2 = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);
        // Transfer the new token to the "toTokenAccount" we just created
        const txid = await transfer(connection, keypair, ata.address, ata2.address, keypair, 1e6);
        console.log(`Transaction signature: ${txid}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
