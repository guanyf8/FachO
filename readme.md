# FachO

FachO is a cli for IOS developer to modify their dylibs whose dependencies inside cannot be loaded correctly.

It can be used to modify the sources of symbols by change the ordinal of the dylib, so the specific symbols can find their implementations.

## Usage
```bash
facho -list my/local/path/xxx.framework/xxx

facho -from @rpath/libfoo.dylib -to @rpath/libbar.dylib my/local/path/xxx.framework/xxx # if the latter exists, replace the former's load path to the latter's one, otherwise do nothing.
facho -from "2" -to "3" my/local/path/xxx.framework/xxx # change ordinals 2 to 3, so symbols used to belonged to 2 will find their implementations in 3
facho -from @rpath/libfoo.dylib -to "-1" my/local/path/xxx.framework/xxx # change the ordinal to -1 so symbols look up in executable
facho -from @rpath/libfoo.dylib -to "-2" my/local/path/xxx.framework/xxx # change the ordinal to -2 so symbols dynamic look up

facho -change "_lua_newstate" -from @rpath/libfoo.dylib -to "-2" my/local/path/xxx.framework/xxx # change specific symbol to -2
facho -change "_luaL?_.*" -from @rpath/libfoo.dylib -to "-2" my/local/path/xxx.framework/xxx  #regax is supported
facho -change "symbols.txt"  -from @rpath/libfoo.dylib -to "-2" my/local/path/xxx.framework/xxx  # given a txt is ok

facho -add @rpath/new.dylib my/local/path/xxx.framework/xxx # add the dylib to the beginning
facho -delete @rpath/libfoo.dylib my/local/path/xxx.framework/xxx # delete the first dylib it finds and exit
```

attention! If a load path appears more than once, IOS may refuse to load it and crash. But don't worry, fachO can deal to it automatically.


