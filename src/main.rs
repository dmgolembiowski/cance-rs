/// cance-rs
///
/// An unpredictable, self-modifying executable.
///
/// Possible actions may result in loss of network,
/// power cycling, spurrious web requests,
/// random logoffs, and disk modification.
///
/// Adapted from https://github.com/jam1garner/rust-dyn-call/blob/master/src/main.rs
//extern crate system_shutdown;

use system_shutdown::{logout, shutdown};

use rand::thread_rng;
use rand::Rng;
use goblin::Object;
use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;
use std::ffi::OsStr;

// The objective of this program is to modify `pt_mutation`
// during every execution. Its initial value guarantees at least
// one "effect". Up to 5 sequential events can happen.

fn get_sym(name: &str) -> *const () { 
    (((red_flag as usize) - get_sym_offset("red_flag")) + get_sym_offset(name)) as *const ()
}

#[no_mangle] pub fn red_flag() {}

#[no_mangle]
pub extern "Rust" fn pt_mutation() -> usize {
    2 as usize 
}
   

fn get_sym_offset(name: &str) -> usize {
    
    let executable_file = ::std::env::current_exe()
                                .expect("_")
                                .canonicalize()
                                .expect("_")
                                .to_owned();

    let path         = Path::new(&executable_file);
    let executable   =   fs::read(path).unwrap();

    match Object::parse(&executable).unwrap() {
        Object::Elf(elf) => {
            
            let sym = elf.dynsyms
                .iter()
                .find(|sym| {
                    elf.dynstrtab
                        .get(sym.st_name)
                        .unwrap()
                        .unwrap() == name});

            let sym = match sym {
                Some(sym) => sym,
                None => panic!("Symbol '{}' not found. Be sure you're using #[no_mangle].", name)
            };

            sym.st_value as usize
        }
        _ => { panic!("NotImplemented"); }
    }
}


fn main() {
    
    // Always ensure that OP-CODE #0 (generate next ELF is performed)
    let persist_idx = 0 as usize;
    
        
    // `ops` holds the sequence of distinct operations to dispatch
    let mut ops = {
        
        let mut deq = ::std::collections::VecDeque::new();
        deq.push_back(pt_mutation());
        
        let mut rng = thread_rng();

        (0..pt_mutation()).map(|_| {
            deq.push_back(rng.gen::<usize>() % (6 as usize))
        });
            
        deq
    };

    ops.into_iter()
        .map(|slot| dispatch(slot));
    dispatch(0 as usize);
}

fn dispatch(op_code: usize) -> () {
    match op_code {
        0 => {
            let abs_path = ::std::env::current_exe()
                                            .expect("_")
                                            .canonicalize()
                                            .expect("_")
                                            .to_owned();

            let path     = Path::new(&abs_path);
            let buffer   =   fs::read(path).unwrap();

            if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
                     
                println!("Pretending to delete this file...");
                //let _res = ::std::fs::remove_file(&abs_path);
                
                println!("Preparing to overwrite `pt_mutation` with a new value.");
                //let file = File::create(...);

                println!("Changing the executable permission bits.");
            };
        },
        1 => {
            match shutdown() {
                Ok(_) => println!("Shutting down ~UwU~ "),
                Err(error) => eprintln!("Failed to shut down: {}", error),
            }
        },
        2 => {
            println!("");
        },
        3 => {},
        4 => {},
        5 => {
            match logout() {
                Ok(_) => println!("Logging out ..."),
                Err(error) => eprintln!("Failed to log out: {}", error),
            }
        },
        _ => { }
    }
}




