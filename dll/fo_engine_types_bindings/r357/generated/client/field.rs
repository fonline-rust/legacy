/* automatically generated by rust-bindgen 0.59.2 */

#[allow(unused_imports)] use super::*;


#[repr(C)]
pub struct Field {
    pub Crit: *const CritterCl,
    pub DeadCrits: CrClVec,
    pub ScrX: ::std::os::raw::c_int,
    pub ScrY: ::std::os::raw::c_int,
    pub Tiles: Field_TileVec,
    pub Roofs: Field_TileVec,
    pub Items: ItemVec,
    pub RoofNum: int16,
    pub ScrollBlock: bool,
    pub IsWall: bool,
    pub IsWallSAI: bool,
    pub IsWallTransp: bool,
    pub IsScen: bool,
    pub IsExitGrid: bool,
    pub IsNotPassed: bool,
    pub IsNotRaked: bool,
    pub Corner: uint8,
    pub IsNoLight: bool,
    pub LightValues: [uint8; 3usize],
    pub IsMultihex: bool,
}
#[repr(C)]
pub struct Field_Tile {
    pub Anim: *const ::std::os::raw::c_void,
    pub OffsX: int16,
    pub OffsY: int16,
    pub Layer: uint8,
}
#[test]
fn bindgen_test_layout_Field_Tile() {
    assert_eq!(
        ::std::mem::size_of::<Field_Tile>(),
        12usize,
        concat!("Size of: ", stringify!(Field_Tile))
    );
    assert_eq!(
        ::std::mem::align_of::<Field_Tile>(),
        4usize,
        concat!("Alignment of ", stringify!(Field_Tile))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).Anim as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(Anim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).OffsX as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(OffsX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).OffsY as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(OffsY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).Layer as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(Layer)
        )
    );
}
#[test]
fn bindgen_test_layout_Field() {
    assert_eq!(
        ::std::mem::size_of::<Field>(),
        76usize,
        concat!("Size of: ", stringify!(Field))
    );
    assert_eq!(
        ::std::mem::align_of::<Field>(),
        4usize,
        concat!("Alignment of ", stringify!(Field))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Crit as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Crit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).DeadCrits as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(DeadCrits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).ScrX as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(ScrX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).ScrY as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(ScrY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Tiles as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Tiles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Roofs as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Roofs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Items as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Items)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).RoofNum as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(RoofNum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).ScrollBlock as *const _ as usize },
        62usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(ScrollBlock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsWall as *const _ as usize },
        63usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsWall)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsWallSAI as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsWallSAI)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsWallTransp as *const _ as usize },
        65usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsWallTransp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsScen as *const _ as usize },
        66usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsScen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsExitGrid as *const _ as usize },
        67usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsExitGrid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsNotPassed as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsNotPassed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsNotRaked as *const _ as usize },
        69usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsNotRaked)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Corner as *const _ as usize },
        70usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Corner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsNoLight as *const _ as usize },
        71usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsNoLight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).LightValues as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(LightValues)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).IsMultihex as *const _ as usize },
        75usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(IsMultihex)
        )
    );
}
