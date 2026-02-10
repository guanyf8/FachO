

pub struct change_command<'a>{
    pub from:&'a str,
    pub to:&'a str,
}

pub struct add<'a>{
    pub add:&'a str,
}

pub struct delete<'a>{
    pub delete:&'a str,
}

pub struct show<'a>{
    pub show:&'a str,
}

pub enum command_type<'a>{
    CHANGE_COMMAND(change_command<'a>),
    ADD_COMMAND(add<'a>),
    DELETE_COMMAND(delete<'a>),
    SHOW_COMMAND(show<'a>),
    LIST_COMMAND,
}

pub struct command_parser<'a>{
    pub command:command_type<'a>,
    pub target_file:&'a str,
}

const HELP_MESSAGE: &str = r#"Usage: facho <COMMAND> <TARGET>

Commands:
  -list
      List all load dylib commands and their ordinals in the Mach-O binary.

  -show <SYMBOL>
      Show the library ordinal for a specific symbol.

  -change <OLD> -to <NEW> <TARGET>
      Replace a library ordinal.
        - Special ordinals: 
            "-1" → main executable,
            "-2" → dynamic lookup (flat namespace).

Arguments:
  <TARGET>    Path to the Mach-O binary (e.g., my/app/xxx.framework/xxx)

Examples:
  facho -list MyApp
  facho -change "2" -to "3" MyApp
  facho -show _luaL_loadfile MyApp
"#;

impl<'a> command_parser<'a> {
    pub fn parse(args: &'a Vec<String>)->Result<command_parser<'a>,String>{
        let mut iter= args.iter();
        
        let mut change_arg:Option<&'a str>=None;
        let mut from_arg:Option<&'a String>=None;
        let mut to_arg:Option<&'a String>=None; 

        while let Some(_arg)=iter.next(){
            match _arg.as_str(){
                "-for"=>{
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
                "-change"=>{
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
                "-show"=>{
                    return Ok(command_parser{
                        command:command_type::SHOW_COMMAND(
                            show{
                                show:iter.next().unwrap(),
                            }
                        ),
                        target_file:iter.next().unwrap(),
                    });
                },
                _=>{
                    if(from_arg.is_none() || to_arg.is_none()){
                        return Err(HELP_MESSAGE.to_string());
                    }
                    return Ok(command_parser{
                        command:command_type::CHANGE_COMMAND(change_command 
                            {   from: from_arg.unwrap(), 
                                to: to_arg.unwrap() }),
                        target_file:_arg,
                    });
                },
            }
        }
        return Err(HELP_MESSAGE.to_string());
    }
}