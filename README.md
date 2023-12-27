# Bitcoin-and-Ethereum-Wallet-Cryptography
 # Wallet Cryptography

This project demonstrates how to generate a new wallet, including a private key, public key, and address, for either the Bitcoin or Ethereum network.

## Prerequisites

- Rust installed
- `cargo` installed

## Usage

To use this project, follow these steps:

1. Clone the repository:

```
git clone https://github.com/walletcryptography/walletcryptography.git
```

2. Change directory into the project:

```
cd walletcryptography
```

3. Run the following command to build the project:

```
cargo build
```

4. Run the following command to generate a new wallet for the Bitcoin network:

```
./target/debug/walletcryptography bitcoin <private-key>
```

Replace `<private-key>` with a 64-character hexadecimal string representing the private key.

5. Run the following command to generate a new wallet for the Ethereum network:

```
./target/debug/walletcryptography ethereum <private-key>
```

Replace `<private-key>` with a 64-character hexadecimal string representing the private key.

## Output

The output of the program will be a file named `bitcoin_address_<address>` or `ethereum_address_<address>`, where `<address>` is the address of the new wallet. This file will contain the following information:

- Network: The network for which the wallet was generated (Bitcoin or Ethereum).
- Address: The address of the new wallet.
- Private Key: The private key of the new wallet.
- Uncompressed Public Key: The uncompressed public key of the new wallet.

## Code Explanation

The code for this project is written in Rust and uses the following libraries:

- `walletcryptography`: This library provides functions for generating private keys, public keys, and addresses for the Bitcoin and Ethereum networks.
- `bitcoin`: This library provides functions for working with Bitcoin addresses and private keys.
- `ethereum`: This library provides functions for working with Ethereum addresses and private keys.

The main function of the program takes two arguments: the network (Bitcoin or Ethereum) and the private key. The private key must be a 64-character hexadecimal string.

The program first checks that the private key is valid. If the private key is not valid, the program prints an error message and exits.

Next, the program derives the public key from the private key.
