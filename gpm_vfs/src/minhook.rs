#![allow(dead_code)]

use minhook_sys::*;

macro_rules! uns {
    ( $( $x:expr ),* ) => {
        {
            unsafe {
                $($x)*
            }
        }
    };
}

pub fn initialize() -> MhStatus {
    uns!(MH_Initialize())
}

pub fn uninitialize() -> MhStatus {
    uns!(MH_Uninitialize())
}

pub fn create_hook(target: *const usize, detour: *const usize, original: *const usize) -> MhStatus { 
    uns!(MH_CreateHook(target as _, detour as _, &(original as _)))
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

    uns!(MH_CreateHookApi(module.as_ptr(), target_name.as_ptr(), detour as _, &(original as _)))
}

pub fn create_hook_api_ex() -> ! {
    todo!();
}

pub fn remove_hook(target: *const usize) -> MhStatus {
    uns!(MH_RemoveHook(target as _))
}

pub fn enable_hook(target: *const usize) -> MhStatus {
    uns!(MH_EnableHook(target as _))
}

pub fn disable_hook(target: *const usize) -> MhStatus {
    uns!(MH_DisableHook(target as _))
}

pub fn queue_enable_hook(target: *const usize) -> MhStatus {
    uns!(MH_QueueEnableHook(target as _))
}

pub fn queue_disable_hook(target: *const usize) -> MhStatus {
    uns!(MH_QueueDisableHook(target as _))
}

pub fn apply_queued() -> MhStatus {
    uns!(MH_ApplyQueued())
}