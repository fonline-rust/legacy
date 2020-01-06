dynamic_ffi!(ServerApi, 

    pub fn Cl_RunClientScript(
        cl: *mut Critter,
        func_name: *const ::std::os::raw::c_char,
        p0: ::std::os::raw::c_int,
        p1: ::std::os::raw::c_int,
        p2: ::std::os::raw::c_int,
        p3: *const ::std::os::raw::c_char,
        p4: *const uint,
        p4_size: usize,
    ) -> bool;

    pub fn Global_GetCritter(crid: uint) -> *mut Critter;

    pub fn Global_GetMsgStr(lang: usize, textMsg: usize, strNum: uint) -> *mut ScriptString;

    pub fn Item_GetLexems(item: *mut Item) -> *mut ScriptString;

    pub fn ConstantsManager_GetValue(
        collection: usize,
        string: *mut ScriptString,
    ) -> ::std::os::raw::c_int;
);
