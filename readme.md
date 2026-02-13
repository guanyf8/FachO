# FachO

FachO is a cli for IOS developer to modify dylibs whose dependencies inside cannot be loaded correctly.

It can be used to modify the sources of symbols by change the ordinal of the dylib, so the specific symbols can find their implementations.

## Usage
```bash
facho -list my/local/path/bar.framework/bar

facho -change "2" -to "3" my/local/path/bar.framework/bar # change ordinals 2 to 3, so symbols used to belonged to 2 will find their implementations in 3

facho -show _luaL_newstate my/local/path/bar.framework/bar  # show which ordinal of dylib _luaL_newstate belongs to
```

## coming soon
```bash
facho -add "foo.framework/foo" my/local/path/bar.framework/bar # add a dependency
facho -remove "foo.framework/foo" my/local/path/bar.framework/bar # remove a dependency while whose symbols all change to -2 (dynamic look up)
facho -replace "foo.framework/foo" -with "other.framework/other" my/local/path/bar.framework/bar # replace a dependency
facho -symbol "_luaL_newstate" -change "2" -to "3" my/local/path/bar.framework/bar # change the ordinal of _luaL_newstate to 3
facho -symbols "symbols.txt" -change "2" -to "3" my/local/path/bar.framework/bar # change the ordinal of symbols in symbols.txt to 3
facho -symbols "_luaL?.*" -change "2" -to "3" my/local/path/bar.framework/bar # change the ordinal of symbols that match _luaL?.* to 3
```

