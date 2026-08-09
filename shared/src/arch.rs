pub mod arm64;

pub fn init()
{
    arm64::exception::handlers::init();
}