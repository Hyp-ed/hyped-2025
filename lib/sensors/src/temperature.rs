use hyped_io::i2c::I2C;

pub struct Temperature<T: I2C> {}

impl<T: I2C> Temperature<T> {
    pub fn new(i2c: T) -> Temperature<T> {
        todo!()
    }
}
