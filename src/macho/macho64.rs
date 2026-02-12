use crate::bytes_helper::{read_uint32, read_uint32_be, read_uint64};
use crate::command_parser::command_type;
use crate::macho::{load_commands::*, macho::macho_handler};


pub struct macho64_header{
    pub magic:u32,
    pub cputype:u32,
    pub cpusubtype:u32,
    pub filetype:u32,
    pub ncmds:u32,
    pub sizeofcmds:u32,
    pub flags:u32,
    pub reserved:u32,
}


pub struct macho64{
    pub header:macho64_header,
    pub load_commands:Vec<load_command>,
    pub data:Vec<u8>,
}

impl macho64{
    pub fn parse(data:&[u8])->macho64{
        let header:macho64_header=macho64_header{
            magic:read_uint32(data,0),
            cputype:read_uint32(data,4),
            cpusubtype:read_uint32(data,8),
            filetype:read_uint32(data,12),
            ncmds:read_uint32(data,16),
            sizeofcmds:read_uint32(data,20),
            flags:read_uint32(data,24),
            reserved:read_uint32(data,28),
        };
        let mut load_commands:Vec<load_command>=Vec::new();
        let start:usize=0x20;
        let mut offset:usize=start;
        while offset<start+(header.sizeofcmds as usize) {
            let cmd:u32=read_uint32(data,offset);
            let cmdsize:u32=read_uint32(data,offset+4);
            match cmd{
                0xc=>{
                    load_commands.push(load_command::LC_LOAD_DYLIB(
                    lc_load_dylib{
                        cmd,
                        cmdsize,
                        str_offset:read_uint32(data,offset+8),
                        timestamp:read_uint32(data,offset+12),
                        current_version:read_uint32(data,offset+16),
                        compatibility_version:read_uint32(data,offset+20),
                        name:{
                            let name_offset=offset+(read_uint32(data,offset+8) as usize);
                            let mut end=name_offset;
                            while data[end]!=0{
                                end+=1;
                            }
                            String::from_utf8_lossy(&data[name_offset..end]).to_string()
                        },
                    }
                ));},
                0x80000018=>{
                    load_commands.push(load_command::LC_WEAK_DYLIB(
                    lc_weak_dylib{
                        cmd,
                        cmdsize,
                        str_offset:read_uint32(data,offset+8),
                        timestamp:read_uint32(data,offset+12),
                        current_version:read_uint32(data,offset+16),
                        compatibility_version:read_uint32(data,offset+20),
                        name:{
                            let name_offset=offset+(read_uint32(data,offset+8) as usize);
                            let mut end=name_offset;
                            while data[end]!=0{
                                end+=1;
                            }
                            String::from_utf8_lossy(&data[name_offset..end]).to_string()
                        },
                    }
                ));},
                0xd=>{
                    load_commands.push(load_command::LC_ID_DYLIB(
                    lc_id_dylib{
                        cmd,
                        cmdsize,
                        str_offset:read_uint32(data,offset+8),
                        timestamp:read_uint32(data,offset+12),
                        current_version:read_uint32(data,offset+16),
                        compatibility_version:read_uint32(data,offset+20),
                        name:{
                            let name_offset=offset+(read_uint32(data,offset+8) as usize);
                            let mut end=name_offset;
                            while data[end]!=0{
                                end+=1;
                            }
                            String::from_utf8_lossy(&data[name_offset..end]).to_string()
                        },
                    }
                ));},
                0x19=>{
                    load_commands.push(load_command::LC_SEGMENT_64(
                    lc_segment_64{
                        cmd,
                        cmdsize,
                        segname:data[offset+8..offset+0x18].try_into().unwrap(),
                        vmaddr:read_uint64(data,offset+0x18),
                        vmsize:read_uint64(data,offset+0x20),
                        fileoff:read_uint64(data,offset+0x28),
                        filesize:read_uint64(data,offset+0x30),
                        maxprot:read_uint32(data,offset+0x38),
                        initprot:read_uint32(data,offset+0x3c),
                        sections:{
                            let nsections=read_uint32(data,offset+0x40);
                            let mut sections_vec:Vec<sections>=Vec::new();
                            for i in 0..nsections{
                                let sect_offset=offset+0x48+(i as usize)*0x50;
                                sections_vec.push(sections{
                                    sectname:data[sect_offset..sect_offset+0x10].try_into().unwrap(),
                                    segname:data[sect_offset+0x10..sect_offset+0x20].try_into().unwrap(),
                                    addr:read_uint64(data,sect_offset+0x20),
                                    size:read_uint64(data,sect_offset+0x28),
                                    offset:read_uint32(data,sect_offset+0x30),
                                    align:read_uint32(data,sect_offset+0x34),
                                    reloff:read_uint32(data,sect_offset+0x38),
                                    nreloc:read_uint32(data,sect_offset+0x3c),
                                    flags:read_uint32(data,sect_offset+0x40),
                                    reserved1:read_uint32(data,sect_offset+0x44),
                                    reserved2:read_uint32(data,sect_offset+0x48),
                                    reserved3:read_uint32(data,sect_offset+0x4c),
                                });
                            }
                            sections_vec
                        },
                        flags:read_uint32(data,offset+0x44),
                    }
                ));},
                0x80000022=>{
                    load_commands.push(load_command::LC_DYLIB_INFO_ONLY(
                        lc_dylib_info_only{
                            cmd,
                            cmdsize,
                            rebase_off:read_uint32(data,offset+8),
                            rebase_size:read_uint32(data,offset+12),
                            bind_off:read_uint32(data,offset+16),
                            bind_size:read_uint32(data,offset+20),
                            weak_bind_off:read_uint32(data,offset+24),
                            weak_bind_size:read_uint32(data,offset+28),
                            lazy_bind_off:read_uint32(data,offset+32),
                            lazy_bind_size:read_uint32(data,offset+36),
                            export_off:read_uint32(data,offset+40),
                            export_size:read_uint32(data,offset+44),
                        }
                    ));},
                0x2=>{
                    load_commands.push(load_command::LC_SYMTAB(
                        lc_symtab{
                            cmd,
                            cmdsize,
                            symoff:read_uint32(data,offset+8),
                            nsyms:read_uint32(data,offset+12),
                            stroff:read_uint32(data,offset+16),
                            strsize:read_uint32(data,offset+20),
                        }
                    ));},
                _=>{
                    load_commands.push(load_command::OTHER_COMMAND(
                    other_command{
                        cmd,
                        cmdsize,
                    }
                ));},
            }
            offset+=cmdsize as usize;
        }
        return macho64 { header, load_commands, data: data.to_vec() };
    }
}

use crate::task::{modify_dylib, show_dylib};

impl macho_handler for macho64{
    fn get_data(&self)->(&Vec<u8>,&Vec<load_command>){
        (&self.data,&self.load_commands)
    }

    fn get_data_mut(&mut self)->(&mut Vec<u8>,&Vec<load_command>){
        (&mut self.data,&self.load_commands)
    }

    fn get_magic_number(&self)->u32 {
        self.header.magic
    }

    fn write_back(&self)->Vec<u8> {
        self.data.clone()
    }

    fn process(&mut self, command:&command_type)->Result<(),String>{
        match command{
            command_type::CHANGE_COMMAND(change_cmd)=>{
                modify_dylib::modify_dylib_ordinal(self, change_cmd.from.parse().unwrap(), change_cmd.to.parse().unwrap());
                Ok(())
            },
            command_type::ADD_COMMAND(add_cmd)=>{
                
                Ok(())
            },
            command_type::SHOW_COMMAND(show_cmd)=>{
                show_dylib::show_dylib_symbols(self, &show_cmd.show);
                Ok(())
            },
            command_type::DELETE_COMMAND(delete_cmd)=>{
                // Implement the logic to delete a specific load command
                Ok(())
            },
            command_type::LIST_COMMAND=>{
                show_dylib::list_load_dylibs(self);
                Ok(())
            },
            _=>{
                Err("Unsupported command".to_string())
            },
        }
    }
}