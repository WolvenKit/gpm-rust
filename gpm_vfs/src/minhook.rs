#![allow(dead_code)]

use minhook_sys::*;

#[derive(thiserror::Error, Debug)]
pub enum MinhookError {
    #[error("an unknown Minhook error occured {0}")]
    MinhookUnknownError(i32),
    #[error("error {0}: Minhook has already been initialized")]
    MinhookAlreadyInitError(i32),
    #[error("error {0}: Minhook has not been initialized, or has been unitialized")]
    MinhookNotInitError(i32),
    #[error("error {0}: hook at target function {1} has not been created yet")]
    HookNotCreateadError(i32, usize),
    #[error("error {0}: hook at target function {1} has already been created")]
    HookAlreadyCreatedError(i32, usize),
    #[error("error {0}: hook at target function {1} has already been enabled")]
    HookEnabledError(i32, usize),
    #[error("error {0}: hook at target function {1} is not enabled yet, or has been disabled")]
    HookDisabledError(i32, usize),
    #[error("error {0}: target function {1} points to a non-executable or unallocated region of memory")]
    HookNotExecutableError(i32, usize),
    #[error("error {0}: target function {1} cannot be hooked")]
    HookUnsupportedError(i32, usize),
    #[error("error {0}: failed to allocated memory while hooking target function {1}")]
    HookAllocFailError(i32, usize),
    #[error("error {0}: failed to change memory protection while hooking target function {1}")]
    HookMemoryProtectError(i32, usize),
    #[error("error {0}: failed to find target module with name {1}")]
    ModuleNotFoundError(i32, String),
    #[error("error {0}: failed to find target function with name {1} in module {2}")]
    FunctionNotFoundError(i32, String, String)
}

pub fn initialize() -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_Initialize()
    }, None, None, None)
}

pub fn uninitialize() -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_Uninitialize()
    }, None, None, None)
}

pub fn create_hook(target: *const usize, detour: *const usize, original: *const usize) -> Result<(), MinhookError> { 
    into_err(unsafe {
        MH_CreateHook(target as _, detour as _, &(original as _))
    }, Some(target as _), None, None)
}

pub fn create_hook_api(module: &str, target_name: &str, detour: *const usize, original: *const usize) -> Result<(), MinhookError> {
    let mut module_vec = module
        .encode_utf16()
        .collect::<Vec<_>>();
    module_vec.push(0);
    
    let mut target_name_vec = target_name
        .encode_utf16()
        .collect::<Vec<_>>();
    target_name_vec.push(0);

    into_err(unsafe {
        MH_CreateHookApi(module_vec.as_ptr(), target_name_vec.as_ptr(), detour as _, &(original as _))
    }, None, Some(module), Some(target_name))
}

pub fn create_hook_api_ex() -> ! {
    todo!();
}

pub fn remove_hook(target: *const usize) -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_RemoveHook(target as _)
    }, Some(target as _), None, None)
}

pub fn enable_hook(target: *const usize) -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_EnableHook(target as _)
    }, Some(target as _), None, None)
}

pub fn disable_hook(target: *const usize) -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_DisableHook(target as _)
    }, Some(target as _), None, None)
}

pub fn queue_enable_hook(target: *const usize) -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_QueueEnableHook(target as _)
    }, Some(target as _), None, None)
}

pub fn queue_disable_hook(target: *const usize) -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_QueueDisableHook(target as _)
    }, Some(target as _), None, None)
}

pub fn apply_queued() -> Result<(), MinhookError> {
    into_err(unsafe {
        MH_ApplyQueued()
    }, None, None, None)
}

fn into_err(code: MhStatus, func_addr: Option<usize>, mod_name: Option<&str>, func_name: Option<&str>) -> Result<(), MinhookError> {
    let func_addr = func_addr.unwrap_or(0);
    let mod_name = mod_name.unwrap_or("?");
    let func_name = func_name.unwrap_or("?");

    match code {
        MhStatus::Unknown => Err(MinhookError::MinhookUnknownError(code as _)),
        MhStatus::Ok => Ok(()),
        MhStatus::ErrorAlreadyInitialized => Err(MinhookError::MinhookAlreadyInitError(code as _)),
        MhStatus::ErrorNotInitialized => Err(MinhookError::MinhookNotInitError(code as _)),
        MhStatus::ErrorAlreadyCreated => Err(MinhookError::HookAlreadyCreatedError(code as _, func_addr)),
        MhStatus::ErrorNotCreated => Err(MinhookError::HookNotCreateadError(code as _, func_addr)),
        MhStatus::ErrorEnabled => Err(MinhookError::HookEnabledError(code as _, func_addr)),
        MhStatus::ErrorDisabled => Err(MinhookError::HookDisabledError(code as _, func_addr)),
        MhStatus::ErrorNotExecutable => Err(MinhookError::HookNotExecutableError(code as _, func_addr)),
        MhStatus::ErrorUnsupportedFunction => Err(MinhookError::HookUnsupportedError(code as _, func_addr)),
        MhStatus::ErrorMemoryAlloc => Err(MinhookError::HookAllocFailError(code as _, func_addr)),
        MhStatus::ErrorMemoryProtect => Err(MinhookError::HookMemoryProtectError(code as _, func_addr)),
        MhStatus::ErrorModuleNotFound => Err(MinhookError::ModuleNotFoundError(code as _, mod_name.to_owned())),
        MhStatus::ErrorFunctionNotFound => Err(MinhookError::FunctionNotFoundError(code as _, func_name.to_owned(), mod_name.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use crate::minhook;

    #[test]
    fn test_uninit_err() {
        minhook::uninitialize().unwrap();
    }
}