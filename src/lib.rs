pub mod cortex_m33;
mod rp2350;

use std::any::Any;

use byteorder::{ByteOrder, LittleEndian};
pub use rp2350::*;

pub trait MemoryInterface<AddressType: num_traits::Unsigned + Copy> {
    fn read(&self, address: AddressType) -> u8;
    fn write(&mut self, address: AddressType, value: u8);


    fn read_u16(&self, address: AddressType) -> u16 {
        LittleEndian::read_u16(&[self.read(address), self.read(address + AddressType::one())])
    }

    fn read_u32(&self, address: AddressType) -> u32 {
        let one = AddressType::one();
        LittleEndian::read_u32(&[self.read(address), self.read(address + one), self.read(address + one + one), self.read(address + one + one + one)])
    }


    fn write_u16(&mut self, address: AddressType, value: u16) {
        let bytes = value.to_le_bytes();
        for (i, byte) in bytes.into_iter().enumerate() {
            let mut address = address;
            for _ in 0..i { address = address + AddressType::one(); };
            self.write(address, byte);
        }
    }

    fn write_u32(&mut self, address: AddressType, value: u32) {
        let bytes = value.to_le_bytes();
        for (i, byte) in bytes.into_iter().enumerate() {
            let mut address = address;
            for _ in 0..i { address = address + AddressType::one(); };
            self.write(address, byte);
        }
    }

    fn as_any(&self) -> &dyn Any;
}
