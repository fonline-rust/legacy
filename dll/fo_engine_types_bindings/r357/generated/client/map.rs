/* automatically generated by rust-bindgen 0.59.2 */

#[allow(unused_imports)] use super::*;


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[repr(align(4))]
pub struct MapObject {
    pub _bindgen_opaque_blob: [u32; 102usize],
}
#[repr(C)]
pub struct MapObject__bindgen_ty_1 {
    pub MCritter: __BindgenUnionField<MapObject__bindgen_ty_1__bindgen_ty_1>,
    pub MItem: __BindgenUnionField<MapObject__bindgen_ty_1__bindgen_ty_2>,
    pub MScenery: __BindgenUnionField<MapObject__bindgen_ty_1__bindgen_ty_3>,
    pub bindgen_union_field: [u32; 63usize],
}
#[repr(C)]
pub struct MapObject__bindgen_ty_1__bindgen_ty_1 {
    pub Cond: uint8,
    pub Anim1: uint,
    pub Anim2: uint,
    pub ParamIndex: [int16; 40usize],
    pub ParamValue: [::std::os::raw::c_int; 40usize],
}
#[test]
fn bindgen_test_layout_MapObject__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<MapObject__bindgen_ty_1__bindgen_ty_1>(),
        252usize,
        concat!(
            "Size of: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MapObject__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_1>())).Cond as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Cond)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_1>())).Anim1 as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Anim1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_1>())).Anim2 as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Anim2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_1>())).ParamIndex as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(ParamIndex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_1>())).ParamValue as *const _
                as usize
        },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(ParamValue)
        )
    );
}
#[repr(C)]
pub struct MapObject__bindgen_ty_1__bindgen_ty_2 {
    pub OffsetX: int16,
    pub OffsetY: int16,
    pub AnimStayBegin: uint8,
    pub AnimStayEnd: uint8,
    pub AnimWait: uint16,
    pub InfoOffset: uint8,
    pub PicMapHash: uint,
    pub PicInvHash: uint,
    pub Count: uint,
    pub ItemSlot: uint8,
    pub BrokenFlags: uint8,
    pub BrokenCount: uint8,
    pub Deterioration: uint16,
    pub AmmoPid: uint16,
    pub AmmoCount: uint,
    pub LockerDoorId: uint,
    pub LockerCondition: uint16,
    pub LockerComplexity: uint16,
    pub TrapValue: int16,
    pub Val: [::std::os::raw::c_int; 10usize],
}
#[test]
fn bindgen_test_layout_MapObject__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<MapObject__bindgen_ty_1__bindgen_ty_2>(),
        88usize,
        concat!(
            "Size of: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MapObject__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).OffsetX as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(OffsetX)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).OffsetY as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(OffsetY)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).AnimStayBegin
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(AnimStayBegin)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).AnimStayEnd
                as *const _ as usize
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(AnimStayEnd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).AnimWait as *const _
                as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(AnimWait)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).InfoOffset as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(InfoOffset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).PicMapHash as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(PicMapHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).PicInvHash as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(PicInvHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).Count as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(Count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).ItemSlot as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(ItemSlot)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).BrokenFlags
                as *const _ as usize
        },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(BrokenFlags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).BrokenCount
                as *const _ as usize
        },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(BrokenCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).Deterioration
                as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(Deterioration)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).AmmoPid as *const _
                as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(AmmoPid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).AmmoCount as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(AmmoCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).LockerDoorId
                as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(LockerDoorId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).LockerCondition
                as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(LockerCondition)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).LockerComplexity
                as *const _ as usize
        },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(LockerComplexity)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).TrapValue as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(TrapValue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_2>())).Val as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(Val)
        )
    );
}
#[repr(C)]
pub struct MapObject__bindgen_ty_1__bindgen_ty_3 {
    pub OffsetX: int16,
    pub OffsetY: int16,
    pub AnimStayBegin: uint8,
    pub AnimStayEnd: uint8,
    pub AnimWait: uint16,
    pub InfoOffset: uint8,
    pub PicMapHash: uint,
    pub PicInvHash: uint,
    pub CanUse: bool,
    pub CanTalk: bool,
    pub TriggerNum: uint,
    pub ParamsCount: uint8,
    pub Param: [::std::os::raw::c_int; 5usize],
    pub ToMapPid: uint16,
    pub ToEntire: uint,
    pub ToDir: uint8,
}
#[test]
fn bindgen_test_layout_MapObject__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<MapObject__bindgen_ty_1__bindgen_ty_3>(),
        64usize,
        concat!(
            "Size of: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MapObject__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).OffsetX as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(OffsetX)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).OffsetY as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(OffsetY)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).AnimStayBegin
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(AnimStayBegin)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).AnimStayEnd
                as *const _ as usize
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(AnimStayEnd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).AnimWait as *const _
                as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(AnimWait)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).InfoOffset as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(InfoOffset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).PicMapHash as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(PicMapHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).PicInvHash as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(PicInvHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).CanUse as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(CanUse)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).CanTalk as *const _
                as usize
        },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(CanTalk)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).TriggerNum as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(TriggerNum)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).ParamsCount
                as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(ParamsCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).Param as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(Param)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).ToMapPid as *const _
                as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(ToMapPid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).ToEntire as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(ToEntire)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1__bindgen_ty_3>())).ToDir as *const _
                as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(ToDir)
        )
    );
}
#[test]
fn bindgen_test_layout_MapObject__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<MapObject__bindgen_ty_1>(),
        252usize,
        concat!("Size of: ", stringify!(MapObject__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<MapObject__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(MapObject__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1>())).MCritter as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1),
            "::",
            stringify!(MCritter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MapObject__bindgen_ty_1>())).MItem as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1),
            "::",
            stringify!(MItem)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MapObject__bindgen_ty_1>())).MScenery as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MapObject__bindgen_ty_1),
            "::",
            stringify!(MScenery)
        )
    );
}
#[test]
fn bindgen_test_layout_MapObject() {
    assert_eq!(
        ::std::mem::size_of::<MapObject>(),
        408usize,
        concat!("Size of: ", stringify!(MapObject))
    );
    assert_eq!(
        ::std::mem::align_of::<MapObject>(),
        4usize,
        concat!("Alignment of ", stringify!(MapObject))
    );
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct MapEntire {
    pub _bindgen_opaque_blob: [u32; 3usize],
}
#[test]
fn bindgen_test_layout_MapEntire() {
    assert_eq!(
        ::std::mem::size_of::<MapEntire>(),
        12usize,
        concat!("Size of: ", stringify!(MapEntire))
    );
    assert_eq!(
        ::std::mem::align_of::<MapEntire>(),
        4usize,
        concat!("Alignment of ", stringify!(MapEntire))
    );
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceneryToClient {
    pub _bindgen_opaque_blob: [u32; 8usize],
}
#[test]
fn bindgen_test_layout_SceneryToClient() {
    assert_eq!(
        ::std::mem::size_of::<SceneryToClient>(),
        32usize,
        concat!("Size of: ", stringify!(SceneryToClient))
    );
    assert_eq!(
        ::std::mem::align_of::<SceneryToClient>(),
        4usize,
        concat!("Alignment of ", stringify!(SceneryToClient))
    );
}
#[repr(C)]
#[repr(align(4))]
pub struct ProtoMap {
    pub _bindgen_opaque_blob: [u32; 66usize],
}
#[repr(C)]
pub struct ProtoMap__bindgen_ty_1 {
    pub Version: uint,
    pub MaxHexX: uint16,
    pub MaxHexY: uint16,
    pub WorkHexX: ::std::os::raw::c_int,
    pub WorkHexY: ::std::os::raw::c_int,
    pub ScriptModule: [::std::os::raw::c_char; 65usize],
    pub ScriptFunc: [::std::os::raw::c_char; 65usize],
    pub Time: ::std::os::raw::c_int,
    pub NoLogOut: bool,
    pub DayTime: [::std::os::raw::c_int; 4usize],
    pub DayColor: [uint8; 12usize],
    pub HeaderSize: uint16,
    pub Packed: bool,
    pub UnpackedDataLen: uint,
}
#[test]
fn bindgen_test_layout_ProtoMap__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ProtoMap__bindgen_ty_1>(),
        192usize,
        concat!("Size of: ", stringify!(ProtoMap__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ProtoMap__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(ProtoMap__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).Version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).MaxHexX as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(MaxHexX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).MaxHexY as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(MaxHexY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).WorkHexX as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(WorkHexX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).WorkHexY as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(WorkHexY)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).ScriptModule as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(ScriptModule)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).ScriptFunc as *const _ as usize
        },
        81usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(ScriptFunc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).Time as *const _ as usize },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(Time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).NoLogOut as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(NoLogOut)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).DayTime as *const _ as usize },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(DayTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).DayColor as *const _ as usize },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(DayColor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).HeaderSize as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(HeaderSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).Packed as *const _ as usize },
        186usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(Packed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ProtoMap__bindgen_ty_1>())).UnpackedDataLen as *const _ as usize
        },
        188usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap__bindgen_ty_1),
            "::",
            stringify!(UnpackedDataLen)
        )
    );
}
#[repr(C)]
pub struct ProtoMap_Tile {
    pub NameHash: uint,
    pub HexX: uint16,
    pub HexY: uint16,
    pub OffsX: int8,
    pub OffsY: int8,
    pub Layer: uint8,
    pub IsRoof: bool,
}
#[test]
fn bindgen_test_layout_ProtoMap_Tile() {
    assert_eq!(
        ::std::mem::size_of::<ProtoMap_Tile>(),
        12usize,
        concat!("Size of: ", stringify!(ProtoMap_Tile))
    );
    assert_eq!(
        ::std::mem::align_of::<ProtoMap_Tile>(),
        4usize,
        concat!("Alignment of ", stringify!(ProtoMap_Tile))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).NameHash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(NameHash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).HexX as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(HexX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).HexY as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(HexY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).OffsX as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(OffsX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).OffsY as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(OffsY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).Layer as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(Layer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProtoMap_Tile>())).IsRoof as *const _ as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(ProtoMap_Tile),
            "::",
            stringify!(IsRoof)
        )
    );
}
#[test]
fn bindgen_test_layout_ProtoMap() {
    assert_eq!(
        ::std::mem::size_of::<ProtoMap>(),
        264usize,
        concat!("Size of: ", stringify!(ProtoMap))
    );
    assert_eq!(
        ::std::mem::align_of::<ProtoMap>(),
        4usize,
        concat!("Alignment of ", stringify!(ProtoMap))
    );
}
#[repr(C)]
#[repr(align(4))]
pub struct Map {
    pub _bindgen_opaque_blob: [u32; 194usize],
}
#[repr(C)]
pub struct Map__bindgen_ty_1 {
    pub MapId: uint,
    pub MapPid: uint16,
    pub MapRain: uint8,
    pub IsTurnBasedAviable: bool,
    pub MapTime: ::std::os::raw::c_int,
    pub ScriptId: uint,
    pub MapDayTime: [::std::os::raw::c_int; 4usize],
    pub MapDayColor: [uint8; 12usize],
    pub Reserved: [uint; 20usize],
    pub UserData: [::std::os::raw::c_int; 100usize],
}
#[test]
fn bindgen_test_layout_Map__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<Map__bindgen_ty_1>(),
        524usize,
        concat!("Size of: ", stringify!(Map__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<Map__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(Map__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).MapId as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(MapId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).MapPid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(MapPid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).MapRain as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(MapRain)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Map__bindgen_ty_1>())).IsTurnBasedAviable as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(IsTurnBasedAviable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).MapTime as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(MapTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).ScriptId as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(ScriptId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).MapDayTime as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(MapDayTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).MapDayColor as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(MapDayColor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).Reserved as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(Reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Map__bindgen_ty_1>())).UserData as *const _ as usize },
        124usize,
        concat!(
            "Offset of field: ",
            stringify!(Map__bindgen_ty_1),
            "::",
            stringify!(UserData)
        )
    );
}
#[test]
fn bindgen_test_layout_Map() {
    assert_eq!(
        ::std::mem::size_of::<Map>(),
        776usize,
        concat!("Size of: ", stringify!(Map))
    );
    assert_eq!(
        ::std::mem::align_of::<Map>(),
        4usize,
        concat!("Alignment of ", stringify!(Map))
    );
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct ProtoLocation {
    pub _bindgen_opaque_blob: [u32; 20usize],
}
#[test]
fn bindgen_test_layout_ProtoLocation() {
    assert_eq!(
        ::std::mem::size_of::<ProtoLocation>(),
        80usize,
        concat!("Size of: ", stringify!(ProtoLocation))
    );
    assert_eq!(
        ::std::mem::align_of::<ProtoLocation>(),
        4usize,
        concat!("Alignment of ", stringify!(ProtoLocation))
    );
}
#[repr(C)]
#[repr(align(4))]
pub struct Location {
    pub _bindgen_opaque_blob: [u32; 71usize],
}
#[repr(C)]
pub struct Location__bindgen_ty_1 {
    pub LocId: uint,
    pub LocPid: uint16,
    pub WX: uint16,
    pub WY: uint16,
    pub Radius: uint16,
    pub Visible: bool,
    pub GeckVisible: bool,
    pub AutoGarbage: bool,
    pub ToGarbage: bool,
    pub Color: uint,
    pub Reserved3: [uint; 59usize],
}
#[test]
fn bindgen_test_layout_Location__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<Location__bindgen_ty_1>(),
        256usize,
        concat!("Size of: ", stringify!(Location__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<Location__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(Location__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).LocId as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(LocId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).LocPid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(LocPid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).WX as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(WX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).WY as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(WY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).Radius as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(Radius)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).Visible as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(Visible)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Location__bindgen_ty_1>())).GeckVisible as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(GeckVisible)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Location__bindgen_ty_1>())).AutoGarbage as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(AutoGarbage)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Location__bindgen_ty_1>())).ToGarbage as *const _ as usize
        },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(ToGarbage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Location__bindgen_ty_1>())).Color as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(Color)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Location__bindgen_ty_1>())).Reserved3 as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Location__bindgen_ty_1),
            "::",
            stringify!(Reserved3)
        )
    );
}
#[test]
fn bindgen_test_layout_Location() {
    assert_eq!(
        ::std::mem::size_of::<Location>(),
        284usize,
        concat!("Size of: ", stringify!(Location))
    );
    assert_eq!(
        ::std::mem::align_of::<Location>(),
        4usize,
        concat!("Alignment of ", stringify!(Location))
    );
}
