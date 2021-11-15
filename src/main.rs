#![no_std]
#![no_main]
#![feature(adt_const_params)]
#![feature(derive_default_enum)]

#[macro_use]
extern crate nx;

#[macro_use]
extern crate alloc;

extern crate paste;

use nx::result::*;
use nx::util;
use nx::thread;
use nx::diag::assert;
use nx::diag::log;
use nx::ipc::server;
use nx::version;
use nx::fs;
use core::panic;

mod ns;
use ns::IApplicationManagerInterface;

mod hb;

const STACK_HEAP_SIZE: usize = 0x80000;
static mut STACK_HEAP: [u8; STACK_HEAP_SIZE] = [0; STACK_HEAP_SIZE];

#[no_mangle]
pub fn initialize_heap(_hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    unsafe {
        util::PointerAndSize::new(STACK_HEAP.as_mut_ptr(), STACK_HEAP.len())
    }
}

const POINTER_BUF_SIZE: usize = 0x1000;
type Manager = server::ServerManager<POINTER_BUF_SIZE>;

#[no_mangle]
pub fn main() -> Result<()> {
    thread::get_current_thread().name.set_str("nashe.Main")?;
    diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "[nashe] Hello there!\n");

    fs::initialize()?;
    fs::mount_sd_card("sdmc")?;
    ns::client::initialize()?;
    diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "[nashe] Everything initialized!\n");

    let mut manager = Manager::new()?;
    manager.register_mitm_service_server::<ns::mitm::ServiceGetterInterface<{ns::GetterServiceKind::AM2}>>()?;

    diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "[nashe] Looping...\n");
    manager.loop_process()?;

    // This won't be reached anyway...
    ns::client::finalize();
    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertLevel::SvcBreak())
}