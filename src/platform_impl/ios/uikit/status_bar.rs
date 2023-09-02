use objc2::encode::{Encode, Encoding};
use objc2::foundation::NSInteger;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(isize)]
pub enum UIStatusBarStyle {
    Default = 0,
    LightContent = 1,
    DarkContent = 3,
}

unsafe impl Encode for UIStatusBarStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}
