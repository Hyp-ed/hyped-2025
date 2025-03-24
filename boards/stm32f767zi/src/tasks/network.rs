/// Task for running the network stack
#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) -> ! {
    stack.run().await
}
