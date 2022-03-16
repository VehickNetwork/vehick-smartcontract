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
1. At the assembly line, the gadget is fitted in the vehicle.
2. The device reads the vehicle identification number and the odometer value on the first start.
3. The device transmits the VIN to the smart contract and awaits confirmation.
>Until this point, greater security is achieved by binding the address to the VIN via the smart contract call, this way the vehicle can be indentified by two unique addresses.
4. The device listens to the car and updates the mileage value on the blockchain every 1000 kilometers or miles.

##### Using this aproach we don't need extra function in the smart contract for gathering back the information that was sent already. 


### Example 

*Car address:* **vehicle.address.elrond**
*Smart Contract address:* **sc.address.elrond**

```sh
{
    ...
    "receiver": sc.address.elrond
    "sender": car.address.elrond
    ...
    "timestamp": timestamp
    "data": hexEncoded(addVIN@hexEncoded(VIN))
    
} 
{
    ...
    "receiver": sc.address.elrond
    "sender": car.address.elrond
    ...
    "timestamp": timestamp
    "data": hexEncoded(addMileage@hexEncoded(mileage))
    
}

```
##### We filter successful transactions and link the information together for each vehicle.
#### With this approach on the blockchain we have the last value for the mileage and the corresponding timestamp. In order to form a history for a vehicle, we are collecting the data from previous transactions.

