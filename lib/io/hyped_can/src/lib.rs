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

#[derive(Clone, Copy, Debug, defmt::Format)]
pub struct HypedCanFrame {
    pub can_id: u32,   // 32 bit CAN_ID + EFF/RTR/ERR flags
    pub data: [u8; 8], // data that is sent over CAN, split into bytes
}

impl HypedCanFrame {
    pub fn new(can_id: u32, data: [u8; 8]) -> Self {
        HypedCanFrame { can_id, data }
    }
}

pub type Timestamp = embassy_time::Instant;

#[derive(Clone, Debug)]
pub struct HypedEnvelope {
    /// Reception time.
    pub ts: Timestamp,
    /// The actual CAN frame.
    pub frame: HypedCanFrame,
}

pub trait HypedCan {
    /// Attempts to read a CAN frame without blocking.
    ///
    /// Returns [Err(TryReadError::Empty)] if there are no frames in the rx queue.
    fn read_frame(&mut self) -> Result<HypedEnvelope, CanError>;
    /// /// Attempts to transmit a frame without blocking.
    ///
    /// Returns [Err(CanError::Full)] if the frame can not be queued for transmission now.
    ///
    /// The frame will only be accepted if there is no frame with the same priority already queued. This is done
    /// to work around a hardware limitation that could lead to out-of-order delivery of frames with the same priority.
    fn write_frame(&mut self, frame: &HypedCanFrame) -> Result<(), CanError>;
}

/// A CAN interface for sending CAN frames
pub trait HypedCanTx {
    /// Attempts to transmit a frame without blocking.
    ///
    /// Returns [Err(CanError::Full)] if the frame can not be queued for transmission now.
    ///
    /// The frame will only be accepted if there is no frame with the same priority already queued. This is done
    /// to work around a hardware limitation that could lead to out-of-order delivery of frames with the same priority.
    fn write_frame(&mut self, frame: &HypedCanFrame) -> Result<(), CanError>;
}

/// A CAN interface for receiving CAN frames
///
/// (Will probably not be used because we'll be putting received CAN frames into a channel
/// and calling a callback function to handle them.)
pub trait HypedCanRx {
    /// /// Attempts to read a CAN frame without blocking.
    ///
    /// Returns [Err(TryReadError::Empty)] if there are no frames in the rx queue.
    fn read_frame(&mut self) -> Result<HypedEnvelope, CanError>;
}

pub mod mock_can {
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
    use heapless::Deque;

    use crate::{HypedCan, HypedCanFrame};

    /// A fixed-size map of CAN frames
    type CanValues = Deque<HypedCanFrame, 8>;

    /// A mock CAN instance which can be used for testing
    pub struct MockCan<'a> {
        /// Values that have been read from the CAN bus
        frames_to_read: &'a Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        /// Values that have been sent over the CAN bus
        frames_sent: CanValues,
        /// Whether to fail reading frames
        fail_read: &'a Mutex<CriticalSectionRawMutex, bool>,
        /// Whether to fail writing frames
        fail_write: &'a Mutex<CriticalSectionRawMutex, bool>,
    }

    impl HypedCan for MockCan<'_> {
        fn read_frame(&mut self) -> Result<super::HypedEnvelope, super::CanError> {
            if self.fail_read.lock(|fail_read| *fail_read) {
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
            if self.fail_write.lock(|fail_write| *fail_write) {
                return Err(super::CanError::Unknown);
            }
            match self.frames_sent.push_front(*frame) {
                Ok(_) => Ok(()),
                Err(_) => Err(super::CanError::Unknown),
            }
        }
    }

    impl MockCan<'_> {
        pub fn new(
            frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        ) -> Self {
            static FAIL_READ: Mutex<CriticalSectionRawMutex, bool> = Mutex::new(false);
            static FAIL_WRITE: Mutex<CriticalSectionRawMutex, bool> = Mutex::new(false);
            MockCan::new_with_failures(frames_to_read, &FAIL_READ, &FAIL_WRITE)
        }

        pub fn new_with_failures(
            frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
            fail_read: &'static Mutex<CriticalSectionRawMutex, bool>,
            fail_write: &'static Mutex<CriticalSectionRawMutex, bool>,
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
    }

    /// A mock CAN instance which can be used for testing
    pub struct MockCanRx<'a> {
        /// Values that have been read from the CAN bus
        frames_to_read: &'a Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        /// Whether to fail reading frames
        fail_read: &'a Mutex<CriticalSectionRawMutex, bool>,
    }

    pub struct MockCanTx<'a> {
        /// Values that have been sent over the CAN bus
        frames_sent: CanValues,
        /// Whether to fail writing frames
        fail_write: &'a Mutex<CriticalSectionRawMutex, bool>,
    }

    impl crate::HypedCanRx for MockCanRx<'_> {
        fn read_frame(&mut self) -> Result<super::HypedEnvelope, super::CanError> {
            if self.fail_read.lock(|fail_read| *fail_read) {
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
    }

    impl crate::HypedCanTx for MockCanTx<'_> {
        fn write_frame(&mut self, frame: &super::HypedCanFrame) -> Result<(), super::CanError> {
            if self.fail_write.lock(|fail_write| *fail_write) {
                return Err(super::CanError::Unknown);
            }
            match self.frames_sent.push_front(*frame) {
                Ok(_) => Ok(()),
                Err(_) => Err(super::CanError::Unknown),
            }
        }
    }

    impl MockCanRx<'_> {
        pub fn new(
            frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        ) -> Self {
            static FAIL_READ: Mutex<CriticalSectionRawMutex, bool> = Mutex::new(false);
            MockCanRx::new_with_failure(frames_to_read, &FAIL_READ)
        }

        pub fn new_with_failure(
            frames_to_read: &'static Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
            fail_read: &'static Mutex<CriticalSectionRawMutex, bool>,
        ) -> Self {
            MockCanRx {
                frames_to_read,
                fail_read,
            }
        }
    }

    impl MockCanTx<'_> {
        pub fn new() -> Self {
            static FAIL_WRITE: Mutex<CriticalSectionRawMutex, bool> = Mutex::new(false);
            MockCanTx::new_with_failure(&FAIL_WRITE)
        }

        pub fn new_with_failure(fail_write: &'static Mutex<CriticalSectionRawMutex, bool>) -> Self {
            MockCanTx {
                frames_sent: CanValues::new(),
                fail_write,
            }
        }

        pub fn frames_sent(&self) -> &CanValues {
            &self.frames_sent
        }
    }

    impl Default for MockCanTx<'_> {
        fn default() -> Self {
            MockCanTx::new()
        }
    }
}
