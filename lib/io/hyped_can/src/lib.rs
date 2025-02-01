#![no_std]

/// CAN errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enums/enum.BusError.html,
/// https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enum.TryWriteError.html
/// https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enum.TryReadError.html,
/// and https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enums/enum.FrameCreateError.html
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
    Empty,
    Unknown,
    NotEnoughData,
    InvalidDataLength,
    InvalidCanId,
}

#[derive(Clone)]
pub struct HypedCanFrame {
    pub can_id: u32,   // 32 bit CAN_ID + EFF/RTR/ERR flags
    pub data: [u8; 8], // data that is sent over CAN, split into bytes
}

pub type Timestamp = embassy_time::Instant;

#[derive(Clone)]
pub struct HypedEnvelope {
    /// Reception time.
    pub ts: Timestamp,
    /// The actual CAN frame.
    pub frame: HypedCanFrame,
}

/// CAN trait used to abstract the CAN operations
pub trait HypedCan {
    /// Attempts to read a CAN frame without blocking.
    ///
    /// Returns [Err(TryReadError::Empty)] if there are no frames in the rx queue.
    fn read_frame(&mut self) -> Result<HypedEnvelope, CanError>;
    /// Attempts to transmit a frame without blocking.
    ///
    /// Returns [Err(CanError::Full)] if the frame can not be queued for transmission now.
    ///
    /// The frame will only be accepted if there is no frame with the same priority already queued. This is done
    /// to work around a hardware limitation that could lead to out-of-order delivery of frames with the same priority.
    fn write_frame(&mut self, frame: &HypedCanFrame) -> Result<(), CanError>;
}

pub mod mock_can {
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
    use heapless::Deque;

    use crate::HypedCanFrame;

    /// A fixed-size map of CAN frames
    type CanValues = Deque<HypedCanFrame, 8>;

    /// A mock CAN instance which can be used for testing
    pub struct MockCan<'a> {
        /// Values that have been read from the CAN bus
        frames_to_read: &'a Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        /// Values that have been sent over the CAN bus
        frames_sent: CanValues,
        /// Whether to fail reading frames
        fail_read: bool,
        /// Whether to fail writing frames
        fail_write: bool,
    }

    impl crate::HypedCan for MockCan<'_> {
        fn read_frame(&mut self) -> Result<super::HypedEnvelope, super::CanError> {
            if self.fail_read {
                return Err(super::CanError::Unknown);
            }
            self.frames_to_read.lock(|frames_to_read| {
                match frames_to_read.borrow_mut().pop_front() {
                    Some(frame) => Ok(super::HypedEnvelope {
                        ts: embassy_time::Instant::now(),
                        frame,
                    }),
                    None => Err(super::CanError::Empty),
                }
            })
        }

        fn write_frame(&mut self, frame: &super::HypedCanFrame) -> Result<(), super::CanError> {
            if self.fail_write {
                return Err(super::CanError::Unknown);
            }
            match self.frames_sent.push_front(frame.clone()) {
                Ok(_) => Ok(()),
                Err(_) => Err(super::CanError::Unknown),
            }
        }
    }

    impl MockCan<'_> {
        pub fn new(
            frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        ) -> Self {
            MockCan::new_with_failures(frames_to_read, false, false)
        }

        pub fn new_with_failures(
            frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
            fail_read: bool,
            fail_write: bool,
        ) -> Self {
            MockCan {
                frames_to_read,
                frames_sent: CanValues::new(),
                fail_read,
                fail_write,
            }
        }

        /// Get the values that have been sent over the CAN bus
        pub fn get_can_frames(&self) -> &CanValues {
            &self.frames_sent
        }

        pub fn set_read_to_fail(&mut self) {
            self.fail_read = true;
        }

        pub fn set_write_to_fail(&mut self) {
            self.fail_write = true;
        }

        pub fn set_read_to_pass(&mut self) {
            self.fail_read = false;
        }

        pub fn set_write_to_pass(&mut self) {
            self.fail_write = false;
        }
    }
}
