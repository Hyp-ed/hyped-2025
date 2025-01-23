#![no_std]

/// CAN errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enums/enum.BusError.html
/// and https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enum.TryWriteError.html
#[derive(Debug)]
pub enum CanError {
    Stuff,
    Form,
    Acknowledge,
    BitRecessive,
    BitDominant,
    Crc,
    Software,
    BusOff,
    BusPassive,
    BusWarning,
    Full,
    Unknown,
}
#[derive(Clone)]
pub struct CanFrame {
    can_id: u32,                        // 32 bit CAN_ID + EFF/RTR/ERR flags
    data: [u8; 8],                      // data that is sent over CAN, split into bytes
}

/// CAN trait used to abstract the CAN operations
pub trait HypedCan {
    async fn read_frame(&mut self) -> Option<u8>;
    fn try_read_frame(&mut self) -> Option<u8>;
    async fn write_frame(&mut self, message: &CanFrame) -> Result<(), CanError>;
    fn try_write_frame(&mut self, message: &CanFrame) -> Result<(), CanError>;
}

pub mod mock_can {
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
    use heapless::Deque;

    use crate::CanFrame;

    /// A fixed-size map of CAN values, indexed by device address and register address
    type CanValues = Deque<CanFrame, 8>;

    /// A mock CAN instance which can be used for testing
    pub struct MockCan<'a> {
        frames_to_read: &'a Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        frames_sent: CanValues,
    }

    impl crate::HypedCan for MockCan<'_> {
        /// Read a frame from the CAN bus
        async fn read_frame(&mut self) -> Option<u8> {
            self.frames_to_read.lock(|frames_to_read| {
                frames_to_read.borrow_mut().pop_front().map(|frame| frame.data[0])
            })
        }
    
        fn try_read_frame(&mut self) -> Option<u8> {
            self.frames_to_read.lock(|frames_to_read| {
                frames_to_read.borrow().front().map(|frame| frame.data[0])
            })
        }
    
        /// Write a CAN frame to the CAN bus
        async fn write_frame(&mut self, message: &super::CanFrame) -> Result<(), super::CanError> {
            
            match self.frames_sent.push_front(message.clone()) {
                Ok(_) => Ok(()),
                Err(_) => Err(super::CanError::Unknown),
            }
        }
    
        fn try_write_frame(&mut self, message: &super::CanFrame) -> Result<(), super::CanError> {
            match self.frames_sent.push_front(message.clone()) {
                Ok(_) => Ok(()),
                Err(_) => Err(super::CanError::Unknown),
            }
        }
    
    }
    
    impl MockCan<'_> {
        pub fn new(frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>) -> Self {
            MockCan { 
                frames_to_read,
                frames_sent: CanValues::new(),}
        }
        /// Get the values that have been sent over the CAN bus
        pub fn get_can(&self) -> &CanValues {
            &self.frames_sent
        }
    }
}
