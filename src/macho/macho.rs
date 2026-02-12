
use crate::{bytes_helper::read_uint32, bytes_helper::read_uint32_be, command_parser::command_type,bytes_helper::read_uint64 };
use crate::macho::load_commands::*;
pub trait macho_handler{
    fn process(&mut self, command:&command_type)->Result<(),String>;

    fn get_magic_number(&self)->u32;

    fn get_data(&self)->(& Vec<u8>,& Vec<load_command>){
        unimplemented!()
    }

    fn get_data_mut(&mut self)->(&mut Vec<u8>,&Vec<load_command>){
        unimplemented!()
    }

    fn write_back(&self)->Vec<u8>;
}



