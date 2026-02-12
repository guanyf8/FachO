use crate::bytes_helper::{read_uint32, read_uint32_be, read_uleb128, read_sleb128};
use crate::macho::load_commands::lc_weak_dylib;
use crate::macho::{self, macho64::macho64,load_commands,macho::macho_handler};


pub fn show_dylib_symbols(macho:&dyn macho_handler, symbol_to_find: &str)->Option<i8> {
    // let data=&mut fat_data[fat_offset..fat_offset+size];
    
    let magic = macho.get_magic_number();
    // if magic != 0xFEEDFACF {
    //     panic!("Not a valid Mach-O 64-bit binary");
    // }

    let (data,load_commands)=macho.get_data();

     for load_command in load_commands{
         match load_command{
            load_commands::load_command::LC_SYMTAB(lc_symtab)=>{
                let sym_off= lc_symtab.symoff;
                let sym_size= lc_symtab.nsyms;
                let str_off= lc_symtab.stroff;
                let str_size= lc_symtab.strsize;

                let mut index:usize=sym_off as usize;
                let end=index+sym_size as usize*16;
                while index<end{
                    let strx=read_uint32(data,index) as usize;
                    let cstr = std::ffi::CStr::from_bytes_until_nul(&data[str_off as usize + strx..]);
                    let name=cstr.unwrap().to_str().unwrap();
                    // let type_byte=data[index+4];
                    // let sect=data[index+5];
                    let desc:u16=data[index+6] as u16 | ((data[index+7] as u16)<<8);
                    // let value:u64=data[index+8] as u64
                        // | ((data[index+9] as u64)<<8)
                        // | ((data[index+10] as u64)<<16)
                        // | ((data[index+11] as u64)<<24)
                        // | ((data[index+12] as u64)<<32)
                        // | ((data[index+13] as u64)<<40)
                        // | ((data[index+14] as u64)<<48)
                        // | ((data[index+15] as u64)<<56);
                    if name==symbol_to_find{
                        if desc==0{
                            println!("Symbol '{}' is not dynamically linked", symbol_to_find);
                            return None;
                        }
                        else{
                            let ordinal=data[index+7] as i8;
                            println!("{}",ordinal as i32);
                            return Some(ordinal);
                        }
                    }
                    index+=16;
                }
            }
            _=>{

            }

         }
     }
    return None;
}


pub fn list_load_dylibs(macho:&dyn macho_handler){

    let (_,load_commands)=macho.get_data();
    let mut i=0;
    for load_command in load_commands{
        match load_command{
            load_commands::load_command::LC_LOAD_DYLIB(lc_dylib)=>{
                println!("{}: {}", i, lc_dylib.name);
                i+=1;
            }
            load_commands::load_command::LC_ID_DYLIB(lc_id_dylib)=>{
                println!("{}: {}", i, lc_id_dylib.name);
                i+=1;
            }
            load_commands::load_command::LC_WEAK_DYLIB(lc_weak_dylib)=>{
                println!("{}: {}", i, lc_weak_dylib.name);
                i+=1;
            }
            _=>{

            }

         }
     }
}
