# Vehicle Information Smart Contract
## version 0.1.0

###  Information provided
- Car mileage
- Timestamp
- Vehicle Identification Number (VIN)

### Security 
We chose to design a system in which the car is responsible for all data submitted to the blockchain in the following order for increased security:
- Each vehicle has a unique address in the elrond ecosystem.
- This address must be associated with the vehicle identification number (VIN).

### Main Logic
1. The device reads the vehicle identification number on the first start.
2. The device transmits the VIN to the smart contract.
>Until this point, greater security is achieved by binding the address to the VIN via the smart contract call, this way the vehicle can be identified by two unique addresses. Only the car owns the private key of the public address.
3. The device listens to the car and updates the mileage value on the blockchain every 1000 kilometers or miles.


#### With this approach on the blockchain we have the last value for the mileage and the corresponding timestamp. In order to form a history for a vehicle, we are collecting the data from previous transactions.


## Devnet address:

```
erd1qqqqqqqqqqqqqpgq83mmnl478cywl0fsxzuvu32f8enzm5za0huqq2p7ug
````
## Devnet Explorer:

[https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgq83mmnl478cywl0fsxzuvu32f8enzm5za0huqq2p7ug](https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgq83mmnl478cywl0fsxzuvu32f8enzm5za0huqq2p7ug)


## Interactions:
```
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq83mmnl478cywl0fsxzuvu32f8enzm5za0huqq2p7ug --chain="D" --pem="wallets/walletKey2.pem" --gas-limit=5000000 --function --arguments --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

```
