
#![allow(non_snake_case,unused_variables,dead_code)]
#[cfg(target_arch = "x86")]
pub mod imp {
    use std;
    use winapi::{USHORT, ULONG64, c_void, DWORD_PTR, LPCVOID, SIZE_T};
    use kernel32;
    use std::fmt;
	//use kernel32;

    #[repr(C,packed)]
    #[derive(Default,Debug)]
    pub struct Thunk {
        m_mov: DWORD, // mov dword ptr [esp+0x4], pThis (esp+0x4 is hWnd)
        m_this: DWORD, // 
        m_jmp: BYTE, // jmp WndProc
        m_relproc: DWORD, // relative jmp
    }

	impl Thunk {
        pub fn print(&self) {
            println!("print x86:{}",std::mem::size_of::<Thunk>());
        }

        pub fn init(&mut self, func: DWORD_PTR, p_this: *const c_void) {
            self.m_mov = 0x042444C7;
            self.m_this = p_this as ULONG_PTR as DWORD;
            self.m_jmp = 0xe9;
            self.m_relproc = (func as INT_PTR - (self as *const _ as INT_PTR + std::mem::size_of::<Thunk>() as INT_PTR)) as DWORD;
            unsafe {
                let p = self as *const Thunk;
                kernel32::FlushInstructionCache(kernel32::GetCurrentProcess(),
                                                p as LPCVOID,
                                                std::mem::size_of::<Thunk>() as SIZE_T);
            }
            //println!("{}", std::mem::size_of_val(self));
        }

        pub fn GetCodeAddress(&self) -> *const Thunk {
            self as *const Self
        }
	}
}

#[cfg(target_arch = "x86")]
mod tests{
    use super::imp::*;
    #[test]
    fn print() {
        let t = Thunk::default();
        t.print();
        println!("{:?}", t);
    }
}
