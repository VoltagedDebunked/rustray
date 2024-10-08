pub const RAYLIB_VERSION_MAJOR: u32 = 5;
pub const RAYLIB_VERSION_MINOR: u32 = 5;
pub const RAYLIB_VERSION_PATCH: u32 = 0;
pub const RAYLIB_VERSION: &[u8; 8usize] = b"5.5-dev\0";
pub const PI: f64 = 3.141592653589793;
pub const DEG2RAD: f64 = 0.017453292519943295;
pub const RAD2DEG: f64 = 57.29577951308232;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[test]
fn bindgen_test_layout_Vector2() {
    const UNINIT: ::std::mem::MaybeUninit<Vector2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector2>(),
        8usize,
        concat!("Size of: ", stringify!(Vector2))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector2>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector2),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector2),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[test]
fn bindgen_test_layout_Vector3() {
    const UNINIT: ::std::mem::MaybeUninit<Vector3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector3>(),
        12usize,
        concat!("Size of: ", stringify!(Vector3))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector3>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector3))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[test]
fn bindgen_test_layout_Vector4() {
    const UNINIT: ::std::mem::MaybeUninit<Vector4> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector4>(),
        16usize,
        concat!("Size of: ", stringify!(Vector4))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector4>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector4))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(w)
        )
    );
}
pub type Quaternion = Vector4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}
#[test]
fn bindgen_test_layout_Matrix() {
    const UNINIT: ::std::mem::MaybeUninit<Matrix> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Matrix>(),
        64usize,
        concat!("Size of: ", stringify!(Matrix))
    );
    assert_eq!(
        ::std::mem::align_of::<Matrix>(),
        4usize,
        concat!("Alignment of ", stringify!(Matrix))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m4) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m8) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m12) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m12)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m1) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m5) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m9) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m9)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m13) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m13)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m2) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m6) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m10) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m14) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m14)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m3) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m7) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m7)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m11) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m11)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m15) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m15)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_Color() {
    const UNINIT: ::std::mem::MaybeUninit<Color> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Color>(),
        4usize,
        concat!("Size of: ", stringify!(Color))
    );
    assert_eq!(
        ::std::mem::align_of::<Color>(),
        1usize,
        concat!("Alignment of ", stringify!(Color))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(r))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(g))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        3usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(a))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[test]
fn bindgen_test_layout_Rectangle() {
    const UNINIT: ::std::mem::MaybeUninit<Rectangle> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Rectangle>(),
        16usize,
        concat!("Size of: ", stringify!(Rectangle))
    );
    assert_eq!(
        ::std::mem::align_of::<Rectangle>(),
        4usize,
        concat!("Alignment of ", stringify!(Rectangle))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut ::std::os::raw::c_void,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mipmaps: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Image() {
    const UNINIT: ::std::mem::MaybeUninit<Image> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Image>(),
        24usize,
        concat!("Size of: ", stringify!(Image))
    );
    assert_eq!(
        ::std::mem::align_of::<Image>(),
        8usize,
        concat!("Alignment of ", stringify!(Image))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mipmaps) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(mipmaps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(format)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub id: ::std::os::raw::c_uint,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mipmaps: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Texture() {
    const UNINIT: ::std::mem::MaybeUninit<Texture> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Texture>(),
        20usize,
        concat!("Size of: ", stringify!(Texture))
    );
    assert_eq!(
        ::std::mem::align_of::<Texture>(),
        4usize,
        concat!("Alignment of ", stringify!(Texture))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mipmaps) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(mipmaps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(format)
        )
    );
}
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: ::std::os::raw::c_uint,
    pub texture: Texture,
    pub depth: Texture,
}
#[test]
fn bindgen_test_layout_RenderTexture() {
    const UNINIT: ::std::mem::MaybeUninit<RenderTexture> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RenderTexture>(),
        44usize,
        concat!("Size of: ", stringify!(RenderTexture))
    );
    assert_eq!(
        ::std::mem::align_of::<RenderTexture>(),
        4usize,
        concat!("Alignment of ", stringify!(RenderTexture))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RenderTexture),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RenderTexture),
            "::",
            stringify!(texture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RenderTexture),
            "::",
            stringify!(depth)
        )
    );
}
pub type RenderTexture2D = RenderTexture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub layout: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NPatchInfo() {
    const UNINIT: ::std::mem::MaybeUninit<NPatchInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<NPatchInfo>(),
        36usize,
        concat!("Size of: ", stringify!(NPatchInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<NPatchInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(NPatchInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).left) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).top) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(top)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).right) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(right)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bottom) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(bottom)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).layout) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(layout)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfo {
    pub value: ::std::os::raw::c_int,
    pub offsetX: ::std::os::raw::c_int,
    pub offsetY: ::std::os::raw::c_int,
    pub advanceX: ::std::os::raw::c_int,
    pub image: Image,
}
#[test]
fn bindgen_test_layout_GlyphInfo() {
    const UNINIT: ::std::mem::MaybeUninit<GlyphInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<GlyphInfo>(),
        40usize,
        concat!("Size of: ", stringify!(GlyphInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<GlyphInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(GlyphInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offsetX) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(offsetX)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offsetY) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(offsetY)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).advanceX) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(advanceX)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).image) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(image)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub baseSize: ::std::os::raw::c_int,
    pub glyphCount: ::std::os::raw::c_int,
    pub glyphPadding: ::std::os::raw::c_int,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
#[test]
fn bindgen_test_layout_Font() {
    const UNINIT: ::std::mem::MaybeUninit<Font> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Font>(),
        48usize,
        concat!("Size of: ", stringify!(Font))
    );
    assert_eq!(
        ::std::mem::align_of::<Font>(),
        8usize,
        concat!("Alignment of ", stringify!(Font))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baseSize) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(baseSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).glyphCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(glyphCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).glyphPadding) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(glyphPadding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(texture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).recs) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(recs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).glyphs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(glyphs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Camera3D() {
    const UNINIT: ::std::mem::MaybeUninit<Camera3D> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Camera3D>(),
        44usize,
        concat!("Size of: ", stringify!(Camera3D))
    );
    assert_eq!(
        ::std::mem::align_of::<Camera3D>(),
        4usize,
        concat!("Alignment of ", stringify!(Camera3D))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).position) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(position)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).up) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(up)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fovy) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(fovy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).projection) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(projection)
        )
    );
}
pub type Camera = Camera3D;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}
#[test]
fn bindgen_test_layout_Camera2D() {
    const UNINIT: ::std::mem::MaybeUninit<Camera2D> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Camera2D>(),
        24usize,
        concat!("Size of: ", stringify!(Camera2D))
    );
    assert_eq!(
        ::std::mem::align_of::<Camera2D>(),
        4usize,
        concat!("Alignment of ", stringify!(Camera2D))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rotation) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(rotation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).zoom) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(zoom)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertexCount: ::std::os::raw::c_int,
    pub triangleCount: ::std::os::raw::c_int,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub texcoords2: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut ::std::os::raw::c_uchar,
    pub indices: *mut ::std::os::raw::c_ushort,
    pub animVertices: *mut f32,
    pub animNormals: *mut f32,
    pub boneIds: *mut ::std::os::raw::c_uchar,
    pub boneWeights: *mut f32,
    pub boneMatrices: *mut Matrix,
    pub boneCount: ::std::os::raw::c_int,
    pub vaoId: ::std::os::raw::c_uint,
    pub vboId: *mut ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Mesh() {
    const UNINIT: ::std::mem::MaybeUninit<Mesh> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Mesh>(),
        120usize,
        concat!("Size of: ", stringify!(Mesh))
    );
    assert_eq!(
        ::std::mem::align_of::<Mesh>(),
        8usize,
        concat!("Alignment of ", stringify!(Mesh))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vertexCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vertexCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).triangleCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(triangleCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vertices) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vertices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texcoords) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(texcoords)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texcoords2) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(texcoords2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).normals) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(normals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tangents) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(tangents)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colors) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(colors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).indices) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(indices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).animVertices) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(animVertices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).animNormals) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(animNormals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneIds) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(boneIds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneWeights) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(boneWeights)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneMatrices) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(boneMatrices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneCount) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(boneCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vaoId) as usize - ptr as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vaoId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vboId) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vboId)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub id: ::std::os::raw::c_uint,
    pub locs: *mut ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Shader() {
    const UNINIT: ::std::mem::MaybeUninit<Shader> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Shader>(),
        16usize,
        concat!("Size of: ", stringify!(Shader))
    );
    assert_eq!(
        ::std::mem::align_of::<Shader>(),
        8usize,
        concat!("Alignment of ", stringify!(Shader))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Shader),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).locs) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Shader),
            "::",
            stringify!(locs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
    pub texture: Texture2D,
    pub color: Color,
    pub value: f32,
}
#[test]
fn bindgen_test_layout_MaterialMap() {
    const UNINIT: ::std::mem::MaybeUninit<MaterialMap> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<MaterialMap>(),
        28usize,
        concat!("Size of: ", stringify!(MaterialMap))
    );
    assert_eq!(
        ::std::mem::align_of::<MaterialMap>(),
        4usize,
        concat!("Alignment of ", stringify!(MaterialMap))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MaterialMap),
            "::",
            stringify!(texture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).color) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MaterialMap),
            "::",
            stringify!(color)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MaterialMap),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub shader: Shader,
    pub maps: *mut MaterialMap,
    pub params: [f32; 4usize],
}
#[test]
fn bindgen_test_layout_Material() {
    const UNINIT: ::std::mem::MaybeUninit<Material> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Material>(),
        40usize,
        concat!("Size of: ", stringify!(Material))
    );
    assert_eq!(
        ::std::mem::align_of::<Material>(),
        8usize,
        concat!("Alignment of ", stringify!(Material))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shader) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Material),
            "::",
            stringify!(shader)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maps) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Material),
            "::",
            stringify!(maps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).params) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Material),
            "::",
            stringify!(params)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
#[test]
fn bindgen_test_layout_Transform() {
    const UNINIT: ::std::mem::MaybeUninit<Transform> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Transform>(),
        40usize,
        concat!("Size of: ", stringify!(Transform))
    );
    assert_eq!(
        ::std::mem::align_of::<Transform>(),
        4usize,
        concat!("Alignment of ", stringify!(Transform))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).translation) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Transform),
            "::",
            stringify!(translation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rotation) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Transform),
            "::",
            stringify!(rotation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scale) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Transform),
            "::",
            stringify!(scale)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoneInfo {
    pub name: [::std::os::raw::c_char; 32usize],
    pub parent: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_BoneInfo() {
    const UNINIT: ::std::mem::MaybeUninit<BoneInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<BoneInfo>(),
        36usize,
        concat!("Size of: ", stringify!(BoneInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<BoneInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(BoneInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BoneInfo),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(BoneInfo),
            "::",
            stringify!(parent)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
    pub transform: Matrix,
    pub meshCount: ::std::os::raw::c_int,
    pub materialCount: ::std::os::raw::c_int,
    pub meshes: *mut Mesh,
    pub materials: *mut Material,
    pub meshMaterial: *mut ::std::os::raw::c_int,
    pub boneCount: ::std::os::raw::c_int,
    pub bones: *mut BoneInfo,
    pub bindPose: *mut Transform,
}
#[test]
fn bindgen_test_layout_Model() {
    const UNINIT: ::std::mem::MaybeUninit<Model> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Model>(),
        120usize,
        concat!("Size of: ", stringify!(Model))
    );
    assert_eq!(
        ::std::mem::align_of::<Model>(),
        8usize,
        concat!("Alignment of ", stringify!(Model))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).transform) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(transform)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).meshCount) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(meshCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).materialCount) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(materialCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).meshes) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(meshes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).materials) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(materials)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).meshMaterial) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(meshMaterial)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneCount) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(boneCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bones) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(bones)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bindPose) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(bindPose)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelAnimation {
    pub boneCount: ::std::os::raw::c_int,
    pub frameCount: ::std::os::raw::c_int,
    pub bones: *mut BoneInfo,
    pub framePoses: *mut *mut Transform,
    pub name: [::std::os::raw::c_char; 32usize],
}
#[test]
fn bindgen_test_layout_ModelAnimation() {
    const UNINIT: ::std::mem::MaybeUninit<ModelAnimation> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ModelAnimation>(),
        56usize,
        concat!("Size of: ", stringify!(ModelAnimation))
    );
    assert_eq!(
        ::std::mem::align_of::<ModelAnimation>(),
        8usize,
        concat!("Alignment of ", stringify!(ModelAnimation))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(boneCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(frameCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bones) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(bones)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).framePoses) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(framePoses)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(name)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}
#[test]
fn bindgen_test_layout_Ray() {
    const UNINIT: ::std::mem::MaybeUninit<Ray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Ray>(),
        24usize,
        concat!("Size of: ", stringify!(Ray))
    );
    assert_eq!(
        ::std::mem::align_of::<Ray>(),
        4usize,
        concat!("Alignment of ", stringify!(Ray))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).position) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Ray),
            "::",
            stringify!(position)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).direction) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Ray),
            "::",
            stringify!(direction)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayCollision {
    pub hit: bool,
    pub distance: f32,
    pub point: Vector3,
    pub normal: Vector3,
}
#[test]
fn bindgen_test_layout_RayCollision() {
    const UNINIT: ::std::mem::MaybeUninit<RayCollision> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RayCollision>(),
        32usize,
        concat!("Size of: ", stringify!(RayCollision))
    );
    assert_eq!(
        ::std::mem::align_of::<RayCollision>(),
        4usize,
        concat!("Alignment of ", stringify!(RayCollision))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hit) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(hit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).distance) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(distance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).point) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(point)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).normal) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(normal)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}
#[test]
fn bindgen_test_layout_BoundingBox() {
    const UNINIT: ::std::mem::MaybeUninit<BoundingBox> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<BoundingBox>(),
        24usize,
        concat!("Size of: ", stringify!(BoundingBox))
    );
    assert_eq!(
        ::std::mem::align_of::<BoundingBox>(),
        4usize,
        concat!("Alignment of ", stringify!(BoundingBox))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BoundingBox),
            "::",
            stringify!(min)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(BoundingBox),
            "::",
            stringify!(max)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub frameCount: ::std::os::raw::c_uint,
    pub sampleRate: ::std::os::raw::c_uint,
    pub sampleSize: ::std::os::raw::c_uint,
    pub channels: ::std::os::raw::c_uint,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_Wave() {
    const UNINIT: ::std::mem::MaybeUninit<Wave> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Wave>(),
        24usize,
        concat!("Size of: ", stringify!(Wave))
    );
    assert_eq!(
        ::std::mem::align_of::<Wave>(),
        8usize,
        concat!("Alignment of ", stringify!(Wave))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(frameCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleRate) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(sampleRate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(sampleSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channels) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioProcessor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStream {
    pub buffer: *mut rAudioBuffer,
    pub processor: *mut rAudioProcessor,
    pub sampleRate: ::std::os::raw::c_uint,
    pub sampleSize: ::std::os::raw::c_uint,
    pub channels: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_AudioStream() {
    const UNINIT: ::std::mem::MaybeUninit<AudioStream> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AudioStream>(),
        32usize,
        concat!("Size of: ", stringify!(AudioStream))
    );
    assert_eq!(
        ::std::mem::align_of::<AudioStream>(),
        8usize,
        concat!("Alignment of ", stringify!(AudioStream))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).processor) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(processor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleRate) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(sampleRate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleSize) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(sampleSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channels) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(channels)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
    pub stream: AudioStream,
    pub frameCount: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Sound() {
    const UNINIT: ::std::mem::MaybeUninit<Sound> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Sound>(),
        40usize,
        concat!("Size of: ", stringify!(Sound))
    );
    assert_eq!(
        ::std::mem::align_of::<Sound>(),
        8usize,
        concat!("Alignment of ", stringify!(Sound))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stream) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Sound),
            "::",
            stringify!(stream)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Sound),
            "::",
            stringify!(frameCount)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
    pub stream: AudioStream,
    pub frameCount: ::std::os::raw::c_uint,
    pub looping: bool,
    pub ctxType: ::std::os::raw::c_int,
    pub ctxData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_Music() {
    const UNINIT: ::std::mem::MaybeUninit<Music> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Music>(),
        56usize,
        concat!("Size of: ", stringify!(Music))
    );
    assert_eq!(
        ::std::mem::align_of::<Music>(),
        8usize,
        concat!("Alignment of ", stringify!(Music))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stream) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(stream)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(frameCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).looping) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(looping)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ctxType) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(ctxType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ctxData) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(ctxData)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrDeviceInfo {
    pub hResolution: ::std::os::raw::c_int,
    pub vResolution: ::std::os::raw::c_int,
    pub hScreenSize: f32,
    pub vScreenSize: f32,
    pub eyeToScreenDistance: f32,
    pub lensSeparationDistance: f32,
    pub interpupillaryDistance: f32,
    pub lensDistortionValues: [f32; 4usize],
    pub chromaAbCorrection: [f32; 4usize],
}
#[test]
fn bindgen_test_layout_VrDeviceInfo() {
    const UNINIT: ::std::mem::MaybeUninit<VrDeviceInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VrDeviceInfo>(),
        60usize,
        concat!("Size of: ", stringify!(VrDeviceInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<VrDeviceInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(VrDeviceInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hResolution) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(hResolution)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vResolution) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(vResolution)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hScreenSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(hScreenSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vScreenSize) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(vScreenSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).eyeToScreenDistance) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(eyeToScreenDistance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lensSeparationDistance) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(lensSeparationDistance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interpupillaryDistance) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(interpupillaryDistance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lensDistortionValues) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(lensDistortionValues)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).chromaAbCorrection) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(chromaAbCorrection)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2usize],
    pub viewOffset: [Matrix; 2usize],
    pub leftLensCenter: [f32; 2usize],
    pub rightLensCenter: [f32; 2usize],
    pub leftScreenCenter: [f32; 2usize],
    pub rightScreenCenter: [f32; 2usize],
    pub scale: [f32; 2usize],
    pub scaleIn: [f32; 2usize],
}
#[test]
fn bindgen_test_layout_VrStereoConfig() {
    const UNINIT: ::std::mem::MaybeUninit<VrStereoConfig> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VrStereoConfig>(),
        304usize,
        concat!("Size of: ", stringify!(VrStereoConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<VrStereoConfig>(),
        4usize,
        concat!("Alignment of ", stringify!(VrStereoConfig))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).projection) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(projection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).viewOffset) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(viewOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).leftLensCenter) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(leftLensCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rightLensCenter) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(rightLensCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).leftScreenCenter) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(leftScreenCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rightScreenCenter) as usize - ptr as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(rightScreenCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scale) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(scale)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scaleIn) as usize - ptr as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(scaleIn)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FilePathList {
    pub capacity: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub paths: *mut *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_FilePathList() {
    const UNINIT: ::std::mem::MaybeUninit<FilePathList> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<FilePathList>(),
        16usize,
        concat!("Size of: ", stringify!(FilePathList))
    );
    assert_eq!(
        ::std::mem::align_of::<FilePathList>(),
        8usize,
        concat!("Alignment of ", stringify!(FilePathList))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).capacity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FilePathList),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FilePathList),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).paths) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FilePathList),
            "::",
            stringify!(paths)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEvent {
    pub frame: ::std::os::raw::c_uint,
    pub type_: ::std::os::raw::c_uint,
    pub params: [::std::os::raw::c_int; 4usize],
}
#[test]
fn bindgen_test_layout_AutomationEvent() {
    const UNINIT: ::std::mem::MaybeUninit<AutomationEvent> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AutomationEvent>(),
        24usize,
        concat!("Size of: ", stringify!(AutomationEvent))
    );
    assert_eq!(
        ::std::mem::align_of::<AutomationEvent>(),
        4usize,
        concat!("Alignment of ", stringify!(AutomationEvent))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frame) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEvent),
            "::",
            stringify!(frame)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEvent),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).params) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEvent),
            "::",
            stringify!(params)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEventList {
    pub capacity: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub events: *mut AutomationEvent,
}
#[test]
fn bindgen_test_layout_AutomationEventList() {
    const UNINIT: ::std::mem::MaybeUninit<AutomationEventList> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AutomationEventList>(),
        16usize,
        concat!("Size of: ", stringify!(AutomationEventList))
    );
    assert_eq!(
        ::std::mem::align_of::<AutomationEventList>(),
        8usize,
        concat!("Alignment of ", stringify!(AutomationEventList))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).capacity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEventList),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEventList),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).events) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEventList),
            "::",
            stringify!(events)
        )
    );
}
pub const ConfigFlags_FLAG_VSYNC_HINT: ConfigFlags = 64;
pub const ConfigFlags_FLAG_FULLSCREEN_MODE: ConfigFlags = 2;
pub const ConfigFlags_FLAG_WINDOW_RESIZABLE: ConfigFlags = 4;
pub const ConfigFlags_FLAG_WINDOW_UNDECORATED: ConfigFlags = 8;
pub const ConfigFlags_FLAG_WINDOW_HIDDEN: ConfigFlags = 128;
pub const ConfigFlags_FLAG_WINDOW_MINIMIZED: ConfigFlags = 512;
pub const ConfigFlags_FLAG_WINDOW_MAXIMIZED: ConfigFlags = 1024;
pub const ConfigFlags_FLAG_WINDOW_UNFOCUSED: ConfigFlags = 2048;
pub const ConfigFlags_FLAG_WINDOW_TOPMOST: ConfigFlags = 4096;
pub const ConfigFlags_FLAG_WINDOW_ALWAYS_RUN: ConfigFlags = 256;
pub const ConfigFlags_FLAG_WINDOW_TRANSPARENT: ConfigFlags = 16;
pub const ConfigFlags_FLAG_WINDOW_HIGHDPI: ConfigFlags = 8192;
pub const ConfigFlags_FLAG_WINDOW_MOUSE_PASSTHROUGH: ConfigFlags = 16384;
pub const ConfigFlags_FLAG_BORDERLESS_WINDOWED_MODE: ConfigFlags = 32768;
pub const ConfigFlags_FLAG_MSAA_4X_HINT: ConfigFlags = 32;
pub const ConfigFlags_FLAG_INTERLACED_HINT: ConfigFlags = 65536;
pub type ConfigFlags = ::std::os::raw::c_uint;
pub const TraceLogLevel_LOG_ALL: TraceLogLevel = 0;
pub const TraceLogLevel_LOG_TRACE: TraceLogLevel = 1;
pub const TraceLogLevel_LOG_DEBUG: TraceLogLevel = 2;
pub const TraceLogLevel_LOG_INFO: TraceLogLevel = 3;
pub const TraceLogLevel_LOG_WARNING: TraceLogLevel = 4;
pub const TraceLogLevel_LOG_ERROR: TraceLogLevel = 5;
pub const TraceLogLevel_LOG_FATAL: TraceLogLevel = 6;
pub const TraceLogLevel_LOG_NONE: TraceLogLevel = 7;
pub type TraceLogLevel = ::std::os::raw::c_uint;
pub const KeyboardKey_KEY_NULL: KeyboardKey = 0;
pub const KeyboardKey_KEY_APOSTROPHE: KeyboardKey = 39;
pub const KeyboardKey_KEY_COMMA: KeyboardKey = 44;
pub const KeyboardKey_KEY_MINUS: KeyboardKey = 45;
pub const KeyboardKey_KEY_PERIOD: KeyboardKey = 46;
pub const KeyboardKey_KEY_SLASH: KeyboardKey = 47;
pub const KeyboardKey_KEY_ZERO: KeyboardKey = 48;
pub const KeyboardKey_KEY_ONE: KeyboardKey = 49;
pub const KeyboardKey_KEY_TWO: KeyboardKey = 50;
pub const KeyboardKey_KEY_THREE: KeyboardKey = 51;
pub const KeyboardKey_KEY_FOUR: KeyboardKey = 52;
pub const KeyboardKey_KEY_FIVE: KeyboardKey = 53;
pub const KeyboardKey_KEY_SIX: KeyboardKey = 54;
pub const KeyboardKey_KEY_SEVEN: KeyboardKey = 55;
pub const KeyboardKey_KEY_EIGHT: KeyboardKey = 56;
pub const KeyboardKey_KEY_NINE: KeyboardKey = 57;
pub const KeyboardKey_KEY_SEMICOLON: KeyboardKey = 59;
pub const KeyboardKey_KEY_EQUAL: KeyboardKey = 61;
pub const KeyboardKey_KEY_A: KeyboardKey = 65;
pub const KeyboardKey_KEY_B: KeyboardKey = 66;
pub const KeyboardKey_KEY_C: KeyboardKey = 67;
pub const KeyboardKey_KEY_D: KeyboardKey = 68;
pub const KeyboardKey_KEY_E: KeyboardKey = 69;
pub const KeyboardKey_KEY_F: KeyboardKey = 70;
pub const KeyboardKey_KEY_G: KeyboardKey = 71;
pub const KeyboardKey_KEY_H: KeyboardKey = 72;
pub const KeyboardKey_KEY_I: KeyboardKey = 73;
pub const KeyboardKey_KEY_J: KeyboardKey = 74;
pub const KeyboardKey_KEY_K: KeyboardKey = 75;
pub const KeyboardKey_KEY_L: KeyboardKey = 76;
pub const KeyboardKey_KEY_M: KeyboardKey = 77;
pub const KeyboardKey_KEY_N: KeyboardKey = 78;
pub const KeyboardKey_KEY_O: KeyboardKey = 79;
pub const KeyboardKey_KEY_P: KeyboardKey = 80;
pub const KeyboardKey_KEY_Q: KeyboardKey = 81;
pub const KeyboardKey_KEY_R: KeyboardKey = 82;
pub const KeyboardKey_KEY_S: KeyboardKey = 83;
pub const KeyboardKey_KEY_T: KeyboardKey = 84;
pub const KeyboardKey_KEY_U: KeyboardKey = 85;
pub const KeyboardKey_KEY_V: KeyboardKey = 86;
pub const KeyboardKey_KEY_W: KeyboardKey = 87;
pub const KeyboardKey_KEY_X: KeyboardKey = 88;
pub const KeyboardKey_KEY_Y: KeyboardKey = 89;
pub const KeyboardKey_KEY_Z: KeyboardKey = 90;
pub const KeyboardKey_KEY_LEFT_BRACKET: KeyboardKey = 91;
pub const KeyboardKey_KEY_BACKSLASH: KeyboardKey = 92;
pub const KeyboardKey_KEY_RIGHT_BRACKET: KeyboardKey = 93;
pub const KeyboardKey_KEY_GRAVE: KeyboardKey = 96;
pub const KeyboardKey_KEY_SPACE: KeyboardKey = 32;
pub const KeyboardKey_KEY_ESCAPE: KeyboardKey = 256;
pub const KeyboardKey_KEY_ENTER: KeyboardKey = 257;
pub const KeyboardKey_KEY_TAB: KeyboardKey = 258;
pub const KeyboardKey_KEY_BACKSPACE: KeyboardKey = 259;
pub const KeyboardKey_KEY_INSERT: KeyboardKey = 260;
pub const KeyboardKey_KEY_DELETE: KeyboardKey = 261;
pub const KeyboardKey_KEY_RIGHT: KeyboardKey = 262;
pub const KeyboardKey_KEY_LEFT: KeyboardKey = 263;
pub const KeyboardKey_KEY_DOWN: KeyboardKey = 264;
pub const KeyboardKey_KEY_UP: KeyboardKey = 265;
pub const KeyboardKey_KEY_PAGE_UP: KeyboardKey = 266;
pub const KeyboardKey_KEY_PAGE_DOWN: KeyboardKey = 267;
pub const KeyboardKey_KEY_HOME: KeyboardKey = 268;
pub const KeyboardKey_KEY_END: KeyboardKey = 269;
pub const KeyboardKey_KEY_CAPS_LOCK: KeyboardKey = 280;
pub const KeyboardKey_KEY_SCROLL_LOCK: KeyboardKey = 281;
pub const KeyboardKey_KEY_NUM_LOCK: KeyboardKey = 282;
pub const KeyboardKey_KEY_PRINT_SCREEN: KeyboardKey = 283;
pub const KeyboardKey_KEY_PAUSE: KeyboardKey = 284;
pub const KeyboardKey_KEY_F1: KeyboardKey = 290;
pub const KeyboardKey_KEY_F2: KeyboardKey = 291;
pub const KeyboardKey_KEY_F3: KeyboardKey = 292;
pub const KeyboardKey_KEY_F4: KeyboardKey = 293;
pub const KeyboardKey_KEY_F5: KeyboardKey = 294;
pub const KeyboardKey_KEY_F6: KeyboardKey = 295;
pub const KeyboardKey_KEY_F7: KeyboardKey = 296;
pub const KeyboardKey_KEY_F8: KeyboardKey = 297;
pub const KeyboardKey_KEY_F9: KeyboardKey = 298;
pub const KeyboardKey_KEY_F10: KeyboardKey = 299;
pub const KeyboardKey_KEY_F11: KeyboardKey = 300;
pub const KeyboardKey_KEY_F12: KeyboardKey = 301;
pub const KeyboardKey_KEY_LEFT_SHIFT: KeyboardKey = 340;
pub const KeyboardKey_KEY_LEFT_CONTROL: KeyboardKey = 341;
pub const KeyboardKey_KEY_LEFT_ALT: KeyboardKey = 342;
pub const KeyboardKey_KEY_LEFT_SUPER: KeyboardKey = 343;
pub const KeyboardKey_KEY_RIGHT_SHIFT: KeyboardKey = 344;
pub const KeyboardKey_KEY_RIGHT_CONTROL: KeyboardKey = 345;
pub const KeyboardKey_KEY_RIGHT_ALT: KeyboardKey = 346;
pub const KeyboardKey_KEY_RIGHT_SUPER: KeyboardKey = 347;
pub const KeyboardKey_KEY_KB_MENU: KeyboardKey = 348;
pub const KeyboardKey_KEY_KP_0: KeyboardKey = 320;
pub const KeyboardKey_KEY_KP_1: KeyboardKey = 321;
pub const KeyboardKey_KEY_KP_2: KeyboardKey = 322;
pub const KeyboardKey_KEY_KP_3: KeyboardKey = 323;
pub const KeyboardKey_KEY_KP_4: KeyboardKey = 324;
pub const KeyboardKey_KEY_KP_5: KeyboardKey = 325;
pub const KeyboardKey_KEY_KP_6: KeyboardKey = 326;
pub const KeyboardKey_KEY_KP_7: KeyboardKey = 327;
pub const KeyboardKey_KEY_KP_8: KeyboardKey = 328;
pub const KeyboardKey_KEY_KP_9: KeyboardKey = 329;
pub const KeyboardKey_KEY_KP_DECIMAL: KeyboardKey = 330;
pub const KeyboardKey_KEY_KP_DIVIDE: KeyboardKey = 331;
pub const KeyboardKey_KEY_KP_MULTIPLY: KeyboardKey = 332;
pub const KeyboardKey_KEY_KP_SUBTRACT: KeyboardKey = 333;
pub const KeyboardKey_KEY_KP_ADD: KeyboardKey = 334;
pub const KeyboardKey_KEY_KP_ENTER: KeyboardKey = 335;
pub const KeyboardKey_KEY_KP_EQUAL: KeyboardKey = 336;
pub const KeyboardKey_KEY_BACK: KeyboardKey = 4;
pub const KeyboardKey_KEY_MENU: KeyboardKey = 5;
pub const KeyboardKey_KEY_VOLUME_UP: KeyboardKey = 24;
pub const KeyboardKey_KEY_VOLUME_DOWN: KeyboardKey = 25;
pub type KeyboardKey = ::std::os::raw::c_uint;
pub const MouseButton_MOUSE_BUTTON_LEFT: MouseButton = 0;
pub const MouseButton_MOUSE_BUTTON_RIGHT: MouseButton = 1;
pub const MouseButton_MOUSE_BUTTON_MIDDLE: MouseButton = 2;
pub const MouseButton_MOUSE_BUTTON_SIDE: MouseButton = 3;
pub const MouseButton_MOUSE_BUTTON_EXTRA: MouseButton = 4;
pub const MouseButton_MOUSE_BUTTON_FORWARD: MouseButton = 5;
pub const MouseButton_MOUSE_BUTTON_BACK: MouseButton = 6;
pub type MouseButton = ::std::os::raw::c_uint;
pub const MouseCursor_MOUSE_CURSOR_DEFAULT: MouseCursor = 0;
pub const MouseCursor_MOUSE_CURSOR_ARROW: MouseCursor = 1;
pub const MouseCursor_MOUSE_CURSOR_IBEAM: MouseCursor = 2;
pub const MouseCursor_MOUSE_CURSOR_CROSSHAIR: MouseCursor = 3;
pub const MouseCursor_MOUSE_CURSOR_POINTING_HAND: MouseCursor = 4;
pub const MouseCursor_MOUSE_CURSOR_RESIZE_EW: MouseCursor = 5;
pub const MouseCursor_MOUSE_CURSOR_RESIZE_NS: MouseCursor = 6;
pub const MouseCursor_MOUSE_CURSOR_RESIZE_NWSE: MouseCursor = 7;
pub const MouseCursor_MOUSE_CURSOR_RESIZE_NESW: MouseCursor = 8;
pub const MouseCursor_MOUSE_CURSOR_RESIZE_ALL: MouseCursor = 9;
pub const MouseCursor_MOUSE_CURSOR_NOT_ALLOWED: MouseCursor = 10;
pub type MouseCursor = ::std::os::raw::c_uint;
pub const GamepadButton_GAMEPAD_BUTTON_UNKNOWN: GamepadButton = 0;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_FACE_UP: GamepadButton = 1;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_FACE_RIGHT: GamepadButton = 2;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_FACE_DOWN: GamepadButton = 3;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_FACE_LEFT: GamepadButton = 4;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_FACE_UP: GamepadButton = 5;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_FACE_RIGHT: GamepadButton = 6;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_FACE_DOWN: GamepadButton = 7;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_FACE_LEFT: GamepadButton = 8;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_TRIGGER_1: GamepadButton = 9;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_TRIGGER_2: GamepadButton = 10;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_TRIGGER_1: GamepadButton = 11;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_TRIGGER_2: GamepadButton = 12;
pub const GamepadButton_GAMEPAD_BUTTON_MIDDLE_LEFT: GamepadButton = 13;
pub const GamepadButton_GAMEPAD_BUTTON_MIDDLE: GamepadButton = 14;
pub const GamepadButton_GAMEPAD_BUTTON_MIDDLE_RIGHT: GamepadButton = 15;
pub const GamepadButton_GAMEPAD_BUTTON_LEFT_THUMB: GamepadButton = 16;
pub const GamepadButton_GAMEPAD_BUTTON_RIGHT_THUMB: GamepadButton = 17;
pub type GamepadButton = ::std::os::raw::c_uint;
pub const GamepadAxis_GAMEPAD_AXIS_LEFT_X: GamepadAxis = 0;
pub const GamepadAxis_GAMEPAD_AXIS_LEFT_Y: GamepadAxis = 1;
pub const GamepadAxis_GAMEPAD_AXIS_RIGHT_X: GamepadAxis = 2;
pub const GamepadAxis_GAMEPAD_AXIS_RIGHT_Y: GamepadAxis = 3;
pub const GamepadAxis_GAMEPAD_AXIS_LEFT_TRIGGER: GamepadAxis = 4;
pub const GamepadAxis_GAMEPAD_AXIS_RIGHT_TRIGGER: GamepadAxis = 5;
pub type GamepadAxis = ::std::os::raw::c_uint;
pub const MaterialMapIndex_MATERIAL_MAP_ALBEDO: MaterialMapIndex = 0;
pub const MaterialMapIndex_MATERIAL_MAP_METALNESS: MaterialMapIndex = 1;
pub const MaterialMapIndex_MATERIAL_MAP_NORMAL: MaterialMapIndex = 2;
pub const MaterialMapIndex_MATERIAL_MAP_ROUGHNESS: MaterialMapIndex = 3;
pub const MaterialMapIndex_MATERIAL_MAP_OCCLUSION: MaterialMapIndex = 4;
pub const MaterialMapIndex_MATERIAL_MAP_EMISSION: MaterialMapIndex = 5;
pub const MaterialMapIndex_MATERIAL_MAP_HEIGHT: MaterialMapIndex = 6;
pub const MaterialMapIndex_MATERIAL_MAP_CUBEMAP: MaterialMapIndex = 7;
pub const MaterialMapIndex_MATERIAL_MAP_IRRADIANCE: MaterialMapIndex = 8;
pub const MaterialMapIndex_MATERIAL_MAP_PREFILTER: MaterialMapIndex = 9;
pub const MaterialMapIndex_MATERIAL_MAP_BRDF: MaterialMapIndex = 10;
pub type MaterialMapIndex = ::std::os::raw::c_uint;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_POSITION: ShaderLocationIndex = 0;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_TEXCOORD01: ShaderLocationIndex = 1;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_TEXCOORD02: ShaderLocationIndex = 2;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_NORMAL: ShaderLocationIndex = 3;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_TANGENT: ShaderLocationIndex = 4;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_COLOR: ShaderLocationIndex = 5;
pub const ShaderLocationIndex_SHADER_LOC_MATRIX_MVP: ShaderLocationIndex = 6;
pub const ShaderLocationIndex_SHADER_LOC_MATRIX_VIEW: ShaderLocationIndex = 7;
pub const ShaderLocationIndex_SHADER_LOC_MATRIX_PROJECTION: ShaderLocationIndex = 8;
pub const ShaderLocationIndex_SHADER_LOC_MATRIX_MODEL: ShaderLocationIndex = 9;
pub const ShaderLocationIndex_SHADER_LOC_MATRIX_NORMAL: ShaderLocationIndex = 10;
pub const ShaderLocationIndex_SHADER_LOC_VECTOR_VIEW: ShaderLocationIndex = 11;
pub const ShaderLocationIndex_SHADER_LOC_COLOR_DIFFUSE: ShaderLocationIndex = 12;
pub const ShaderLocationIndex_SHADER_LOC_COLOR_SPECULAR: ShaderLocationIndex = 13;
pub const ShaderLocationIndex_SHADER_LOC_COLOR_AMBIENT: ShaderLocationIndex = 14;
pub const ShaderLocationIndex_SHADER_LOC_MAP_ALBEDO: ShaderLocationIndex = 15;
pub const ShaderLocationIndex_SHADER_LOC_MAP_METALNESS: ShaderLocationIndex = 16;
pub const ShaderLocationIndex_SHADER_LOC_MAP_NORMAL: ShaderLocationIndex = 17;
pub const ShaderLocationIndex_SHADER_LOC_MAP_ROUGHNESS: ShaderLocationIndex = 18;
pub const ShaderLocationIndex_SHADER_LOC_MAP_OCCLUSION: ShaderLocationIndex = 19;
pub const ShaderLocationIndex_SHADER_LOC_MAP_EMISSION: ShaderLocationIndex = 20;
pub const ShaderLocationIndex_SHADER_LOC_MAP_HEIGHT: ShaderLocationIndex = 21;
pub const ShaderLocationIndex_SHADER_LOC_MAP_CUBEMAP: ShaderLocationIndex = 22;
pub const ShaderLocationIndex_SHADER_LOC_MAP_IRRADIANCE: ShaderLocationIndex = 23;
pub const ShaderLocationIndex_SHADER_LOC_MAP_PREFILTER: ShaderLocationIndex = 24;
pub const ShaderLocationIndex_SHADER_LOC_MAP_BRDF: ShaderLocationIndex = 25;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_BONEIDS: ShaderLocationIndex = 26;
pub const ShaderLocationIndex_SHADER_LOC_VERTEX_BONEWEIGHTS: ShaderLocationIndex = 27;
pub const ShaderLocationIndex_SHADER_LOC_BONE_MATRICES: ShaderLocationIndex = 28;
pub type ShaderLocationIndex = ::std::os::raw::c_uint;
pub const ShaderUniformDataType_SHADER_UNIFORM_FLOAT: ShaderUniformDataType = 0;
pub const ShaderUniformDataType_SHADER_UNIFORM_VEC2: ShaderUniformDataType = 1;
pub const ShaderUniformDataType_SHADER_UNIFORM_VEC3: ShaderUniformDataType = 2;
pub const ShaderUniformDataType_SHADER_UNIFORM_VEC4: ShaderUniformDataType = 3;
pub const ShaderUniformDataType_SHADER_UNIFORM_INT: ShaderUniformDataType = 4;
pub const ShaderUniformDataType_SHADER_UNIFORM_IVEC2: ShaderUniformDataType = 5;
pub const ShaderUniformDataType_SHADER_UNIFORM_IVEC3: ShaderUniformDataType = 6;
pub const ShaderUniformDataType_SHADER_UNIFORM_IVEC4: ShaderUniformDataType = 7;
pub const ShaderUniformDataType_SHADER_UNIFORM_SAMPLER2D: ShaderUniformDataType = 8;
pub type ShaderUniformDataType = ::std::os::raw::c_uint;
pub const ShaderAttributeDataType_SHADER_ATTRIB_FLOAT: ShaderAttributeDataType = 0;
pub const ShaderAttributeDataType_SHADER_ATTRIB_VEC2: ShaderAttributeDataType = 1;
pub const ShaderAttributeDataType_SHADER_ATTRIB_VEC3: ShaderAttributeDataType = 2;
pub const ShaderAttributeDataType_SHADER_ATTRIB_VEC4: ShaderAttributeDataType = 3;
pub type ShaderAttributeDataType = ::std::os::raw::c_uint;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_GRAYSCALE: PixelFormat = 1;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA: PixelFormat = 2;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R5G6B5: PixelFormat = 3;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R8G8B8: PixelFormat = 4;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R5G5B5A1: PixelFormat = 5;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R4G4B4A4: PixelFormat = 6;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8: PixelFormat = 7;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R32: PixelFormat = 8;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R32G32B32: PixelFormat = 9;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R32G32B32A32: PixelFormat = 10;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R16: PixelFormat = 11;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R16G16B16: PixelFormat = 12;
pub const PixelFormat_PIXELFORMAT_UNCOMPRESSED_R16G16B16A16: PixelFormat = 13;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_DXT1_RGB: PixelFormat = 14;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_DXT1_RGBA: PixelFormat = 15;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_DXT3_RGBA: PixelFormat = 16;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_DXT5_RGBA: PixelFormat = 17;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_ETC1_RGB: PixelFormat = 18;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_ETC2_RGB: PixelFormat = 19;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA: PixelFormat = 20;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_PVRT_RGB: PixelFormat = 21;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_PVRT_RGBA: PixelFormat = 22;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA: PixelFormat = 23;
pub const PixelFormat_PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA: PixelFormat = 24;
pub type PixelFormat = ::std::os::raw::c_uint;
pub const TextureFilter_TEXTURE_FILTER_POINT: TextureFilter = 0;
pub const TextureFilter_TEXTURE_FILTER_BILINEAR: TextureFilter = 1;
pub const TextureFilter_TEXTURE_FILTER_TRILINEAR: TextureFilter = 2;
pub const TextureFilter_TEXTURE_FILTER_ANISOTROPIC_4X: TextureFilter = 3;
pub const TextureFilter_TEXTURE_FILTER_ANISOTROPIC_8X: TextureFilter = 4;
pub const TextureFilter_TEXTURE_FILTER_ANISOTROPIC_16X: TextureFilter = 5;
pub type TextureFilter = ::std::os::raw::c_uint;
pub const TextureWrap_TEXTURE_WRAP_REPEAT: TextureWrap = 0;
pub const TextureWrap_TEXTURE_WRAP_CLAMP: TextureWrap = 1;
pub const TextureWrap_TEXTURE_WRAP_MIRROR_REPEAT: TextureWrap = 2;
pub const TextureWrap_TEXTURE_WRAP_MIRROR_CLAMP: TextureWrap = 3;
pub type TextureWrap = ::std::os::raw::c_uint;
pub const CubemapLayout_CUBEMAP_LAYOUT_AUTO_DETECT: CubemapLayout = 0;
pub const CubemapLayout_CUBEMAP_LAYOUT_LINE_VERTICAL: CubemapLayout = 1;
pub const CubemapLayout_CUBEMAP_LAYOUT_LINE_HORIZONTAL: CubemapLayout = 2;
pub const CubemapLayout_CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR: CubemapLayout = 3;
pub const CubemapLayout_CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE: CubemapLayout = 4;
pub const CubemapLayout_CUBEMAP_LAYOUT_PANORAMA: CubemapLayout = 5;
pub type CubemapLayout = ::std::os::raw::c_uint;
pub const FontType_FONT_DEFAULT: FontType = 0;
pub const FontType_FONT_BITMAP: FontType = 1;
pub const FontType_FONT_SDF: FontType = 2;
pub type FontType = ::std::os::raw::c_uint;
pub const BlendMode_BLEND_ALPHA: BlendMode = 0;
pub const BlendMode_BLEND_ADDITIVE: BlendMode = 1;
pub const BlendMode_BLEND_MULTIPLIED: BlendMode = 2;
pub const BlendMode_BLEND_ADD_COLORS: BlendMode = 3;
pub const BlendMode_BLEND_SUBTRACT_COLORS: BlendMode = 4;
pub const BlendMode_BLEND_ALPHA_PREMULTIPLY: BlendMode = 5;
pub const BlendMode_BLEND_CUSTOM: BlendMode = 6;
pub const BlendMode_BLEND_CUSTOM_SEPARATE: BlendMode = 7;
pub type BlendMode = ::std::os::raw::c_uint;
pub const Gesture_GESTURE_NONE: Gesture = 0;
pub const Gesture_GESTURE_TAP: Gesture = 1;
pub const Gesture_GESTURE_DOUBLETAP: Gesture = 2;
pub const Gesture_GESTURE_HOLD: Gesture = 4;
pub const Gesture_GESTURE_DRAG: Gesture = 8;
pub const Gesture_GESTURE_SWIPE_RIGHT: Gesture = 16;
pub const Gesture_GESTURE_SWIPE_LEFT: Gesture = 32;
pub const Gesture_GESTURE_SWIPE_UP: Gesture = 64;
pub const Gesture_GESTURE_SWIPE_DOWN: Gesture = 128;
pub const Gesture_GESTURE_PINCH_IN: Gesture = 256;
pub const Gesture_GESTURE_PINCH_OUT: Gesture = 512;
pub type Gesture = ::std::os::raw::c_uint;
pub const CameraMode_CAMERA_CUSTOM: CameraMode = 0;
pub const CameraMode_CAMERA_FREE: CameraMode = 1;
pub const CameraMode_CAMERA_ORBITAL: CameraMode = 2;
pub const CameraMode_CAMERA_FIRST_PERSON: CameraMode = 3;
pub const CameraMode_CAMERA_THIRD_PERSON: CameraMode = 4;
pub type CameraMode = ::std::os::raw::c_uint;
pub const CameraProjection_CAMERA_PERSPECTIVE: CameraProjection = 0;
pub const CameraProjection_CAMERA_ORTHOGRAPHIC: CameraProjection = 1;
pub type CameraProjection = ::std::os::raw::c_uint;
pub const NPatchLayout_NPATCH_NINE_PATCH: NPatchLayout = 0;
pub const NPatchLayout_NPATCH_THREE_PATCH_VERTICAL: NPatchLayout = 1;
pub const NPatchLayout_NPATCH_THREE_PATCH_HORIZONTAL: NPatchLayout = 2;
pub type NPatchLayout = ::std::os::raw::c_uint;
pub type TraceLogCallback = ::std::option::Option<
    unsafe extern "C" fn(
        logLevel: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ),
>;
pub type LoadFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar,
>;
pub type SaveFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
    ) -> bool,
>;
pub type LoadFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type SaveFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool,
>;
extern "C" {
    pub fn InitWindow(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn CloseWindow();
}
extern "C" {
    pub fn WindowShouldClose() -> bool;
}
extern "C" {
    pub fn IsWindowReady() -> bool;
}
extern "C" {
    pub fn IsWindowFullscreen() -> bool;
}
extern "C" {
    pub fn IsWindowHidden() -> bool;
}
extern "C" {
    pub fn IsWindowMinimized() -> bool;
}
extern "C" {
    pub fn IsWindowMaximized() -> bool;
}
extern "C" {
    pub fn IsWindowFocused() -> bool;
}
extern "C" {
    pub fn IsWindowResized() -> bool;
}
extern "C" {
    pub fn IsWindowState(flag: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn SetWindowState(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ClearWindowState(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ToggleFullscreen();
}
extern "C" {
    pub fn ToggleBorderlessWindowed();
}
extern "C" {
    pub fn MaximizeWindow();
}
extern "C" {
    pub fn MinimizeWindow();
}
extern "C" {
    pub fn RestoreWindow();
}
extern "C" {
    pub fn SetWindowIcon(image: Image);
}
extern "C" {
    pub fn SetWindowIcons(images: *mut Image, count: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowTitle(title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetWindowPosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMonitor(monitor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMinSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMaxSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowOpacity(opacity: f32);
}
extern "C" {
    pub fn SetWindowFocused();
}
extern "C" {
    pub fn GetWindowHandle() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GetScreenWidth() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetScreenHeight() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetRenderWidth() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetRenderHeight() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCurrentMonitor() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPosition(monitor: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn GetMonitorWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPhysicalWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPhysicalHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorRefreshRate(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetWindowPosition() -> Vector2;
}
extern "C" {
    pub fn GetWindowScaleDPI() -> Vector2;
}
extern "C" {
    pub fn GetMonitorName(monitor: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetClipboardText(text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GetClipboardText() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn EnableEventWaiting();
}
extern "C" {
    pub fn DisableEventWaiting();
}
extern "C" {
    pub fn ShowCursor();
}
extern "C" {
    pub fn HideCursor();
}
extern "C" {
    pub fn IsCursorHidden() -> bool;
}
extern "C" {
    pub fn EnableCursor();
}
extern "C" {
    pub fn DisableCursor();
}
extern "C" {
    pub fn IsCursorOnScreen() -> bool;
}
extern "C" {
    pub fn ClearBackground(color: Color);
}
extern "C" {
    pub fn BeginDrawing();
}
extern "C" {
    pub fn EndDrawing();
}
extern "C" {
    pub fn BeginMode2D(camera: Camera2D);
}
extern "C" {
    pub fn EndMode2D();
}
extern "C" {
    pub fn BeginMode3D(camera: Camera3D);
}
extern "C" {
    pub fn EndMode3D();
}
extern "C" {
    pub fn BeginTextureMode(target: RenderTexture2D);
}
extern "C" {
    pub fn EndTextureMode();
}
extern "C" {
    pub fn BeginShaderMode(shader: Shader);
}
extern "C" {
    pub fn EndShaderMode();
}
extern "C" {
    pub fn BeginBlendMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn EndBlendMode();
}
extern "C" {
    pub fn BeginScissorMode(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn EndScissorMode();
}
extern "C" {
    pub fn BeginVrStereoMode(config: VrStereoConfig);
}
extern "C" {
    pub fn EndVrStereoMode();
}
extern "C" {
    pub fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;
}
extern "C" {
    pub fn UnloadVrStereoConfig(config: VrStereoConfig);
}
extern "C" {
    pub fn LoadShader(
        vsFileName: *const ::std::os::raw::c_char,
        fsFileName: *const ::std::os::raw::c_char,
    ) -> Shader;
}
extern "C" {
    pub fn LoadShaderFromMemory(
        vsCode: *const ::std::os::raw::c_char,
        fsCode: *const ::std::os::raw::c_char,
    ) -> Shader;
}
extern "C" {
    pub fn IsShaderReady(shader: Shader) -> bool;
}
extern "C" {
    pub fn GetShaderLocation(
        shader: Shader,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetShaderLocationAttrib(
        shader: Shader,
        attribName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetShaderValue(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShaderValueV(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShaderValueMatrix(shader: Shader, locIndex: ::std::os::raw::c_int, mat: Matrix);
}
extern "C" {
    pub fn SetShaderValueTexture(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
extern "C" {
    pub fn UnloadShader(shader: Shader);
}
extern "C" {
    pub fn GetScreenToWorldRay(position: Vector2, camera: Camera) -> Ray;
}
extern "C" {
    pub fn GetScreenToWorldRayEx(
        position: Vector2,
        camera: Camera,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Ray;
}
extern "C" {
    pub fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2;
}
extern "C" {
    pub fn GetWorldToScreenEx(
        position: Vector3,
        camera: Camera,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Vector2;
}
extern "C" {
    pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2;
}
extern "C" {
    pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;
}
extern "C" {
    pub fn GetCameraMatrix(camera: Camera) -> Matrix;
}
extern "C" {
    pub fn GetCameraMatrix2D(camera: Camera2D) -> Matrix;
}
extern "C" {
    pub fn SetTargetFPS(fps: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetFrameTime() -> f32;
}
extern "C" {
    pub fn GetTime() -> f64;
}
extern "C" {
    pub fn GetFPS() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SwapScreenBuffer();
}
extern "C" {
    pub fn PollInputEvents();
}
extern "C" {
    pub fn WaitTime(seconds: f64);
}
extern "C" {
    pub fn SetRandomSeed(seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn GetRandomValue(
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LoadRandomSequence(
        count: ::std::os::raw::c_uint,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn UnloadRandomSequence(sequence: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn TakeScreenshot(fileName: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetConfigFlags(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn OpenURL(url: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn TraceLog(logLevel: ::std::os::raw::c_int, text: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn SetTraceLogLevel(logLevel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn MemAlloc(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn MemRealloc(
        ptr: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn MemFree(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn SetTraceLogCallback(callback: TraceLogCallback);
}
extern "C" {
    pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback);
}
extern "C" {
    pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback);
}
extern "C" {
    pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback);
}
extern "C" {
    pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback);
}
extern "C" {
    pub fn LoadFileData(
        fileName: *const ::std::os::raw::c_char,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn UnloadFileData(data: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn SaveFileData(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn ExportDataAsCode(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fileName: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn LoadFileText(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn UnloadFileText(text: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn SaveFileText(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn FileExists(fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn DirectoryExists(dirPath: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsFileExtension(
        fileName: *const ::std::os::raw::c_char,
        ext: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn GetFileLength(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetFileExtension(
        fileName: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetFileName(filePath: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetFileNameWithoutExt(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetDirectoryPath(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetPrevDirectoryPath(
        dirPath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetWorkingDirectory() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetApplicationDirectory() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn MakeDirectory(dirPath: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ChangeDirectory(dir: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsPathFile(path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsFileNameValid(fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn LoadDirectoryFiles(dirPath: *const ::std::os::raw::c_char) -> FilePathList;
}
extern "C" {
    pub fn LoadDirectoryFilesEx(
        basePath: *const ::std::os::raw::c_char,
        filter: *const ::std::os::raw::c_char,
        scanSubdirs: bool,
    ) -> FilePathList;
}
extern "C" {
    pub fn UnloadDirectoryFiles(files: FilePathList);
}
extern "C" {
    pub fn IsFileDropped() -> bool;
}
extern "C" {
    pub fn LoadDroppedFiles() -> FilePathList;
}
extern "C" {
    pub fn UnloadDroppedFiles(files: FilePathList);
}
extern "C" {
    pub fn GetFileModTime(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn CompressData(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        compDataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn DecompressData(
        compData: *const ::std::os::raw::c_uchar,
        compDataSize: ::std::os::raw::c_int,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn EncodeDataBase64(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        outputSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DecodeDataBase64(
        data: *const ::std::os::raw::c_uchar,
        outputSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn LoadAutomationEventList(fileName: *const ::std::os::raw::c_char) -> AutomationEventList;
}
extern "C" {
    pub fn UnloadAutomationEventList(list: AutomationEventList);
}
extern "C" {
    pub fn ExportAutomationEventList(
        list: AutomationEventList,
        fileName: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn SetAutomationEventList(list: *mut AutomationEventList);
}
extern "C" {
    pub fn SetAutomationEventBaseFrame(frame: ::std::os::raw::c_int);
}
extern "C" {
    pub fn StartAutomationEventRecording();
}
extern "C" {
    pub fn StopAutomationEventRecording();
}
extern "C" {
    pub fn PlayAutomationEvent(event: AutomationEvent);
}
extern "C" {
    pub fn IsKeyPressed(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyPressedRepeat(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyDown(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyReleased(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsKeyUp(key: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetKeyPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCharPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetExitKey(key: ::std::os::raw::c_int);
}
extern "C" {
    pub fn IsGamepadAvailable(gamepad: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetGamepadName(gamepad: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn IsGamepadButtonPressed(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonDown(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonReleased(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonUp(gamepad: ::std::os::raw::c_int, button: ::std::os::raw::c_int)
        -> bool;
}
extern "C" {
    pub fn GetGamepadButtonPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGamepadAxisCount(gamepad: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGamepadAxisMovement(
        gamepad: ::std::os::raw::c_int,
        axis: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn SetGamepadMappings(mappings: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetGamepadVibration(gamepad: ::std::os::raw::c_int, leftMotor: f32, rightMotor: f32);
}
extern "C" {
    pub fn IsMouseButtonPressed(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonDown(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonReleased(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonUp(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetMouseX() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMouseY() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMousePosition() -> Vector2;
}
extern "C" {
    pub fn GetMouseDelta() -> Vector2;
}
extern "C" {
    pub fn SetMousePosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetMouseOffset(offsetX: ::std::os::raw::c_int, offsetY: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetMouseScale(scaleX: f32, scaleY: f32);
}
extern "C" {
    pub fn GetMouseWheelMove() -> f32;
}
extern "C" {
    pub fn GetMouseWheelMoveV() -> Vector2;
}
extern "C" {
    pub fn SetMouseCursor(cursor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetTouchX() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchY() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchPosition(index: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn GetTouchPointId(index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchPointCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetGesturesEnabled(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn IsGestureDetected(gesture: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn GetGestureDetected() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGestureHoldDuration() -> f32;
}
extern "C" {
    pub fn GetGestureDragVector() -> Vector2;
}
extern "C" {
    pub fn GetGestureDragAngle() -> f32;
}
extern "C" {
    pub fn GetGesturePinchVector() -> Vector2;
}
extern "C" {
    pub fn GetGesturePinchAngle() -> f32;
}
extern "C" {
    pub fn UpdateCamera(camera: *mut Camera, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: f32);
}
extern "C" {
    pub fn SetShapesTexture(texture: Texture2D, source: Rectangle);
}
extern "C" {
    pub fn GetShapesTexture() -> Texture2D;
}
extern "C" {
    pub fn GetShapesTextureRectangle() -> Rectangle;
}
extern "C" {
    pub fn DrawPixel(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawPixelV(position: Vector2, color: Color);
}
extern "C" {
    pub fn DrawLine(
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
}
extern "C" {
    pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawLineStrip(points: *const Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawCircle(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleSector(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleSectorLines(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleGradient(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        inner: Color,
        outer: Color,
    );
}
extern "C" {
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawCircleLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawEllipse(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawEllipseLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRing(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRingLines(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangle(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
}
extern "C" {
    pub fn DrawRectangleRec(rec: Rectangle, color: Color);
}
extern "C" {
    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
}
extern "C" {
    pub fn DrawRectangleGradientV(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        top: Color,
        bottom: Color,
    );
}
extern "C" {
    pub fn DrawRectangleGradientH(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        left: Color,
        right: Color,
    );
}
extern "C" {
    pub fn DrawRectangleGradientEx(
        rec: Rectangle,
        topLeft: Color,
        bottomLeft: Color,
        topRight: Color,
        bottomRight: Color,
    );
}
extern "C" {
    pub fn DrawRectangleLines(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: f32, color: Color);
}
extern "C" {
    pub fn DrawRectangleRounded(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleRoundedLines(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleRoundedLinesEx(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        lineThick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn DrawTriangleFan(points: *const Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawTriangleStrip(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPoly(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPolyLines(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPolyLinesEx(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        lineThick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineLinear(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineBasis(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineCatmullRom(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineBezierQuadratic(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineBezierCubic(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawSplineSegmentBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentBezierQuadratic(
        p1: Vector2,
        c2: Vector2,
        p3: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointBezierQuad(p1: Vector2, c2: Vector2, p3: Vector2, t: f32) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointTriangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointPoly(
        point: Vector2,
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionLines(
        startPos1: Vector2,
        endPos1: Vector2,
        startPos2: Vector2,
        endPos2: Vector2,
        collisionPoint: *mut Vector2,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointLine(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircleLine(center: Vector2, radius: f32, p1: Vector2, p2: Vector2)
        -> bool;
}
extern "C" {
    pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
}
extern "C" {
    pub fn LoadImage(fileName: *const ::std::os::raw::c_char) -> Image;
}
extern "C" {
    pub fn LoadImageRaw(
        fileName: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        headerSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageSvg(
        fileNameOrString: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageAnim(
        fileName: *const ::std::os::raw::c_char,
        frames: *mut ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageAnimFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        frames: *mut ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageFromTexture(texture: Texture2D) -> Image;
}
extern "C" {
    pub fn LoadImageFromScreen() -> Image;
}
extern "C" {
    pub fn IsImageReady(image: Image) -> bool;
}
extern "C" {
    pub fn UnloadImage(image: Image);
}
extern "C" {
    pub fn ExportImage(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportImageToMemory(
        image: Image,
        fileType: *const ::std::os::raw::c_char,
        fileSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn ExportImageAsCode(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GenImageColor(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientLinear(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
        start: Color,
        end: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientRadial(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientSquare(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageChecked(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        checksX: ::std::os::raw::c_int,
        checksY: ::std::os::raw::c_int,
        col1: Color,
        col2: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageWhiteNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        factor: f32,
    ) -> Image;
}
extern "C" {
    pub fn GenImagePerlinNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        scale: f32,
    ) -> Image;
}
extern "C" {
    pub fn GenImageCellular(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        tileSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn GenImageText(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    ) -> Image;
}
extern "C" {
    pub fn ImageCopy(image: Image) -> Image;
}
extern "C" {
    pub fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
}
extern "C" {
    pub fn ImageFromChannel(image: Image, selectedChannel: ::std::os::raw::c_int) -> Image;
}
extern "C" {
    pub fn ImageText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
extern "C" {
    pub fn ImageTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    ) -> Image;
}
extern "C" {
    pub fn ImageFormat(image: *mut Image, newFormat: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageToPOT(image: *mut Image, fill: Color);
}
extern "C" {
    pub fn ImageCrop(image: *mut Image, crop: Rectangle);
}
extern "C" {
    pub fn ImageAlphaCrop(image: *mut Image, threshold: f32);
}
extern "C" {
    pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32);
}
extern "C" {
    pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
}
extern "C" {
    pub fn ImageAlphaPremultiply(image: *mut Image);
}
extern "C" {
    pub fn ImageBlurGaussian(image: *mut Image, blurSize: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageKernelConvolution(
        image: *mut Image,
        kernel: *const f32,
        kernelSize: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResize(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResizeNN(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResizeCanvas(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        fill: Color,
    );
}
extern "C" {
    pub fn ImageMipmaps(image: *mut Image);
}
extern "C" {
    pub fn ImageDither(
        image: *mut Image,
        rBpp: ::std::os::raw::c_int,
        gBpp: ::std::os::raw::c_int,
        bBpp: ::std::os::raw::c_int,
        aBpp: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageFlipVertical(image: *mut Image);
}
extern "C" {
    pub fn ImageFlipHorizontal(image: *mut Image);
}
extern "C" {
    pub fn ImageRotate(image: *mut Image, degrees: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageRotateCW(image: *mut Image);
}
extern "C" {
    pub fn ImageRotateCCW(image: *mut Image);
}
extern "C" {
    pub fn ImageColorTint(image: *mut Image, color: Color);
}
extern "C" {
    pub fn ImageColorInvert(image: *mut Image);
}
extern "C" {
    pub fn ImageColorGrayscale(image: *mut Image);
}
extern "C" {
    pub fn ImageColorContrast(image: *mut Image, contrast: f32);
}
extern "C" {
    pub fn ImageColorBrightness(image: *mut Image, brightness: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
}
extern "C" {
    pub fn LoadImageColors(image: Image) -> *mut Color;
}
extern "C" {
    pub fn LoadImagePalette(
        image: Image,
        maxPaletteSize: ::std::os::raw::c_int,
        colorCount: *mut ::std::os::raw::c_int,
    ) -> *mut Color;
}
extern "C" {
    pub fn UnloadImageColors(colors: *mut Color);
}
extern "C" {
    pub fn UnloadImagePalette(colors: *mut Color);
}
extern "C" {
    pub fn GetImageAlphaBorder(image: Image, threshold: f32) -> Rectangle;
}
extern "C" {
    pub fn GetImageColor(image: Image, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int)
        -> Color;
}
extern "C" {
    pub fn ImageClearBackground(dst: *mut Image, color: Color);
}
extern "C" {
    pub fn ImageDrawPixel(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawLine(
        dst: *mut Image,
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawLineEx(
        dst: *mut Image,
        start: Vector2,
        end: Vector2,
        thick: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircle(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleLines(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleLinesV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawRectangle(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
}
extern "C" {
    pub fn ImageDrawRectangleLines(
        dst: *mut Image,
        rec: Rectangle,
        thick: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawTriangleEx(
        dst: *mut Image,
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
        c1: Color,
        c2: Color,
        c3: Color,
    );
}
extern "C" {
    pub fn ImageDrawTriangleLines(
        dst: *mut Image,
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawTriangleFan(
        dst: *mut Image,
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawTriangleStrip(
        dst: *mut Image,
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDraw(
        dst: *mut Image,
        src: Image,
        srcRec: Rectangle,
        dstRec: Rectangle,
        tint: Color,
    );
}
extern "C" {
    pub fn ImageDrawText(
        dst: *mut Image,
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawTextEx(
        dst: *mut Image,
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn LoadTexture(fileName: *const ::std::os::raw::c_char) -> Texture2D;
}
extern "C" {
    pub fn LoadTextureFromImage(image: Image) -> Texture2D;
}
extern "C" {
    pub fn LoadTextureCubemap(image: Image, layout: ::std::os::raw::c_int) -> TextureCubemap;
}
extern "C" {
    pub fn LoadRenderTexture(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> RenderTexture2D;
}
extern "C" {
    pub fn IsTextureReady(texture: Texture2D) -> bool;
}
extern "C" {
    pub fn UnloadTexture(texture: Texture2D);
}
extern "C" {
    pub fn IsRenderTextureReady(target: RenderTexture2D) -> bool;
}
extern "C" {
    pub fn UnloadRenderTexture(target: RenderTexture2D);
}
extern "C" {
    pub fn UpdateTexture(texture: Texture2D, pixels: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn UpdateTextureRec(
        texture: Texture2D,
        rec: Rectangle,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GenTextureMipmaps(texture: *mut Texture2D);
}
extern "C" {
    pub fn SetTextureFilter(texture: Texture2D, filter: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetTextureWrap(texture: Texture2D, wrap: ::std::os::raw::c_int);
}
extern "C" {
    pub fn DrawTexture(
        texture: Texture2D,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
}
extern "C" {
    pub fn DrawTextureEx(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
}
extern "C" {
    pub fn DrawTexturePro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureNPatch(
        texture: Texture2D,
        nPatchInfo: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn ColorIsEqual(col1: Color, col2: Color) -> bool;
}
extern "C" {
    pub fn Fade(color: Color, alpha: f32) -> Color;
}
extern "C" {
    pub fn ColorToInt(color: Color) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ColorNormalize(color: Color) -> Vector4;
}
extern "C" {
    pub fn ColorFromNormalized(normalized: Vector4) -> Color;
}
extern "C" {
    pub fn ColorToHSV(color: Color) -> Vector3;
}
extern "C" {
    pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> Color;
}
extern "C" {
    pub fn ColorTint(color: Color, tint: Color) -> Color;
}
extern "C" {
    pub fn ColorBrightness(color: Color, factor: f32) -> Color;
}
extern "C" {
    pub fn ColorContrast(color: Color, contrast: f32) -> Color;
}
extern "C" {
    pub fn ColorAlpha(color: Color, alpha: f32) -> Color;
}
extern "C" {
    pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color;
}
extern "C" {
    pub fn ColorLerp(color1: Color, color2: Color, factor: f32) -> Color;
}
extern "C" {
    pub fn GetColor(hexValue: ::std::os::raw::c_uint) -> Color;
}
extern "C" {
    pub fn GetPixelColor(
        srcPtr: *mut ::std::os::raw::c_void,
        format: ::std::os::raw::c_int,
    ) -> Color;
}
extern "C" {
    pub fn SetPixelColor(
        dstPtr: *mut ::std::os::raw::c_void,
        color: Color,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GetPixelDataSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetFontDefault() -> Font;
}
extern "C" {
    pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;
}
extern "C" {
    pub fn LoadFontEx(
        fileName: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
    ) -> Font;
}
extern "C" {
    pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ::std::os::raw::c_int) -> Font;
}
extern "C" {
    pub fn LoadFontFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
    ) -> Font;
}
extern "C" {
    pub fn IsFontReady(font: Font) -> bool;
}
extern "C" {
    pub fn LoadFontData(
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
    ) -> *mut GlyphInfo;
}
extern "C" {
    pub fn GenImageFontAtlas(
        glyphs: *const GlyphInfo,
        glyphRecs: *mut *mut Rectangle,
        glyphCount: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        padding: ::std::os::raw::c_int,
        packMethod: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UnloadFont(font: Font);
}
extern "C" {
    pub fn ExportFontAsCode(font: Font, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);
}
extern "C" {
    pub fn DrawText(
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextPro(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        origin: Vector2,
        rotation: f32,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextCodepoint(
        font: Font,
        codepoint: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextCodepoints(
        font: Font,
        codepoints: *const ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn SetTextLineSpacing(spacing: ::std::os::raw::c_int);
}
extern "C" {
    pub fn MeasureText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MeasureTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetGlyphIndex(font: Font, codepoint: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGlyphInfo(font: Font, codepoint: ::std::os::raw::c_int) -> GlyphInfo;
}
extern "C" {
    pub fn GetGlyphAtlasRec(font: Font, codepoint: ::std::os::raw::c_int) -> Rectangle;
}
extern "C" {
    pub fn LoadUTF8(
        codepoints: *const ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn UnloadUTF8(text: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn LoadCodepoints(
        text: *const ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn UnloadCodepoints(codepoints: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetCodepointCount(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepoint(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepointNext(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepointPrevious(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CodepointToUTF8(
        codepoint: ::std::os::raw::c_int,
        utf8Size: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextCopy(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextIsEqual(
        text1: *const ::std::os::raw::c_char,
        text2: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn TextLength(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn TextFormat(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextSubtext(
        text: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextReplace(
        text: *const ::std::os::raw::c_char,
        replace: *const ::std::os::raw::c_char,
        by: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextInsert(
        text: *const ::std::os::raw::c_char,
        insert: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextJoin(
        textList: *mut *const ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        delimiter: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextSplit(
        text: *const ::std::os::raw::c_char,
        delimiter: ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextAppend(
        text: *mut ::std::os::raw::c_char,
        append: *const ::std::os::raw::c_char,
        position: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn TextFindIndex(
        text: *const ::std::os::raw::c_char,
        find: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextToUpper(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToLower(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToPascal(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToSnake(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToCamel(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToInteger(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextToFloat(text: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
}
extern "C" {
    pub fn DrawPoint3D(position: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCircle3D(
        center: Vector3,
        radius: f32,
        rotationAxis: Vector3,
        rotationAngle: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
}
extern "C" {
    pub fn DrawTriangleStrip3D(
        points: *const Vector3,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
extern "C" {
    pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
extern "C" {
    pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
}
extern "C" {
    pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawSphereEx(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSphereWires(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinder(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: f32,
        endRadius: f32,
        sides: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderWires(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderWiresEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: f32,
        endRadius: f32,
        sides: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCapsule(
        startPos: Vector3,
        endPos: Vector3,
        radius: f32,
        slices: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCapsuleWires(
        startPos: Vector3,
        endPos: Vector3,
        radius: f32,
        slices: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
}
extern "C" {
    pub fn DrawRay(ray: Ray, color: Color);
}
extern "C" {
    pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: f32);
}
extern "C" {
    pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;
}
extern "C" {
    pub fn LoadModelFromMesh(mesh: Mesh) -> Model;
}
extern "C" {
    pub fn IsModelReady(model: Model) -> bool;
}
extern "C" {
    pub fn UnloadModel(model: Model);
}
extern "C" {
    pub fn GetModelBoundingBox(model: Model) -> BoundingBox;
}
extern "C" {
    pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelWiresEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawModelPoints(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelPointsEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBoundingBox(box_: BoundingBox, color: Color);
}
extern "C" {
    pub fn DrawBillboard(
        camera: Camera,
        texture: Texture2D,
        position: Vector3,
        scale: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBillboardRec(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBillboardPro(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
}
extern "C" {
    pub fn UpdateMeshBuffer(
        mesh: Mesh,
        index: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn UnloadMesh(mesh: Mesh);
}
extern "C" {
    pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
}
extern "C" {
    pub fn DrawMeshInstanced(
        mesh: Mesh,
        material: Material,
        transforms: *const Matrix,
        instances: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;
}
extern "C" {
    pub fn GenMeshTangents(mesh: *mut Mesh);
}
extern "C" {
    pub fn ExportMesh(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportMeshAsCode(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GenMeshPoly(sides: ::std::os::raw::c_int, radius: f32) -> Mesh;
}
extern "C" {
    pub fn GenMeshPlane(
        width: f32,
        length: f32,
        resX: ::std::os::raw::c_int,
        resZ: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh;
}
extern "C" {
    pub fn GenMeshSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshHemiSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshCylinder(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
extern "C" {
    pub fn GenMeshCone(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
extern "C" {
    pub fn GenMeshTorus(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshKnot(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;
}
extern "C" {
    pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;
}
extern "C" {
    pub fn LoadMaterials(
        fileName: *const ::std::os::raw::c_char,
        materialCount: *mut ::std::os::raw::c_int,
    ) -> *mut Material;
}
extern "C" {
    pub fn LoadMaterialDefault() -> Material;
}
extern "C" {
    pub fn IsMaterialReady(material: Material) -> bool;
}
extern "C" {
    pub fn UnloadMaterial(material: Material);
}
extern "C" {
    pub fn SetMaterialTexture(
        material: *mut Material,
        mapType: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
extern "C" {
    pub fn SetModelMeshMaterial(
        model: *mut Model,
        meshId: ::std::os::raw::c_int,
        materialId: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn LoadModelAnimations(
        fileName: *const ::std::os::raw::c_char,
        animCount: *mut ::std::os::raw::c_int,
    ) -> *mut ModelAnimation;
}
extern "C" {
    pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UnloadModelAnimation(anim: ModelAnimation);
}
extern "C" {
    pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: ::std::os::raw::c_int);
}
extern "C" {
    pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;
}
extern "C" {
    pub fn UpdateModelAnimationBoneMatrices(
        model: Model,
        anim: ModelAnimation,
        frame: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn CheckCollisionSpheres(
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;
}
extern "C" {
    pub fn CheckCollisionBoxSphere(box_: BoundingBox, center: Vector3, radius: f32) -> bool;
}
extern "C" {
    pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: f32) -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionBox(ray: Ray, box_: BoundingBox) -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)
        -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionQuad(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision;
}
pub type AudioCallback = ::std::option::Option<
    unsafe extern "C" fn(bufferData: *mut ::std::os::raw::c_void, frames: ::std::os::raw::c_uint),
>;
extern "C" {
    pub fn InitAudioDevice();
}
extern "C" {
    pub fn CloseAudioDevice();
}
extern "C" {
    pub fn IsAudioDeviceReady() -> bool;
}
extern "C" {
    pub fn SetMasterVolume(volume: f32);
}
extern "C" {
    pub fn GetMasterVolume() -> f32;
}
extern "C" {
    pub fn LoadWave(fileName: *const ::std::os::raw::c_char) -> Wave;
}
extern "C" {
    pub fn LoadWaveFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Wave;
}
extern "C" {
    pub fn IsWaveReady(wave: Wave) -> bool;
}
extern "C" {
    pub fn LoadSound(fileName: *const ::std::os::raw::c_char) -> Sound;
}
extern "C" {
    pub fn LoadSoundFromWave(wave: Wave) -> Sound;
}
extern "C" {
    pub fn LoadSoundAlias(source: Sound) -> Sound;
}
extern "C" {
    pub fn IsSoundReady(sound: Sound) -> bool;
}
extern "C" {
    pub fn UpdateSound(
        sound: Sound,
        data: *const ::std::os::raw::c_void,
        sampleCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn UnloadWave(wave: Wave);
}
extern "C" {
    pub fn UnloadSound(sound: Sound);
}
extern "C" {
    pub fn UnloadSoundAlias(alias: Sound);
}
extern "C" {
    pub fn ExportWave(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportWaveAsCode(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn PlaySound(sound: Sound);
}
extern "C" {
    pub fn StopSound(sound: Sound);
}
extern "C" {
    pub fn PauseSound(sound: Sound);
}
extern "C" {
    pub fn ResumeSound(sound: Sound);
}
extern "C" {
    pub fn IsSoundPlaying(sound: Sound) -> bool;
}
extern "C" {
    pub fn SetSoundVolume(sound: Sound, volume: f32);
}
extern "C" {
    pub fn SetSoundPitch(sound: Sound, pitch: f32);
}
extern "C" {
    pub fn SetSoundPan(sound: Sound, pan: f32);
}
extern "C" {
    pub fn WaveCopy(wave: Wave) -> Wave;
}
extern "C" {
    pub fn WaveCrop(
        wave: *mut Wave,
        initFrame: ::std::os::raw::c_int,
        finalFrame: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn WaveFormat(
        wave: *mut Wave,
        sampleRate: ::std::os::raw::c_int,
        sampleSize: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn LoadWaveSamples(wave: Wave) -> *mut f32;
}
extern "C" {
    pub fn UnloadWaveSamples(samples: *mut f32);
}
extern "C" {
    pub fn LoadMusicStream(fileName: *const ::std::os::raw::c_char) -> Music;
}
extern "C" {
    pub fn LoadMusicStreamFromMemory(
        fileType: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Music;
}
extern "C" {
    pub fn IsMusicReady(music: Music) -> bool;
}
extern "C" {
    pub fn UnloadMusicStream(music: Music);
}
extern "C" {
    pub fn PlayMusicStream(music: Music);
}
extern "C" {
    pub fn IsMusicStreamPlaying(music: Music) -> bool;
}
extern "C" {
    pub fn UpdateMusicStream(music: Music);
}
extern "C" {
    pub fn StopMusicStream(music: Music);
}
extern "C" {
    pub fn PauseMusicStream(music: Music);
}
extern "C" {
    pub fn ResumeMusicStream(music: Music);
}
extern "C" {
    pub fn SeekMusicStream(music: Music, position: f32);
}
extern "C" {
    pub fn SetMusicVolume(music: Music, volume: f32);
}
extern "C" {
    pub fn SetMusicPitch(music: Music, pitch: f32);
}
extern "C" {
    pub fn SetMusicPan(music: Music, pan: f32);
}
extern "C" {
    pub fn GetMusicTimeLength(music: Music) -> f32;
}
extern "C" {
    pub fn GetMusicTimePlayed(music: Music) -> f32;
}
extern "C" {
    pub fn LoadAudioStream(
        sampleRate: ::std::os::raw::c_uint,
        sampleSize: ::std::os::raw::c_uint,
        channels: ::std::os::raw::c_uint,
    ) -> AudioStream;
}
extern "C" {
    pub fn IsAudioStreamReady(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn UnloadAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn UpdateAudioStream(
        stream: AudioStream,
        data: *const ::std::os::raw::c_void,
        frameCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn IsAudioStreamProcessed(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn PlayAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn PauseAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn ResumeAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn StopAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32);
}
extern "C" {
    pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32);
}
extern "C" {
    pub fn SetAudioStreamPan(stream: AudioStream, pan: f32);
}
extern "C" {
    pub fn SetAudioStreamBufferSizeDefault(size: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback);
}
extern "C" {
    pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
}
extern "C" {
    pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
}
extern "C" {
    pub fn AttachAudioMixedProcessor(processor: AudioCallback);
}
extern "C" {
    pub fn DetachAudioMixedProcessor(processor: AudioCallback);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    const UNINIT: ::std::mem::MaybeUninit<__va_list_tag> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gp_offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fp_offset) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overflow_arg_area) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reg_save_area) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
