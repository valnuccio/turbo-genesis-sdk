#[cfg(not(target_family = "wasm"))]
pub fn play_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn play_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/audio")]
        extern "C" {
            fn play_sound(key_ptr: *const u8, key_len: u32) -> u32;
        }
        play_sound(key_ptr, key_len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/audio")]
        extern "C" {
            fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32;
        }
        pause_sound(key_ptr, key_len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/audio")]
        extern "C" {
            fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32;
        }
        stop_sound(key_ptr, key_len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/audio")]
        extern "C" {
            fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32;
        }
        is_sound_playing(key_ptr, key_len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn set_panning(_key_ptr: *const u8, _key_len: u32, _panning: f32) {}

#[cfg(target_family = "wasm")]
pub fn set_panning(key_ptr: *const u8, key_len: u32, panning: f32) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/audio")]
        extern "C" {
            fn set_panning(key_ptr: *const u8, key_len: u32, panning: f32);
        }
        set_panning(key_ptr, key_len, panning);
    }
}
