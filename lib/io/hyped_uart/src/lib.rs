#![no_std]

pub trait HypedUart {
    // Add required functions here
}

pub mod mock_adc {
    /// A mock UART instance which can be used for testing
    pub struct MockUart {}

    impl crate::HypedUart for MockUart {
        // Add required functions here
    }

    impl MockUart {
        pub fn new() -> Self {
            MockUart {}
        }
    }
}
