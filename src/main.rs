#![allow(non_upper_case_globals)]

use chrono::Utc;
use core_foundation::base::CFIndexConvertible;
use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURLRef, CFURL};
use core_graphics::{display, image::CGImage, window};
use foreign_types::ForeignType;
use std::ffi::c_void;
use std::ptr;

fn main() {
    let x = window::copy_window_info(
        window::kCGWindowListOptionOnScreenOnly | window::kCGWindowListOptionExcludeDesktopElements,
        window::kCGNullWindowID,
    )
    .unwrap();

    // println!("{:?}", x);

    let elems = [67, 499];
    // let elems = [1];

    unsafe {
        let image = window::CGWindowListCreateImageFromArray(
            display::CGRectInfinite,
            core_foundation::array::CFArrayCreate(
                ptr::null_mut(),
                elems.as_ptr() as *const *const c_void,
                elems.len().to_CFIndex(),
                &core_foundation::array::kCFTypeArrayCallBacks,
            ),
            display::kCGWindowImageBestResolution,
        );

        assert!(!image.is_null());

        let screenshot = CGImage::from_ptr(image);
        let path = format!("./out/screenshot_{}.png", Utc::now().timestamp_millis());
        let destination = CGImageDestinationCreateWithURL(
            CFURL::from_path(path, false).unwrap().as_concrete_TypeRef(),
            CFString::new("public.png").as_concrete_TypeRef(),
            1,
            ptr::null(),
        );

        assert!(!destination.is_null());

        CGImageDestinationAddImage(destination, screenshot, ptr::null());
        assert!(CGImageDestinationFinalize(destination));
    }
}

pub enum CGImageDestination {}
pub type CGImageDestinationRef = *mut CGImageDestination;

#[link(name = "ImageIO", kind = "framework")]
extern "C" {
    pub fn CGImageDestinationCreateWithURL(
        url: CFURLRef,
        type_: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
    pub fn CGImageDestinationFinalize(idst: CGImageDestinationRef) -> bool;
    pub fn CGImageDestinationAddImage(
        idst: CGImageDestinationRef,
        image: CGImage,
        properties: CFDictionaryRef,
    );
}
