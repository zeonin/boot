#![feature(asm, lang_items, naked_functions, start)]

#![no_std]
#![no_main]

extern crate uefi;
extern crate rlibc;

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}

#[start]
#[no_mangle]
pub extern fn efi_main(image_handle:    uefi::EFI_HANDLE,
                       system_table:    *const uefi::EFI_SYSTEM_TABLE) -> isize {
    let hello = ['H' as u16,
                 'e' as u16,
                 'l' as u16,
                 'l' as u16,
                 'o' as u16,
                 ' ' as u16,
                 'W' as u16,
                 'o' as u16,
                 'r' as u16,
                 'l' as u16,
                 'd' as u16,
                 '!' as u16,
                 '\n' as u16,
                 0 as u16];

    unsafe {
        let conout = (*system_table).ConOut;
        let output = (*conout).OutputString;

        let hello_ptr = &hello as *const u16;

        output(conout, hello_ptr);
    }
    loop {}
    0
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}
