
# NEP-141 Standard token contract for NEAR protocol

This smart contract creates a token on the NEAR blockchain. This token follows
the [NEP-141 and NEP-148 standards](https://nomicon.io/Standards/Tokens/FungibleToken/) and does
not have a token minting function.




## Authors

- [@hack-a-chain-software](https://github.com/hack-a-chain-software)


## Appendix

In order to deploy and create a token, there are a few prerequisites necessary:
- Install near CLI (Command Line Interface) - (Please ensure you [have NodeJS](https://nodejs.org/en/download/package-manager/) > 12.)
- Install RUST
- Add Wasm toolchain

### NEAR CLI
To Install the near CLI, open your terminal:
 - On Mac open Terminal using Spotlight with these steps: Press Command + Space Bar on your Mac Keyboard. Type in “Terminal” When you see Terminal in the Spotlight search list, click it to open the app
 - On Windows, click Start and search for "Command Prompt." Alternatively, you can also access the command prompt by pressing Ctrl + r on your keyboard, type "cmd" and then click OK.

and run the following command: 
```bash
  npm install -g near-cli
```
Now, you can run:

```bash
near
```

After that, you can log in on the NEAR account that you would like to be 
the **address where the contract will be deployed** - Please note that this 
is **not the owner of the contract**. To log in, type: 
```bash
near login
```

This will bring you to NEAR Wallet again where you can confirm the creation of a full-access key.

### RUST

Rust is the programming language used to program the smart contract. In order to 
use this contract, you must have rust installed on your computer.

To install Rust we will be using ```rustup``` an installer for rust.
Please see the directions from the [Rustup](https://rustup.rs/#) site. For OS X or Unix, you may use (type on command line):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Pro tip: to keep your command line clean, type ```clear``` and hit enter.

### Wasm toolchain

Smart contracts compile to WebAssembly (Wasm) so we'll add the toolchain for Rust:

```bash
rustup target add wasm32-unknown-unknown
```

More info about it can be found [here](https://rustwasm.github.io/docs/book/).

If you followed correctly the steps above, you are now ready to go. 
You can read more about the NEAR CLI and the deployment of rust codes [here](https://www.near-sdk.io/zero-to-hero/basics/set-up-skeleton)




## Deployment

Assuming that you already have the ```NEAR CLI```, ```Rust``` and the ```Wasm Toolchain``` installed, and is logged in 
into the account that you want to deploy the project, we can now 
deploy it.

Now, we are going to deploy this project to the nar blockchain mainnet. 

Frist, make sure you are on the project folder. You can change yout folder by typing:

```bash
cd your-project-folder-path
```

Now, check if your project folders have a folder called ``` out ``` 
and a file called ``` main.wasm ``` if not, [check to near-sdk git](https://github.com/near/near-sdk-rs) 
on how to compile the code.


To make it easier to deploy the project, lets create an enviroment variable
with the **address that we want for our contract** (you must be logged into this wallet)

```bash
  export CONTRACT_ID="YOUR_ACCOUNT_NAME.near"
```

Now, finally, we are going to run the following command to deploy the code:

```bash
near deploy --wasmFile out/main.wasm --accountId $CONTRACT_ID
```

At this point, the contract should have been deployed to your account and you're ready to move onto configuring the 
token specifications and setting the contract owner.

### CONFIGURING THE CONTRACT 

Now, are contract is deployed. The next step is to configure it, according to your tokenomics.

If we check the code, will see that we have the following parameters used to define a token:

```bash
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
```

The ```owner_id ``` is the account that will own the contract. This account will be able perform 
actions that are restricted 

The ```total_supply ``` is the Token total supply. Note that this amount won't be changed - this contract
, due to business logic, does not support minting after the initial supply is minted. 

At last, the ``` FungibleTokenMetadata ``` ([reference](https://nomicon.io/Standards/Tokens/FungibleToken/Metadata)) is all the token metadata, wich means its all the extra token information
, that describes it. 

This metadata has the following format:
```bash
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,

```

An implementing contract MUST include the following fields on-chain:
- ```spec```: a string. Should be ```ft-1.0.0``` to indicate that a Fungible Token contract adheres to the current versions of this Metadata and the Fungible Token Core specs. This will allow consumers of the Fungible Token to know if they support the features of a given contract
- ```name```: the human-readable name of the token, E.g.: Bitcoin
- ```symbol```: the abbreviation, E.g.: BTC
- ```decimals```: used in frontends to show the proper significant digits of a token. This concept is explained well in this [OpenZeppelin](https://docs.openzeppelin.com/contracts/3.x/erc20#a-note-on-decimals) post - NEAR NEP-141 is 24 decimals.

An implementing contract MAY include the following fields on-chain
- ```icon```: a small image associated with this token. Must be a data URL, to help consumers display it quickly while protecting <br> user data [more information](https://nomicon.io/Standards/Tokens/FungibleToken/Metadata).
- ```reference```: a link to a valid JSON file containing various keys offering supplementary details on the token. <br>Example: /ipfs/QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm, https://example.com/token.json, etc. If the information given in this document conflicts with the on-chain attributes, the values in reference shall be considered the source of truth.
- ```reference_hash```:the base64-encoded sha256 hash of the JSON file contained in the reference field. This is to guard against off-chain tampering.

Although, we **strongly recommend** that you that you implement this following fields.

Now that we have everything covered, 

```bash
    near call $FT_CONTRACT_ID new '{
      "owner_id": "owner.near",
      "total_supply": "100000000000000",
      "metadata": {
         "spec": "ft-1.0.0",
         "name": "Bitcoin",
         "symbol": "BTC",
         "icon": "data:image/svg+xml,%3C…",
         "reference": "https://example.com/wbtc.json",
         "reference_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M=",
         "decimals": 8
      }
    }' --accountId owner.new

```


