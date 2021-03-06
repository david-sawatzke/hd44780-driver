use embedded_hal::blocking::delay::{DelayMs, DelayUs};

mod fourbit;

pub use self::fourbit::FourBitBus;

pub trait DataBus {
    fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D);

    // TODO
    // fn read(...)
}
