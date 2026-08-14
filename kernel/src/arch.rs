use crate::arch::arm64::assembly::interrupt;

pub mod arm64;

pub fn init()
{
    arm64::exception::handlers::init();
    
    unsafe 
    {
        interrupt::enable_interrupts();
    }
}