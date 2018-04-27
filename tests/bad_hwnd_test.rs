extern crate direct2d;
extern crate rand;
extern crate winapi;

use direct2d::render_target::HwndRenderTarget;
use direct2d::{Error, Factory};
use rand::{Rng, SeedableRng, XorShiftRng};

const SEED: [u32; 4] = [0x4695b3d0, 0x3e1e33b9, 0xaec57978, 0xd44c5bac];
const INVALID_WINDOW_HANDLE: i32 = 0x80070578u32 as i32; // HRESULT_FROM_WIN32(1400)

#[test]
fn random_hwnds_should_fail() {
    let mut rng = XorShiftRng::from_seed(SEED);
    let factory = Factory::new().unwrap();

    for hwnd in rng.gen_iter::<usize>().take(100_000) {
        let result = HwndRenderTarget::create(&factory)
            .with_hwnd(hwnd as _)
            .build();

        match result {
            Err(Error::ComError(e)) if e == INVALID_WINDOW_HANDLE => continue,
            Err(e) => panic!("Unexpected failure: {} ({:?})", e.get_message(), e),
            Ok(_) => panic!("Should fail, but didn't (0x{:x})", hwnd),
        }
    }
}
