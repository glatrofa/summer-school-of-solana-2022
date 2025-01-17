# Build Solana program
```
cd program
```
```
cargo build-bpf
```
# Test Solana program with TypeScript Client
```
cd client
```
Install npm packages
```
npm install
```
Start solana test validator
```
solana-test-validator -r
```
Deploy Solana program (must be already built and with some SOL to program/keys)
```
solana program deploy ../program/target/deploy/turnstile.so --program-id keys/program.json
```
Airdrop SOLs to initializer and user
```
solana airdrop 10 keys/initializer.json
solana airdrop 10 keys/user.json
```
Run `init` instruction
```
npm run init
```
If an error occurs, try using an older version of node
```
nvm use v16.14.2
```
Run `coin` instruction
```
npm run coin
```
Run `push` instruction
```
npm run push
```

To view the state of the program view the information of the state account:
```
solana account <state_pub.json public key>
```
