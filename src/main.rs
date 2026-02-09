use std::result;



mod modify_dylib;
mod macho;
mod buffer_helper;
mod command_parser;

fn modify_ordinal(fat_data: &mut [u8], old_ordinal: i32, new_ordinal: i32)  {
    // Placeholder implementation
    modify_dylib::modify_fat_ordinal(fat_data, old_ordinal, new_ordinal);
}

fn change(file: &str,symbols:Option<String>,from:&str,to:&str){
    let old_ordinal: i32 = from.parse().expect("Invalid old ordinal");
    let new_ordinal: i32 = to.parse().expect("Invalid new ordinal");
    let mut fat_data = std::fs::read(&file).expect("Failed to read file");
    
    std::fs::write(&file, &fat_data).expect("Failed to write file");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let real_arg:Vec<String>=args[1..].to_vec();
    let result=command_parser::command_parser::parse(&real_arg);
    if let Err(e)=result{
        eprintln!("Error: {}",e);
        return;
    }
    let command=result.unwrap();
    match command.command {
        command_parser::command_type::CHANGE_COMMAND(change_cmd)=>{
            println!("Change command: from {} to {}",change_cmd.from,change_cmd.to);
            let file= command.target_file;
            let old_ordinal: i32 = change_cmd.from.parse().expect("Invalid old ordinal");
            let new_ordinal: i32 = change_cmd.to.parse().expect("Invalid new ordinal");
            let mut file_data = std::fs::read(&file).expect("Failed to read file");
            modify_dylib::modify_fat_ordinal(&mut file_data, old_ordinal, new_ordinal);
            std::fs::write(&file, &file_data).expect("Failed to write file");
  
        },
        command_parser::command_type::ADD_COMMAND(add_cmd)=>{
            println!("Not Yet Support");
        },
        command_parser::command_type::DELETE_COMMAND(delete_cmd)=>{
            println!("Not Yet Support");
        },
        command_parser::command_type::LIST_COMMAND=>{
            let file= command.target_file;
            let file_data = std::fs::read(&file).expect("Failed to read file");
            let macho_instance = macho::macho64::parse(&file_data);
            let mut iter=macho_instance.load_commands.iter();
            let mut i=1;
            while let Some(cmd) = iter.next() {
                
                match cmd {
                    macho::load_command::LC_LOAD_DYLIB(dylib_cmd) => {
                        println!("Found LC_LOAD_DYLIB: {}, ordinal = {}", dylib_cmd.name,i);
                        i+=1;
                    }
                    _ => {}
                }
            }
            println!("Parsed macho64: ncmds = {}", macho_instance.header.ncmds);
        },
        _=>{
            eprintln!("Usage: [-change symbols] -from <old_ordinal> to <new_ordinal>");
            return;
        },
    }

}
