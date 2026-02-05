
use crate::buffer_helper::read_uint32;
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

pub struct lc_load_dylib<'a>{
    pub cmd:u32,
    pub cmdsize:u32,
    pub str_offset:u32,
    pub timestamp:u32,
    pub current_version:u32,
    pub compatibility_version:u32,
    pub name:&'a str
}

pub struct other_command{
    pub cmd:u32,
    pub cmdsize:u32,
    // other fields as needed
}

pub enum load_command<'a>{
    LC_LOAD_DYLIB(lc_load_dylib<'a>),
    OTHER_COMMAND(other_command),
}

pub struct macho64<'a>{
    pub header:macho64_header,
    pub load_commands:Vec<load_command<'a>>,
}

impl<'a> macho64<'a>{
    pub fn parse(data:&'a[u8])->macho64<'a>{
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
            if cmd==0xc{
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
                            std::str::from_utf8(&data[name_offset..end]).unwrap()
                        },
                    }
                ));
            }else{
                load_commands.push(load_command::OTHER_COMMAND(
                    other_command{
                        cmd,
                        cmdsize,
                    }
                ));
            }
            offset+=cmdsize as usize;
        }
        return macho64 { header, load_commands };
    }
}




