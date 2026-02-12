pub struct sections{
    pub sectname:[u8;16],
    pub segname:[u8;16],
    pub addr:u64,
    pub size:u64,
    pub offset:u32,
    pub align:u32,
    pub reloff:u32,
    pub nreloc:u32,
    pub flags:u32,
    pub reserved1:u32,
    pub reserved2:u32,
    pub reserved3:u32,
}

pub struct sections32{
    pub sectname:[u8;16],
    pub segname:[u8;16],
    pub addr:u32,
    pub size:u32,
    pub offset:u32,
    pub align:u32,
    pub reloff:u32,
    pub nreloc:u32,
    pub flags:u32,
    pub reserved1:u32,
    pub reserved2:u32,
}

pub struct lc_load_dylib{
    pub cmd:u32,
    pub cmdsize:u32,
    pub str_offset:u32,
    pub timestamp:u32,
    pub current_version:u32,
    pub compatibility_version:u32,
    pub name:String,
}

pub struct lc_weak_dylib{
    pub cmd:u32,
    pub cmdsize:u32,
    pub str_offset:u32,
    pub timestamp:u32,
    pub current_version:u32,
    pub compatibility_version:u32,
    pub name:String,
}

pub struct lc_id_dylib{
    pub cmd:u32,
    pub cmdsize:u32,
    pub str_offset:u32,
    pub timestamp:u32,
    pub current_version:u32,
    pub compatibility_version:u32,
    pub name:String,
}

pub struct lc_segment_64{
    pub cmd:u32,
    pub cmdsize:u32,
    pub segname:[u8;16],
    pub vmaddr:u64,
    pub vmsize:u64,
    pub fileoff:u64,
    pub filesize:u64,
    pub maxprot:u32,
    pub initprot:u32,
    pub sections:Vec<sections>,
    pub flags:u32,
}

pub struct lc_segment{
    pub cmd:u32,
    pub cmdsize:u32,
    pub segname:[u8;16],
    pub vmaddr:u32,
    pub vmsize:u32,
    pub fileoff:u32,
    pub filesize:u32,
    pub maxprot:u32,
    pub initprot:u32,
    pub sections:Vec<sections32>,
    pub flags:u32,
}

pub struct lc_dylib_info_only{
    pub cmd:u32,
    pub cmdsize:u32,
    pub rebase_off:u32,
    pub rebase_size:u32,
    pub bind_off:u32,
    pub bind_size:u32,
    pub weak_bind_off:u32,
    pub weak_bind_size:u32,
    pub lazy_bind_off:u32,
    pub lazy_bind_size:u32,
    pub export_off:u32,
    pub export_size:u32,
}

pub struct lc_symtab{
    pub cmd:u32,
    pub cmdsize:u32,
    pub symoff:u32,
    pub nsyms:u32,
    pub stroff:u32,
    pub strsize:u32,
}

pub struct other_command{
    pub cmd:u32,
    pub cmdsize:u32,
    // other fields as needed
}


pub enum load_command{
    LC_LOAD_DYLIB(lc_load_dylib),
    LC_WEAK_DYLIB(lc_weak_dylib),
    LC_ID_DYLIB(lc_id_dylib),
    LC_SEGMENT_64(lc_segment_64),
    LC_SEGMENT(lc_segment),
    LC_DYLIB_INFO_ONLY(lc_dylib_info_only),
    LC_SYMTAB(lc_symtab),
    OTHER_COMMAND(other_command),
}