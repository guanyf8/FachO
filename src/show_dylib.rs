use crate::buffer_helper::{read_uint32, read_uint32_be, read_uleb128, read_sleb128};
use crate::macho::{self, macho64};

pub fn show_dylib_symbols(data: &mut [u8], symbol_to_find: &str)->Option<i8> {
    // let data=&mut fat_data[fat_offset..fat_offset+size];
    
    let magic = read_uint32(data, 0);
    if magic != 0xFEEDFACF {
        panic!("Not a valid Mach-O 64-bit binary");
    }

    let ncmds = read_uint32(data, 16);
    const LOAD_COMMAND_START: usize = 32;

    let mut offset: usize = LOAD_COMMAND_START;

    for _ in 0..ncmds {
        let cmd=read_uint32(data,offset);
        let cmdsize:usize=read_uint32(data,offset+4) as usize;

        if cmd==0x2{
            let sym_off= read_uint32(data,offset+8);
            let sym_size= read_uint32(data,offset+12);
            let str_off= read_uint32(data,offset+16);
            let str_size= read_uint32(data,offset+20);

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
                        return Some(ordinal);
                    }
                }
                index+=16;
            }
        }
        offset+=cmdsize;
    }
    return None;
}


pub fn show_fat_dylib_symbols(fat_data: &mut [u8], symbol_to_find: &str) {
    let magic = read_uint32(&fat_data, 0);
    if magic == 0xbebafeca {
        let nfat_arch = read_uint32_be(&fat_data, 4);
        let mut offset=8;
        for _i in 0..nfat_arch {
            let arch_offset: usize = read_uint32_be(&fat_data, offset + 8) as usize;
            let arch_size: usize = read_uint32_be(&fat_data, offset + 12) as usize;
            show_dylib_symbols(&mut fat_data[arch_offset..arch_offset+arch_size], symbol_to_find);
            offset += 20;
        }
    }else if magic == 0xFEEDFACF {
        let size = fat_data.len() as usize;
        show_dylib_symbols(&mut fat_data[0..size], symbol_to_find);
    }
}

//暂时不支持fat
pub fn list_load_dylibs(data:&[u8]){
    let macho_instance = macho::macho64::parse(data);
        let mut i=1;
        macho_instance.load_commands.iter().for_each(|cmd|{
            match cmd {
                macho::load_command::LC_LOAD_DYLIB(dylib_cmd) => {
                    println!("{}, {}", i,dylib_cmd.name);
                    i+=1;
                }
                _ => {}
            }
        });
}
