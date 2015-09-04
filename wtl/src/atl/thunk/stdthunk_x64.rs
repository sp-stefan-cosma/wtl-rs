

#[cfg(target_arch = "x86_64")]
pub mod imp {
	use std;
	use winapi::{USHORT,ULONG64,c_void,DWORD_PTR,LPCVOID,SIZE_T};
	use kernel32;
	use std::fmt;

	//http://kelvinh.github.io/blog/2013/08/05/windows-x64-calling-conventions/
	#[repr(C,packed)] #[derive(Default,Debug)]
	pub struct Thunk {
		rcx_mov:USHORT,         // mov rcx, pThis
	    rcx_imm:ULONG64,         //
	    rax_mov:USHORT,         // mov rax, target
	    rax_imm:ULONG64,         //
	    rax_jmp:USHORT,         // jmp target
	}

	impl Thunk {
		pub fn print(&self){
			println!("print x64:{}",std::mem::size_of::<Thunk>());
		}

		pub fn init(&mut self,func:DWORD_PTR,p_this:*const c_void){
			self.rcx_mov = 0xb948;          // mov rcx, pThis
        	self.rcx_imm = p_this as ULONG64;  //
        	self.rax_mov = 0xb848;          // mov rax, target
        	self.rax_imm = func as ULONG64;   //
        	self.rax_jmp = 0xe0ff;          // jmp rax
        	unsafe{
        		let p = self as *const Thunk;
        		kernel32::FlushInstructionCache(kernel32::GetCurrentProcess(), p as LPCVOID, std::mem::size_of::<Thunk>() as SIZE_T);
        	}
        	//println!("{}", std::mem::size_of_val(self));
		}

		pub fn GetCodeAddress(&self)->*const Thunk {
			self as *const Self
		}
	}

	impl fmt::Display for Thunk {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// try!(writeln!(f, "thunk:{:p} ", self));
	  //       try!(writeln!(f, "move:0x{:x} ", self.rcx_mov));
	  //       try!(writeln!(f, "this:0x{:x} ", self.rcx_imm));
	  //       try!(writeln!(f, "move:0x{:x} ", self.rax_mov));
	  //       try!(writeln!(f, "target:0x{:x} ", self.rax_imm));
	  //       try!(writeln!(f, "jmp:0x{:x} ", self.rax_jmp));
	  		//disp as hex
	  		let bytes = std::mem::size_of::<Thunk>();
	  		let mut p = self as * const Thunk as *const u8;
	  		for i in 0..bytes{
	  			unsafe{
	  				
	  				try!(write!(f, "{:x} ", *p));
	  				p = p.offset(1);
	  			}
	  		}
	        Ok(())
	    }
	}
}


#[cfg(target_arch = "x86_64")]
mod tests{
	#[test]
	fn print(){
		let t = super::imp::Thunk::default();
		t.print();
		println!("{:?}", t);
	}
}
