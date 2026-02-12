use crate::macho::{macho::macho_handler, load_commands::*};
use crate::bytes_helper::{read_uint32};
use crate::command_parser::command_type;


pub struct macho32_header{
    pub magic:u32,
    pub cputype:u32,
    pub cpusubtype:u32,
    pub filetype:u32,
    pub ncmds:u32,
    pub sizeofcmds:u32,
    pub flags:u32,
}

pub struct macho32{
    pub header:macho32_header,
    pub load_commands:Vec<load_command>,
    pub data:Vec<u8>,
}

impl macho32{
    pub fn parse(data:&[u8])->macho32{
        let header:macho32_header=macho32_header{
            magic:read_uint32(data,0),
            cputype:read_uint32(data,4),
            cpusubtype:read_uint32(data,8),
            filetype:read_uint32(data,12),
            ncmds:read_uint32(data,16),
            sizeofcmds:read_uint32(data,20),
            flags:read_uint32(data,24),
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
                    ));
                },
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
                    ));
                },
                0x1d=>{
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
                    ));
                },
                0x19=>{
                    load_commands.push(load_command::LC_SEGMENT(
                        lc_segment{
                            cmd,
                            cmdsize,
                            segname:data[offset+8..offset+0x18].try_into().unwrap(),
                            vmaddr:read_uint32(data,offset+0x18),
                            vmsize:read_uint32(data,offset+0x1c),
                            fileoff:read_uint32(data,offset+0x20),
                            filesize:read_uint32(data,offset+0x24),
                            maxprot:read_uint32(data,offset+0x28),
                            initprot:read_uint32(data,offset+0x2c),
                            sections:{
                                let nsections=read_uint32(data,offset+0x30);
                                let mut sections_vec:Vec<sections32>=Vec::new();
                                for i in 0..nsections{
                                    let sect_offset=offset+0x48+(i as usize)*0x38;
                                    sections_vec.push(sections32{
                                        sectname:data[sect_offset..sect_offset+0x10].try_into().unwrap(),
                                        segname:data[sect_offset+0x10..sect_offset+0x20].try_into().unwrap(),
                                        addr:read_uint32(data,sect_offset+0x20),
                                        size:read_uint32(data,sect_offset+0x24),
                                        offset:read_uint32(data,sect_offset+0x28),
                                        align:read_uint32(data,sect_offset+0x2c),
                                        reloff:read_uint32(data,sect_offset+0x30),
                                        nreloc:read_uint32(data,sect_offset+0x34),
                                        flags:read_uint32(data,sect_offset+0x38),
                                        reserved1:read_uint32(data,sect_offset+0x3c),
                                        reserved2:read_uint32(data,sect_offset+0x40),
                                    });
                                }
                                sections_vec
                            },
                            flags:read_uint32(data,offset+0x44),
                        }
                    ));
                },
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
                    ));
                },
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
                    ));
                },
                _=>{
                    load_commands.push(load_command::OTHER_COMMAND(
                        other_command{
                            cmd,
                            cmdsize,
                        }
                    ));
                },
            }
            offset+=cmdsize as usize;
        }
        return macho32 { header, load_commands, data: data.to_vec() };
    }
}