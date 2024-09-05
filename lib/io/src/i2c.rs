pub trait HypedI2c {
    fn read_byte(&self, device_address: u8, register_address: u8) -> u8;
    fn write_byte_to_register(
        &self,
        device_addres: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), ()>;
    fn write_byte(&self, device_address: u8, data: u8) -> Result<(), ()>;
}

pub mod mock_i2c {
    pub struct MockI2C {}

    impl crate::i2c::HypedI2c for MockI2C {
        fn read_byte(&self, device_address: u8, register_address: u8) -> u8 {
            todo!()
        }
        fn write_byte(&self, device_address: u8, data: u8) -> Result<(), ()> {
            todo!()
        }
        fn write_byte_to_register(
            &self,
            device_addres: u8,
            register_address: u8,
            data: u8,
        ) -> Result<(), ()> {
            todo!()
        }
    }

    impl MockI2C {
        pub fn new() -> Self {
            todo!()
        }
    }
}
