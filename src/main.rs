mod task;
mod macho;
mod bytes_helper;
mod command_parser;

use macho::{macho64::macho64, fat_macho::fat_macho, macho::macho_handler};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let real_arg=&args[1..];
    let result=command_parser::command_parser::parse(real_arg);
    if let Err(e)=result{
        eprintln!("Error: {}",e);
        return;
    }
    let command=result.unwrap();
    let mut file_data = std::fs::read(&command.target_file).expect("Failed to read file");
    let magic_number: u32 = bytes_helper::read_uint32(&file_data, 0);
    let mut macho_handler: Box<dyn macho_handler>;
    if magic_number == 0xbebafeca {
        let fat_macho = fat_macho::parse(&file_data);
        macho_handler = Box::new(fat_macho);
    } else if magic_number == 0xFEEDFACF {
        let macho = macho64::parse(&file_data);
         macho_handler = Box::new(macho);
    } else {
        eprintln!("Unsupported file format");
        return;
    }
    macho_handler.process(&command.command).expect("Failed to process command");
    let final_data=macho_handler.write_back();
    std::fs::write(&command.target_file, final_data).expect("Failed to write file");

}
