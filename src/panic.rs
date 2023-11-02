use core::hint;
use core::panic::PanicInfo;

use crate::kprintln;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    kprintln!("panicked");
    loop {
        hint::spin_loop();
    }
}
