# FachO

FachO is a cli for IOS developer to modify their dylibs whose dependencies inside cannot be loaded correctly.

It can be used to modify the sources of symbols by change the ordinal of the dylib, so the specific symbols can find their implementations.

## Usage
```bash
facho -list my/local/path/xxx.framework/xxx

facho -change "2" -to "3" my/local/path/xxx.framework/xxx # change ordinals 2 to 3, so symbols used to belonged to 2 will find their implementations in 3
```


