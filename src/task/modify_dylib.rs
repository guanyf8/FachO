use crate::bytes_helper::{read_uleb128, read_sleb128};
use crate::macho::load_commands::load_command;
use crate::macho::{macho::macho_handler};


pub fn modify_dylib_ordinal(macho:&mut dyn macho_handler,old_ordinal: i32, new_ordinal: i32) {
    

    let magic_number=macho.get_magic_number();
    let (data, load_commands)=macho.get_data_mut();

    for load_command in load_commands{
        match load_command{
            load_command::LC_DYLIB_INFO_ONLY(lc_dyld_info_only)=>{
                for (i, size) in [(lc_dyld_info_only.bind_off, lc_dyld_info_only.bind_size),
                                (lc_dyld_info_only.weak_bind_off, lc_dyld_info_only.weak_bind_size),
                                (lc_dyld_info_only.lazy_bind_off, lc_dyld_info_only.lazy_bind_size)]{
                    let mut index=i as usize;
                    let end=index as usize +size as usize;
                    while index<end{
                        let byte=data[index];
                        let opcode=byte & 0xF0;
                        let immediate=(byte & 0x0F) as i32;
                        
                        if opcode == 0x00{ // # BIND_OPCODE_DONE
                            index+=1;
                        }else if opcode == 0x10{ //# BIND_OPCODE_SET_DYLIB_ORDINAL_IMM
                            if immediate ==old_ordinal{
                                data[index]=if new_ordinal<0 {0x30}else{0x10} | ((new_ordinal & 0x0F) as u8);
                            }
                            index+=1;
                        }else if opcode == 0x20{  //  # BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB
                            index+=1;
                            read_uleb128(data,&mut index);
                        }else if opcode == 0x30{  // # BIND_OPCODE_SET_DYLIB_SPECIAL_IMM
                            let trans_immediate = (byte | 0xf0) as i8;
                            if trans_immediate == old_ordinal as i8{
                                data[index]=if new_ordinal<0 {0x30}else{0x10} | ((new_ordinal & 0x0F) as u8);
                            }
                            index+=1;
                        }else if opcode == 0x40{    //# BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM
                            while data[index]!=0 && index<end{
                                index+=1;
                            }
                            index+=1;
                        }else if opcode == 0x50{     // # BIND_OPCODE_SET_TYPE_IMM
                            index+=1;
                        }else if opcode == 0x60{     //# BIND_OPCODE_SET_ADDEND_SLEB
                            index+=1;
                            read_sleb128(data,&mut index);
                        }else if opcode == 0x70{     //# BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB
                            index+=1;
                            read_uleb128(data,&mut index);
                        }else if opcode == 0x80{    //# BIND_OPCODE_ADD_ADDR_ULEB
                            index+=1;
                            read_uleb128(data,&mut index);
                        }else if opcode == 0x90{   // # BIND_OPCODE_DO_BIND
                            index+=1;
                        }else if opcode == 0xA0{   //# BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB
                            index+=1;
                            read_uleb128(data,&mut index);
                        }else if opcode == 0xB0{    //# BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED
                            index+=1;
                        }else if opcode == 0xC0{     //# BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB
                            index+=1;
                            read_uleb128(data,&mut index);
                            read_uleb128(data,&mut index);
                        }else{
                            panic!("Unknown bind opcode");
                        }

                    }    
                }
            },
            load_command::LC_SYMTAB(lc_symtab)=>{
                let sym_off= lc_symtab.symoff as usize;
                let sym_size= lc_symtab.nsyms as usize;

                let mut index:usize=sym_off as usize;
                let end=index+sym_size as usize*16;
                while index<end{
                    // let strx=read_uint32(data,index) as usize;
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

                    if desc!=0{
                        let ordinal:i8=data[index+7] as i8;
                        if ordinal==old_ordinal as i8{
                            data[index+7]=(new_ordinal&0xff) as u8;
                        }
                    }
                    index+=16;
                }
            }            
            _=>{
                
            }
        }   
    }
}
