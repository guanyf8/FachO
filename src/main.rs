use std::result;



mod modify_dylib;
mod macho;
mod buffer_helper;
mod command_parser;

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
        },
        command_parser::command_type::ADD_COMMAND(add_cmd)=>{
            println!("Add command: {}",add_cmd.add);
        },
        command_parser::command_type::DELETE_COMMAND(delete_cmd)=>{
            println!("Delete command: {}",delete_cmd.delete);
        },
        command_parser::command_type::LIST_COMMAND=>{
            let file= command.target_file;
            let file_data = std::fs::read(&file).expect("Failed to read file");
            let macho_instance = macho::macho64::parse(&file_data);
            let mut iter=macho_instance.load_commands.iter();
            while let Some(cmd) = iter.next() {
                match cmd {
                    macho::load_command::LC_LOAD_DYLIB(dylib_cmd) => {
                        println!("Found LC_LOAD_DYLIB: {}", dylib_cmd.name);
                    }
                    _ => {}
                }
            }
        },
        _=>{
            eprintln!("Usage: [-change symbols] -from <old_ordinal> to <new_ordinal>");
            return;
        },
    }

    let file = args[1].clone();
    let old_ordinal: i32 = args[2].parse().expect("Invalid old ordinal");
    let new_ordinal: i32 = args[3].parse().expect("Invalid new ordinal");
    let mut fat_data = std::fs::read(&file).expect("Failed to read file");
    modify_dylib::modify_dylib(&mut fat_data, old_ordinal, new_ordinal);
    std::fs::write(&file, &fat_data).expect("Failed to write file");

    println!("Parsed macho64: ncmds = {}", macho_instance.header.ncmds);
}
