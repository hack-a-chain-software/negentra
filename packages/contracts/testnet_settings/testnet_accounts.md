### Main contracts

```
NEGENTRA_TOKEN = negentra_token.testnet
NEGENTRA_NFT = negentra_nft.testnet
```

### Owner accounts
The owner account was used to initialize all contracts.  
Therefore this account is used as source for tokens and to alter
contract owner restricted configurations.

When testing normal user functions do not use the owner account, send
a reasonable amount of tokens to a newly created user account and
perform tests there.
```
OWNER_ALL = negentra_owner.testnet
```