/*
 * Frontend for the Peanut-GB emulator.
 * ROMs are loaded from the filesystem, and the Game Boy screen is rendered to the framebuffer.
 *
 * Author: Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-04-01
 * License: GPLv3
 */

use alloc::vec::{self, Vec};
use core::ffi::{c_char, c_int, c_size_t, c_void, CStr};
use log::{error, info};
use crate::{device::{key::Scancode, keyboard::keyboard_buffer, pit, serial::COM3, terminal::{framebuffer, terminal}}, filesystem::tarfs::filesystem, library::{once::Once, spinlock::Spinlock}};

unsafe extern "C" {
    /// Get the size of the `gb_s` structure (implemented in `peanut-gb.c`).
    /// This struct holds the entire state of the emulated Game Boy.
    /// Since we do not have a Rust binding for this, we use a C function to get the size.
    fn gb_size() -> c_int;

    /// Get a pointer to the joypad state in the `gb_s` structure (implemented in `peanut-gb.c`).
    /// The joypad state is a single byte where each bit represents a button state.
    /// If no button is pressed, all bits are set to 1 (0xff).
    /// The buttons are represented by the `JoypadButton` enum.
    fn gb_get_joypad_ptr(gb: *mut c_void) -> *mut u8;

    /// Initialization function for the PeanutGB emulator.
    /// The `gb` parameter must point to block of memory large enough to hold the `gb_s` structure.
    /// The size of this structure can be obtained by calling `gb_size()`.
    /// The `priv_data` parameter can be used to pass additional data to the emulator,
    /// but is currently unused in this implementation.
    /// The other parameters are function pointers and crucial for the emulator to function.
    fn gb_init(gb: *mut c_void,
               gb_rom_read: unsafe extern "C" fn(*mut c_void, u32) -> u8,
               gb_cart_ram_read: unsafe extern "C" fn(*mut c_void, u32) -> u8,
               gb_cart_ram_write: unsafe extern "C" fn(*mut c_void, u32, u8),
               gb_error: unsafe extern "C" fn(*mut c_void, i32, u16),
               priv_data: *const c_void) -> c_int;

    /// Initialize the LCD of the PeanutGB emulator.
    /// This function must be called after the emulator has been initialized.
    /// If this function is not called, the emulator will work, but not render any graphics.
    fn gb_init_lcd(gb: *mut c_void, lcd_draw_line: *const c_void);

    /// Run a single frame of the PeanutGB emulator.
    /// This function must be called in a loop to run the emulator.
    /// To maintain a stable frame rate, the caller should measure the time taken by this function
    /// and sleep for the remaining time to achieve the desired frame rate.
    /// Otherwise, the emulator will run as fast as possible.
    fn gb_run_frame(gb: *mut c_void);

    /// Get the name of the ROM currently loaded in the PeanutGB emulator.
    /// The name is returned as a C string (null-terminated).
    fn gb_get_rom_name(gb: *mut c_void, title_str: *const c_char) -> *const c_char;

    /// Get the RAM size of the currently loaded ROM in the PeanutGB emulator.
    /// The RAM size is written to the given pointer `ram_size`.
    /// A return value of 0 indicates success.
    fn gb_get_save_size_s(gb: *mut c_void, ram_size: *mut c_size_t) -> c_int;
}

/// Bitmask for the joypad buttons. See `gb_get_joypad_ptr` for more details.
#[repr(u8)]
enum JoypadButton {
    A = 0x01,
    B = 0x02,
    Select = 0x04,
    Start = 0x08,
    Right = 0x10,
    Left = 0x20,
    Up = 0x40,
    Down = 0x80,
}

/// Error codes used in `gb_error`.
#[derive(Debug, PartialEq)]
enum GbError {
    UnknownError = 0,
    InvalidOpcode = 1,
    InvalidRead = 2,
    InvalidWrite = 3,
}

impl TryFrom<c_int> for GbError {
    type Error = ();

    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GbError::UnknownError),
            1 => Ok(GbError::InvalidOpcode),
            2 => Ok(GbError::InvalidRead),
            3 => Ok(GbError::InvalidWrite),
            _ => Err(())
        }
    }
}

/// Error codes used in `gb_init`.
#[derive(Debug, PartialEq)]
enum GbInitError {
    NoError = 0,
    CartridgeUnsupported,
    InvalidChecksum,
    UnknownError = 0xff
}

impl TryFrom<c_int> for GbInitError {
    type Error = ();

    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GbInitError::NoError),
            1 => Ok(GbInitError::CartridgeUnsupported),
            2 => Ok(GbInitError::InvalidChecksum),
            3 => Ok(GbInitError::UnknownError),
            _ => Err(())
        }
    }
}

/// The target frame rate for the emulator.
/// The original Game Boy runs at 60 frames per second.
/// Increasing this value will make the emulator run faster,
/// decreasing it will make the emulator run slower.
const TARGET_FRAME_RATE: usize = 60;

/// The number of milliseconds per frame at the target frame rate.
const MS_PER_FRAME: usize = 1000 / TARGET_FRAME_RATE;

/// The original Game Boy screen resolution (160x144 pixels).
const GB_SCREEN_RES: (usize, usize) = (160, 144);

/// The color palette used for rendering.
/// The Game Boy supports 4 shades of gray, represented as 32-bit ARGB colors in this array.
static PALETTE: &[u32] = &[
    0xe0f8d0, // White
    0x88c070, // Light Gray
    0x346856, // Dark Gray
    0x081820, // Black
];

/// The ROM file to be played by the emulator.
static ROM: Once<Vec<u8>> = Once::new();

/// Vec that contains the RAM
static RAM: Once<Spinlock<Vec<u8>>> = Once::new();

/// Read a byte from the ROM file at the offset specified by `addr`.
/// This is a callback function for the PeanutGB emulator.
unsafe extern "C" fn gb_rom_read(_gb: *mut c_void, addr: u32) -> u8 {
    ROM.get().unwrap()[addr as usize]
}

/// Read a byte from the save RAM at the offset specified by `addr`.
/// This is a callback function for the PeanutGB emulator.
///
/// This is mostly needed for save game support and part of an optional assignment.
unsafe extern "C" fn gb_cart_ram_read(_gb: *mut c_void, addr: u32) -> u8 {
    RAM.get().unwrap().lock()[addr as usize]
}

/// Write a byte to the save RAM at the offset specified by `addr`.
/// This is a callback function for the PeanutGB emulator.
///
/// This is mostly needed for save game support and part of an optional assignment.
unsafe extern "C" fn gb_cart_ram_write(_gb: *mut c_void, addr: u32, val: u8) {
    RAM.get().unwrap().lock()[addr as usize] = val;
}

/// Draw a line of pixels from the Game Boy screen to the framebuffer.
/// The buffer pointed to by `pixels` contains the pixel data for the line.
/// Each pixel is represented by a single byte, whose first two bits represent the color index.
/// The other bits are used for Game Boy Color emulation, but are ignored in this implementation.
unsafe extern "C" fn lcd_draw_line(_gb: *mut c_void, pixels: *const u8, line: u8) {
    let data: &[u8] = unsafe {
        core::slice::from_raw_parts(pixels, 160)
    };

    let mut x = 0;
    for byte in data {
        let bits = (byte & 0b11) as usize;
        unsafe { framebuffer().lock().draw_pixel_unchecked(x, line as usize, PALETTE[bits]) };
        x += 1;
    }
}

/// Handle emulation errors.
/// This is a callback function for the PeanutGB emulator.
unsafe extern "C" fn gb_error(_gb: *mut c_void, error: c_int, addr: u16) {
    let error = GbError::try_from(error).unwrap_or(GbError::UnknownError);
    error!("PeanutGB error [{:?}] at address [0x{:0>4x}]!", error, addr);
}

/// Play the given ROM file using the Peanut-GB emulator.
pub fn play(rom_path: &str) {
    let my_handle = filesystem().open(rom_path).unwrap();
    let key_ev_queue = keyboard_buffer();

    // Read ROM
    let rom_size = filesystem().size(my_handle).unwrap();
    ROM.init(|| {
        let mut my_buff = Vec::<u8>::with_capacity(rom_size);
        my_buff.resize(rom_size, 0);
        let _ = filesystem().read(my_handle, &mut my_buff);

        my_buff
        }
    );

    let mut gbs = Vec::<u8>::with_capacity( unsafe { gb_size() } as usize );
    unsafe { gbs.set_len( gb_size() as usize); }
    let gbs_ptr = gbs.as_mut_ptr() as *mut c_void;

    // Initalise gameboy
    unsafe { gb_init(gbs_ptr, 
        gb_rom_read, 
        gb_cart_ram_read, 
        gb_cart_ram_write, 
        gb_error, 
        core::ptr::null()) 
    };

    // RAM stuff
    let cs: c_size_t = 0;
    let success;
    unsafe { success = gb_get_save_size_s(gbs_ptr, cs as *mut c_size_t); }

    if success != 0 {
        panic!("Could not get RAM size.");
    }

    RAM.init(|| {
        let mut my_buff = Vec::<u8>::with_capacity(rom_size);
        my_buff.resize(rom_size, 0);

        Spinlock::new(my_buff)
    });

    // Initialsie display
    unsafe { gb_init_lcd(gbs_ptr, lcd_draw_line as *const c_void) };

    terminal().lock().set_pos(0, 200);
    println!("+++ Controls +++");
    println!("Up/down/Left/Right: Arrow Keys");
    println!("A/B: A and B on Keyboard respectively");
    println!("Start: Enter");
    println!("Select: Spacebar");
    println!("Quit: Escape");

    // Game loop
    loop {
        if let Some(key_event) = key_ev_queue.pop_key_event() {
            let joy_state = unsafe { gb_get_joypad_ptr(gbs_ptr) };
            unsafe {
                match key_event.scancode() {
                    Some(Scancode::Up) => {
                        *joy_state ^= JoypadButton::Up as u8;
                        info!("UP");
                    },
                    Some(Scancode::Down) => {
                        *joy_state ^= JoypadButton::Down as u8;
                        info!("DOWN");
                    },
                    Some(Scancode::Left) => {
                        *joy_state ^= JoypadButton::Left as u8;
                        info!("LEFT");
                    },
                    Some(Scancode::Right) => {
                        *joy_state ^= JoypadButton::Right as u8;
                        info!("RIGHT");
                    },
                    Some(Scancode::A) => {
                        *joy_state ^= JoypadButton::A as u8;
                        info!("A");
                    },
                    Some(Scancode::B) => {
                        *joy_state ^= JoypadButton::B as u8;
                        info!("B");
                    },
                    Some(Scancode::Space) => { // select
                        *joy_state ^= JoypadButton::Select as u8;
                        info!("SPACE");
                    },
                    Some(Scancode::Enter) => { // start
                        *joy_state ^= JoypadButton::Start as u8;
                        info!("ENTER");
                    },
                    Some(Scancode::Escape) => { // start
                        info!("Esc");
                        break;
                    },
                    _ => {},
                }
            }
        }

        let start_time = pit::system_time();
        unsafe { gb_run_frame(gbs_ptr); }
        let diff = pit::system_time() - start_time;

        // 60 fps -> 1000 ms / 60 = ~17 ms per frame
        if diff < 17 {
            info!("{}", diff);
            pit::wait(17 - diff);
        }
    }

    RAM.get().unwrap().lock().clone().into_iter().for_each(|byte| {
         COM3.lock().write_byte(byte);
    });

}