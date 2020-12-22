use minhook;
use vfs_hook;

use std::sync::Arc;

impl CreateFileWHook for VfsHook {
    use winapi::um::fileapi::CreateFileW;
    use winapi::um::winnt::{ LPWSTR, HANDLE };
    use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
    use winapi::shared::minwindef::DWORD;

    type CreateFileWType = unsafe extern "system" fn(
        LPCWSTR,
        DWORD,
        DWORD,
        LPSECURITY_ATTRIBUTES,
        DWORD,
        DWORD,
        HANDLE
    ) -> HANDLE;

    static mut CreateFileWCallback: Option<Rc<CreateFileWType>> = None;

    fn enable(&self) {
        let result = minhook::initialize();
    }

    fn disable(&self) {

    }
}