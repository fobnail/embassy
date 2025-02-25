#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, unwrap};
use embassy::executor::Spawner;
use embassy_stm32::flash::Flash;
use embassy_stm32::Peripherals;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use {defmt_rtt as _, panic_probe as _};

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello Flash!");

    let mut f = Flash::unlock(p.FLASH);

    // Sector 5
    test_flash(&mut f, 128 * 1024, 128 * 1024);

    // Sectors 11..=16, across banks (128K, 16K, 16K, 16K, 16K, 64K)
    test_flash(&mut f, (1024 - 128) * 1024, 256 * 1024);

    // Sectors 23, last in bank 2
    test_flash(&mut f, (2048 - 128) * 1024, 128 * 1024);
}

fn test_flash(f: &mut Flash, offset: u32, size: u32) {
    info!("Testing offset: {=u32:#X}, size: {=u32:#X}", offset, size);

    info!("Reading...");
    let mut buf = [0u8; 32];
    unwrap!(f.read(offset, &mut buf));
    info!("Read: {=[u8]:x}", buf);

    info!("Erasing...");
    unwrap!(f.erase(offset, offset + size));

    info!("Reading...");
    let mut buf = [0u8; 32];
    unwrap!(f.read(offset, &mut buf));
    info!("Read after erase: {=[u8]:x}", buf);

    info!("Writing...");
    unwrap!(f.write(
        offset,
        &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32
        ]
    ));

    info!("Reading...");
    let mut buf = [0u8; 32];
    unwrap!(f.read(offset, &mut buf));
    info!("Read: {=[u8]:x}", buf);
    assert_eq!(
        &buf[..],
        &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32
        ]
    );
}
