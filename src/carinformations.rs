#![no_std]

elrond_wasm::imports!();


#[elrond_wasm::derive::contract]
pub trait Vehick {
    #[init]
    fn init(&self) {
    }


    #[endpoint(addVin)]
    fn add_vin(&self,vinb: ManagedBuffer) {
         let caller = self.blockchain().get_caller();
         require!(self.vin_number(&caller).get().is_empty(),"Vehicle Identification Number exists!");
         self.vin_number(&caller).update(|vin| *vin= vinb );
    }

    #[endpoint(addMeasureUnit)]
    fn add_unit(&self,measure:ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        require!(self.measure_unit(&caller).get().is_empty(),"Measure unit exists");
        self.measure_unit(&caller).update(|unit| *unit= measure );
       
   }

    #[endpoint(addMileage)]
    fn add_mileage(&self, value: BigUint) {
        let caller = self.blockchain().get_caller();
        let prev_mileage= self.mileage(&caller).get();
        require!(!self.vin_number(&caller).get().is_empty(),"Need to add Vehicle Identification Number first");
        require!(prev_mileage < value,"Mileage unreal");
        self.mileage(&caller).update(|mileage| *mileage = value);

    }

    #[view(getMileage)]
    #[storage_mapper("mileage")]
    fn mileage(&self, car_address:&ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getVin)]
    #[storage_mapper("vin")]
    fn vin_number(&self, vin:&ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[view(getMeasureUnit)]
    #[storage_mapper("unit")]
    fn measure_unit(&self, measure:&ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

}
