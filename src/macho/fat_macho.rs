use crate::macho::{macho64::macho64, macho::macho_handler,macho32::macho32};
use crate::bytes_helper::{read_uint32, read_uint32_be};
use crate::command_parser::command_type;

pub struct arch{
    pub cputype:u32,
    pub cpusubtype:u32,
    pub offset:u32,
    pub size:u32,
    pub align:u32,
    pub macho:Box<dyn macho_handler>,
}

pub struct fat_macho{   
    pub magic:u32,
    pub fat_archs:Vec<arch>,
}

impl fat_macho{
    pub fn parse(data:&[u8])->fat_macho{
        let magic:u32=read_uint32(data,0);
        let nfat_arch:u32=read_uint32(data,4);
        let mut fat_archs:Vec<arch> = Vec::new();
        for i in 0..nfat_arch{
            let offset:usize=8+(i as usize)*20;
            let cputype:u32=read_uint32(data,offset);
            let cpusubtype:u32=read_uint32(data,offset+4);
            let arch_offset: u32 = read_uint32_be(data, offset + 8);
            let arch_size: u32 = read_uint32_be(data, offset + 12);
            let align:u32=read_uint32(data,offset+16);
            let macho_data=&data[arch_offset as usize..(arch_offset+arch_size) as usize];
            let macho=macho64::parse(macho_data);
            fat_archs.push(arch{
                cputype,
                cpusubtype,
                offset:arch_offset,
                size:arch_size,
                align,
                macho:Box::new(macho),
            });
        }
        return fat_macho { magic, fat_archs};
    }
}

impl macho_handler for fat_macho{
    fn get_magic_number(&self)->u32 {
        self.magic
    }

    fn process(&mut self, command:&command_type)->Result<(),String>{
        for fat_arch in &mut self.fat_archs{
            fat_arch.macho.process(command)?;
        }
        Ok(())
    }

    fn write_back(&self)->Vec<u8> {
        let mut data:Vec<u8> = Vec::new();
        for fat_arch in &self.fat_archs{
            let (macho_data,_)=fat_arch.macho.get_data();
            data.extend_from_slice(&macho_data);
        }
        data
    }
}