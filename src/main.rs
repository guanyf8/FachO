use std::result;



mod modify_dylib;
mod macho;
mod buffer_helper;
mod command_parser;
mod show_dylib;

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
        command_parser::command_type::SHOW_COMMAND(show_cmd)=>{
            let symbol_to_find = show_cmd.show;
            let file= command.target_file;
            let mut file_data = std::fs::read(&file).expect("Failed to read file");
            match show_dylib::show_dylib_symbols(&mut file_data, symbol_to_find){
                Some(ordinal)=>{
                    println!("{}", ordinal);
                },
                None=>{
                }
            }
        },
        command_parser::command_type::DELETE_COMMAND(delete_cmd)=>{
            println!("Not Yet Support");
        },
        command_parser::command_type::LIST_COMMAND=>{
            let file= command.target_file;
            let file_data = std::fs::read(&file).expect("Failed to read file");
            show_dylib::list_load_dylibs(&file_data);
        },
        _=>{
            eprintln!("Usage: -change <old_ordinal> to <new_ordinal>");
            return;
        },
    }
    

}
