//crate lvl macro
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

//iteam lvl macro
#[entry]
fn main() -> ! {
    let _hello = "Welcome";
    loop {}
}
