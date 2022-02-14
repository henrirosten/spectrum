// SPDX-License-Identifier: EUPL-1.2
// SPDX-FileCopyrightText: 2022 Alyssa Ross <hi@alyssa.is>

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct NetConfig {
    pub fd: i32,
    pub mac: [u8; 6],
}

extern "C" {
    pub fn net_setup(provider_vm_name: *const c_char) -> NetConfig;
}

pub fn format_mac(mac: &[u8; 6]) -> String {
    extern "C" {
        fn format_mac(s: *mut c_char, mac: *const [u8; 6]) -> c_int;
    }

    let mut s = vec![0; 18];

    // Safe because s and mac are correctly sized.
    assert_ne!(unsafe { format_mac(s.as_mut_ptr() as _, mac) }, -1);

    // Drop the null byte.
    s.pop();

    // Safe because a formatted MAC address is always UTF-8.
    unsafe { String::from_utf8_unchecked(s) }
}