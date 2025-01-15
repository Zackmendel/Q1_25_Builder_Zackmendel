import bs58 from 'bs58';

// Directly assign your private key as a base58-encoded string
// const userInput = '25Hj3sQKQDN1sX2TRTkfrFedzsGPcMqhdmxE52CFsNwXBFDDpLdewU9ApCZSQfujhLbCDQqPzJgRSYR2pGGTFMAp';  // Replace with your actual base58 private key

// Function to convert base58 string to wallet (byte array)
function base58_to_wallet() {
  const decodedWallet = bs58.decode('25Hj3sQKQDN1sX2TRTkfrFedzsGPcMqhdmxE52CFsNwXBFDDpLdewU9ApCZSQfujhLbCDQqPzJgRSYR2pGGTFMAp');
  console.log('Decoded wallet (byte array):', decodedWallet);
}

// Call the function to convert from base58 to wallet
base58_to_wallet();




// 25Hj3sQKQDN1sX2TRTkfrFedzsGPcMqhdmxE52CFsNwXBFDDpLdewU9ApCZSQfujhLbCDQqPzJgRSYR2pGGTFMAp
