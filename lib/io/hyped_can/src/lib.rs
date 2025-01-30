#![no_std]

/// CAN errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enums/enum.BusError.html,
/// https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enum.TryWriteError.html
/// and https://docs.embassy.dev/embassy-stm32/git/stm32f767zi/can/enum.TryReadError.html
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
}
#[derive(Clone)]
pub struct CanFrame {
    can_id: u32,   // 32 bit CAN_ID + EFF/RTR/ERR flags
    data: [u8; 8], // data that is sent over CAN, split into bytes
}

pub type Timestamp = embassy_time::Instant;

#[derive(Clone)]
pub struct Envelope {
    /// Reception time.
    pub ts: Timestamp,
    /// The actual CAN frame.
    pub frame: CanFrame,
}

impl Envelope {
    /// Convert into a tuple
    pub fn parts(self) -> (CanFrame, Timestamp) {
        (self.frame, self.ts)
    }
}

/// CAN trait used to abstract the CAN operations
pub trait HypedCan {
    /// Read a CAN frame.
    ///
    /// If no CAN frame is in the RX buffer, this will wait until there is one.
    /// Returns a tuple of the time the message was received and the message frame
    async fn read_frame(&mut self) -> Result<Envelope, CanError>;
    /// Attempts to read a CAN frame without blocking.
    ///
    /// Returns [Err(TryReadError::Empty)] if there are no frames in the rx queue.
    fn try_read_frame(&mut self) -> Result<Envelope, CanError>;
    /// Queues the message to be sent.
    ///
    /// If the TX queue is full, this will wait until there is space, therefore exerting backpressure.
    async fn write_frame(&mut self, frame: &CanFrame) -> ();
    /// Attempts to transmit a frame without blocking.
    ///
    /// Returns [Err(CanError::Full)] if the frame can not be queued for transmission now.
    ///
    /// If FIFO scheduling is enabled, any empty mailbox will be used.
    ///
    /// Otherwise, the frame will only be accepted if there is no frame with the same priority already queued. This is done
    /// to work around a hardware limitation that could lead to out-of-order delivery of frames with the same priority.
    fn try_write_frame(&mut self, frame: &CanFrame) -> Result<(), CanError>;
}

pub mod mock_can {
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
    use heapless::Deque;

    use crate::CanFrame;

    /// A fixed-size map of CAN frames
    type CanValues = Deque<CanFrame, 8>;

    /// A mock CAN instance which can be used for testing
    pub struct MockCan<'a> {
        frames_to_read: &'a Mutex<CriticalSectionRawMutex, RefCell<CanValues>>,
        frames_sent: CanValues,
        try_outcomes: bool,
    }

    impl crate::HypedCan for MockCan<'_> {
        /// Read a frame from the CAN bus
        async fn read_frame(&mut self) -> Result<super::Envelope, super::CanError> {
            self.frames_to_read.lock(|frames_to_read| {
                match frames_to_read.borrow_mut().pop_front() {
                    Some(frame) => Ok(super::Envelope {
                        ts: embassy_time::Instant::now(),
                        frame,
                    }),
                    None => Err(super::CanError::Unknown),
                }
            })
        }

        fn try_read_frame(&mut self) -> Result<super::Envelope, super::CanError> {
            if !self.try_outcomes {
                return Err(super::CanError::Unknown);
            }
            self.frames_to_read.lock(|frames_to_read| {
                match frames_to_read.borrow_mut().pop_front() {
                    Some(frame) => Ok(super::Envelope {
                        ts: embassy_time::Instant::now(),
                        frame,
                    }),
                    None => Err(super::CanError::Empty),
                }
            })
        }

        async fn write_frame(&mut self, frame: &super::CanFrame) -> () {
            self.frames_sent.push_front(frame.clone());
        }

        fn try_write_frame(&mut self, frame: &super::CanFrame) -> Result<(), super::CanError> {
            if !self.try_outcomes {
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
            MockCan {
                frames_to_read,
                frames_sent: CanValues::new(),
                try_outcomes: true,
            }
        }
        /// Get the values that have been sent over the CAN bus
        pub fn get_can_frames(&self) -> &CanValues {
            &self.frames_sent
        }

        pub fn set_try_to_fail(&mut self) {
            self.try_outcomes = false;
        }

        pub fn set_try_to_pass(&mut self) {
            self.try_outcomes = true;
        }
    }
}
