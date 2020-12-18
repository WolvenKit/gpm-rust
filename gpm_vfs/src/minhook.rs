#![allow(dead_code)]

use minhook_sys::*;

pub fn initialize() -> MhStatus {
    unsafe {
        MH_Initialize()
    }
}

pub fn uninitialize() -> MhStatus {
    unsafe {
        MH_Uninitialize()
    }
}

pub fn create_hook(target: *const usize, detour: *const usize, original: *const usize) -> MhStatus { 
    unsafe {
        MH_CreateHook(target as _, detour as _, &(original as _))
    }
}

pub fn create_hook_api(module: &str, target_name: &str, detour: *const usize, original: *const usize) -> MhStatus {
    let mut module = module
        .encode_utf16()
        .collect::<Vec<_>>();
    module.push(0);
    
    let mut target_name = target_name
        .encode_utf16()
        .collect::<Vec<_>>();
    target_name.push(0);

    unsafe {
        MH_CreateHookApi(module.as_ptr(), target_name.as_ptr(), detour as _, &(original as _))
    }
}

pub fn create_hook_api_ex() -> ! {
    todo!();
}

pub fn remove_hook(target: *const usize) -> MhStatus {
    unsafe {
        MH_RemoveHook(target as _)
    }
}

pub fn enable_hook(target: *const usize) -> MhStatus {
    unsafe {
        MH_EnableHook(target as _)
    }
}

pub fn disable_hook(target: *const usize) -> MhStatus {
    unsafe {
        MH_DisableHook(target as _)
    }
}

pub fn queue_enable_hook(target: *const usize) -> MhStatus {
    unsafe {
        MH_QueueEnableHook(target as _)
    }
}

pub fn queue_disable_hook(target: *const usize) -> MhStatus {
    unsafe {
        MH_QueueDisableHook(target as _)
    }
}

pub fn apply_queued() -> MhStatus {
    unsafe {
        MH_ApplyQueued()
    }
}