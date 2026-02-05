

pub struct change_command<'a>{
    pub change:Option<&'a str>,
    pub from:&'a str,
    pub to:&'a str,
}

pub struct add<'a>{
    pub add:&'a str,
}

pub struct delete<'a>{
    pub delete:&'a str,
}

pub enum command_type<'a>{
    CHANGE_COMMAND(change_command<'a>),
    ADD_COMMAND(add<'a>),
    DELETE_COMMAND(delete<'a>),
    LIST_COMMAND,
}

pub struct command_parser<'a>{
    pub command:command_type<'a>,
    pub target_file:&'a str,
}

const HELP_MESSAGE: &str = r#"Usage: facho [OPTIONS] <COMMAND> <TARGET>

Commands:
  -list
      List all load commands in the Mach-O binary.

  -from <OLD> -to <NEW> <TARGET>
      Replace a dylib path or library ordinal.
        - If <OLD> is a path (e.g., @rpath/libfoo.dylib), replace it with <NEW>.
        - If <OLD> is an ordinal (e.g., "2"), change references from that library to <NEW>.
        - Special ordinals: 
            "-1" → main executable,
            "-2" → dynamic lookup (flat namespace).

  -change <SYMBOL_SPEC> -from <OLD> -to <new> <TARGET>
      Change symbol binding for specific symbols from a given dylib.
        - <SYMBOL_SPEC> can be:
            • A literal symbol name (e.g., "_lua_newstate")
            • A regex pattern (e.g., "_luaL?_.*")
            • A file path containing symbol names (e.g., "symbols.txt")

  -add <DYLIB_PATH> <TARGET>
      Insert a new dylib at the beginning of the load command list.

  -delete <DYLIB_PATH> <TARGET>
      Remove the first occurrence of the specified dylib and exit.

Arguments:
  <TARGET>    Path to the Mach-O binary (e.g., my/app/xxx.framework/xxx)

Examples:
  facho -list MyApp
  facho -from @rpath/libA.dylib -to @rpath/libB.dylib MyApp
  facho -from "2" -to "3" MyApp
  facho -change "_luaL?_.*" -from @rpath/libLua.dylib -to "-2" MyApp
  facho -add @rpath/new.dylib MyApp
  facho -delete @rpath/old.dylib MyApp
"#;

impl<'a> command_parser<'a> {
    pub fn parse(args: &'a Vec<String>)->Result<command_parser<'a>,String>{
        let mut iter= args.iter();
        
        let mut change_arg:Option<&'a str>=None;
        let mut from_arg:Option<&'a String>=None;
        let mut to_arg:Option<&'a String>=None; 

        while let Some(_arg)=iter.next(){
            print!("Parsing arg: {}\n",_arg);
            match _arg.as_str(){
                "-change"=>{
                    change_arg=Option::Some(iter.next().unwrap().as_str());
                },
                "-add"=>{
                    return Ok(command_parser{
                        command:command_type::ADD_COMMAND(
                            add{
                                add:iter.next().unwrap(),
                            }
                        ),
                        target_file:iter.next().unwrap(),
                    });
                },
                "-delete"=>{
                    return Ok(command_parser{
                        command:command_type::DELETE_COMMAND(
                            delete{
                                delete:iter.next().unwrap(),
                            }
                        ),
                        target_file:iter.next().unwrap(),
                    });
                },
                "-from"=>{
                    from_arg=iter.next();
                },
                "-to"=>{
                    to_arg=iter.next();
                },
                "-list"=>{
                    return Ok(command_parser{
                        command:command_type::LIST_COMMAND,
                        target_file:iter.next().unwrap(),
                    });
                },
                _=>{
                    if(from_arg.is_none() || to_arg.is_none()){
                        return Err(HELP_MESSAGE.to_string());
                    }
                    return Ok(command_parser{
                        command:command_type::CHANGE_COMMAND(change_command 
                            {   change: change_arg, 
                                from: from_arg.unwrap(), 
                                to: to_arg.unwrap() }),
                        target_file:_arg,
                    });
                },
            }
        }
        return Err(HELP_MESSAGE.to_string());
    }
}