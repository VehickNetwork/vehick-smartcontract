#![no_std]

elrond_wasm::imports!();


#[elrond_wasm::derive::contract]
pub trait CarInfo {
    #[view(getMileage)]
    #[storage_mapper("mileage")]
    fn mileage(&self, car_address:&ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getVIN)]
    #[storage_mapper("vin")]
    fn vin_number(&self, vin:&ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[view(getMeasureUnit)]
    #[storage_mapper("unit")]
    fn measure_unit(&self, measure:&ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[init]
    fn init(&self) {
    }


    #[endpoint(addVIN)]
    fn add_vin(&self,vinb: ManagedBuffer) -> SCResult<()>{
         let caller = self.blockchain().get_caller();
         self.vin_number(&caller).update(|vin| *vin= vinb );
        Ok(())
    }

    #[endpoint(addMeasureUnit)]
    fn add_unit(&self,measure: ManagedBuffer) -> SCResult<()>{
        let caller = self.blockchain().get_caller();
        self.measure_unit(&caller).update(|unit| *unit= measure );
       Ok(())
   }

    #[endpoint(addMileage)]
    fn add_mileage(&self, value: BigUint) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let prev_mileage= self.mileage(&caller).get();
        require!(!self.vin_number(&caller).get().is_empty(),"Need to add VIN First");
        require!(prev_mileage < value,"Mileage unreal");
        self.mileage(&caller).update(|mileage| *mileage = value);
        Ok(())
    }


}
