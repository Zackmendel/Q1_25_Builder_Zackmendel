import bs58 from 'bs58'

const address = '25Hj3sQKQDN1sX2TRTkfrFedzsGPcMqhdmxE52CFsNwXBFDDpLdewU9ApCZSQfujhLbCDQqPzJgRSYR2pGGTFMAp'
const bytes = bs58.decode(address)
// See uint8array-tools package for helpful hex encoding/decoding/compare tools
console.log(Buffer.from(bytes).toString('hex'))
// => 003c176e659bea0f29a3e9bf7880c112b1b31b4dc826268187

