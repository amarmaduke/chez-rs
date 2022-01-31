
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw;

pub type ptr = *mut raw::c_void;
pub type iptr = raw::c_long;
pub type uptr = raw::c_ulong;
pub type string_char = raw::c_uint;
pub type octet = raw::c_uchar;

#[inline(always)]
pub unsafe fn inc_ptr<T: Copy>(x: ptr, by: uptr) -> T { *(((x as uptr) + by) as *mut T) }

macro_rules! predicate {
    ($name:ident, $loc:literal) => {
        #[inline(always)]
        pub fn $name(x: ptr) -> bool { x as uptr == $loc }
    };

    ($name:ident, $mask:literal, $loc:literal) => {
        #[inline(always)]
        pub fn $name(x: ptr) -> bool { (x as uptr) & $mask == $loc }
    };

    ($name:ident, $mask1:literal, $loc1:literal, $loc2:literal) => {
        #[inline(always)]
        pub unsafe fn $name(x: ptr) -> bool {
            ((x as uptr) & $mask1 == $loc1)
            && ((inc_ptr::<ptr>(x, 1) as uptr) == $loc2)
        }
    };

    ($name:ident, $mask1:literal, $loc1:literal, $mask2:literal, $loc2:literal) => {
        #[inline(always)]
        pub unsafe fn $name(x: ptr) -> bool {
            ((x as uptr) & $mask1 == $loc1)
            && ((inc_ptr::<ptr>(x, 1) as uptr) & $mask2 == $loc2)
        }
    }
}

// Type predicates
predicate!(Sfixnump, 0x7, 0x0);
predicate!(Scharp, 0xFF, 0x16);
predicate!(Snullp, 0x26);
predicate!(Seof_objectp, 0x36);
predicate!(Sbwp_objectp, 0x4E);
predicate!(Sbooleanp, 0xF7, 0x6);
predicate!(Spairp, 0x7, 0x1);
predicate!(Ssymbolp, 0x7, 0x3);
predicate!(Sprocedurep, 0x7, 0x5);
predicate!(Sflonump, 0x7, 0x2);
predicate!(Svectorp, 0x7, 0x7, 0x7, 0x0);
predicate!(Sfxvectorp, 0x7, 0x7, 0x7, 0x3);
predicate!(Sbytevectorp, 0x7, 0x7, 0x3, 0x1);
predicate!(Sstringp, 0x7, 0x7, 0x7, 0x2);
predicate!(Sbignump, 0x7, 0x7, 0x1F, 0x6);
predicate!(Sboxp, 0x7, 0x7, 0x7F, 0xE);
predicate!(Sinexactnump, 0x7, 0x7, 0x36);
predicate!(Sexactnump, 0x7, 0x7, 0x56);
predicate!(Sratnump, 0x7, 0x7, 0x16);
predicate!(Sinputportp, 0x7, 0x7, 0x1FF, 0x11E);
predicate!(Soutputportp, 0x7, 0x7, 0x2FF, 0x21E);
predicate!(Srecordp, 0x7, 0x7, 0x7, 0x7);

// Accessors
#[inline(always)] pub fn Sfixnum_value(x: ptr) -> iptr { (x as iptr) / 8 }
#[inline(always)] pub fn Schar_value(x: ptr) -> string_char { ((x as uptr) >> 8) as string_char }
#[inline(always)] pub fn Sboolean_value(x: ptr) -> bool { x != Sfalse }
#[inline(always)] pub unsafe fn Scar(x: ptr) -> ptr { inc_ptr(x, 7) }
#[inline(always)] pub unsafe fn Scdr(x: ptr) -> ptr { inc_ptr(x, 15) }
#[inline(always)] pub unsafe fn Sflonum_value(x: ptr) -> raw::c_double { inc_ptr(x, 6) }
// #define Svector_length(x) ((iptr)((uptr)(*((iptr *)((uptr)(x)+1)))>>4))
// #define Svector_ref(x,i) (((ptr *)((uptr)(x)+9))[i])
// #define Sfxvector_length(x) ((iptr)((uptr)(*((iptr *)((uptr)(x)+1)))>>4))
// #define Sfxvector_ref(x,i) (((ptr *)((uptr)(x)+9))[i])
// #define Sbytevector_length(x) ((iptr)((uptr)(*((iptr *)((uptr)(x)+1)))>>3))
// #define Sbytevector_u8_ref(x,i) (((octet *)((uptr)(x)+9))[i])
/* Warning: Sbytevector_data(x) returns a pointer into x. */
// #define Sbytevector_data(x) &Sbytevector_u8_ref(x,0)
#[inline(always)] pub unsafe fn Sstring_length(x: ptr) -> iptr { ((inc_ptr::<iptr>(x, 1) as uptr) >> 4) as iptr}
// #define Sstring_ref(x,i) Schar_value(((string_char *)((uptr)(x)+9))[i])
// #define Sunbox(x) (*((ptr *)((uptr)(x)+9)))
#[inline(always)] pub unsafe fn Sunbox(x: ptr) -> ptr { inc_ptr(x, 9) }
extern "C" { pub fn Sinteger_value(arg1: ptr) -> iptr; }
// #define Sunsigned_value(x) (uptr)Sinteger_value(x)
extern "C" { pub fn Sinteger32_value(arg1: ptr) -> raw::c_int; }
// #define Sunsigned32_value(x) (unsigned int)Sinteger32_value(x)
extern "C" { pub fn Sinteger64_value(arg1: ptr) -> raw::c_long; }
// #define Sunsigned64_value(x) (unsigned long)Sinteger64_value(x)


// Mutators
extern "C" { pub fn Sset_box(arg1: ptr, arg2: ptr); }
extern "C" { pub fn Sset_car(arg1: ptr, arg2: ptr); }
extern "C" { pub fn Sset_cdr(arg1: ptr, arg2: ptr); }
// #define Sstring_set(x,i,c) ((void)((((string_char *)((uptr)(x)+9))[i]) = (string_char)(uptr)Schar(c)))
// #define Sfxvector_set(x,i,n) ((void)(Sfxvector_ref(x,i) = (n)))
// #define Sbytevector_u8_set(x,i,n) ((void)(Sbytevector_u8_ref(x,i) = (n)))
extern "C" { pub fn Svector_set(arg1: ptr, arg2: iptr, arg3: ptr); }


// Constructors
#[inline(always)] pub const fn Sfixnum(x: iptr) -> ptr { ((x * 8) as uptr) as ptr }
#[inline(always)] pub const fn Schar(x: string_char) -> ptr { ((x << 8 | 0x16) as uptr) as ptr }
pub const Snil: ptr = 0x26 as ptr;
pub const Strue: ptr = 0xE as ptr;
pub const Sfalse: ptr = 0x6 as ptr;
#[inline(always)] pub const fn Sboolean(x: bool) -> ptr { if x { Strue } else { Sfalse } }
pub const Sbwp_object: ptr = 0x4E as ptr;
pub const Seof_object: ptr = 0x36 as ptr;
pub const Svoid: ptr = 0x3E as ptr;
extern "C" { pub fn Scons(arg1: ptr, arg2: ptr) -> ptr; }
extern "C" { pub fn Sstring_to_symbol(arg1: *const raw::c_char) -> ptr; }
extern "C" { pub fn Ssymbol_to_string(arg1: ptr) -> ptr; }
extern "C" { pub fn Sflonum(arg1: f64) -> ptr; }
extern "C" { pub fn Smake_vector(arg1: iptr, arg2: ptr) -> ptr; }
extern "C" { pub fn Smake_fxvector(arg1: iptr, arg2: ptr) -> ptr; }
extern "C" { pub fn Smake_bytevector(arg1: iptr, arg2: raw::c_int) -> ptr; }
extern "C" { pub fn Smake_string(arg1: iptr, arg2: raw::c_int) -> ptr; }
extern "C" { pub fn Smake_uninitialized_string(arg1: iptr) -> ptr; }
extern "C" { pub fn Sstring(arg1: *const raw::c_char) -> ptr; }
extern "C" { pub fn Sstring_of_length(arg1: *const raw::c_char, arg2: iptr) -> ptr; }
extern "C" { pub fn Sstring_utf8(arg1: *const raw::c_char, arg2: iptr) -> ptr; }
extern "C" { pub fn Sbox(arg1: ptr) -> ptr; }
extern "C" { pub fn Sinteger(arg1: iptr) -> ptr; }
extern "C" { pub fn Sunsigned(arg1: uptr) -> ptr; }
extern "C" { pub fn Sinteger32(arg1: raw::c_int) -> ptr; }
extern "C" { pub fn Sunsigned32(arg1: raw::c_uint) -> ptr; }
extern "C" { pub fn Sinteger64(arg1: raw::c_long) -> ptr; }
extern "C" { pub fn Sunsigned64(arg1: raw::c_ulong) -> ptr; }

// Miscellaneous
extern "C" { pub fn Stop_level_value(arg1: ptr) -> ptr; }
extern "C" { pub fn Sset_top_level_value(arg1: ptr, arg2: ptr); }
extern "C" { pub fn Slock_object(arg1: ptr); }
extern "C" { pub fn Sunlock_object(arg1: ptr); }
extern "C" { pub fn Slocked_objectp(arg1: ptr) -> raw::c_int; }
extern "C" { pub fn Sforeign_symbol(arg1: *const raw::c_char, arg2: *mut raw::c_void); }
extern "C" { pub fn Sregister_symbol(arg1: *const raw::c_char, arg2: *mut raw::c_void); }

// Support for calls into Scheme
extern "C" { pub fn Scall0(arg1: ptr) -> ptr; }
extern "C" { pub fn Scall1(arg1: ptr, arg2: ptr) -> ptr; }
extern "C" { pub fn Scall2(arg1: ptr, arg2: ptr, arg3: ptr) -> ptr; }
extern "C" { pub fn Scall3(arg1: ptr, arg2: ptr, arg3: ptr, arg4: ptr) -> ptr; }
extern "C" { pub fn Sinitframe(arg1: iptr); }
extern "C" { pub fn Sput_arg(arg1: iptr, arg2: ptr); }
extern "C" { pub fn Scall(arg1: ptr, arg2: iptr) -> ptr; }
#[inline(always)] pub unsafe fn Sforeign_callable_entry_point(x: ptr) -> Option<unsafe extern "C" fn()> { inc_ptr(x, 65) }
#[inline(always)] pub unsafe fn Sforeign_callable_code_object(x: unsafe extern "C" fn()) -> ptr { ((x as uptr) - 65) as ptr }

// Customization support
extern "C" { pub fn Skernel_version() -> *const raw::c_char; }
extern "C" { pub fn Sretain_static_relocation(); }
extern "C" { pub fn Sset_verbose(arg1: raw::c_int); }
extern "C" { pub fn Sscheme_init(arg1: Option<unsafe extern "C" fn()>);}
extern "C" { pub fn Sregister_boot_file(arg1: *const raw::c_char); }
extern "C" { pub fn Sregister_boot_file_fd(arg1: *const raw::c_char, fd: raw::c_int); }
extern "C" { pub fn Sregister_heap_file(arg1: *const raw::c_char); }
extern "C" { pub fn Scompact_heap(); }
extern "C" { pub fn Ssave_heap(arg1: *const raw::c_char, arg2: raw::c_int); }
extern "C" { pub fn Sbuild_heap(arg1: *const raw::c_char, arg2: Option<unsafe extern "C" fn()>); }
extern "C" { pub fn Senable_expeditor(arg1: *const raw::c_char); }
extern "C" { pub fn Sscheme_start(arg1: raw::c_int, arg2: *mut *const raw::c_char) -> raw::c_int; }
extern "C" { pub fn Sscheme_script(arg1: *const raw::c_char, arg2: raw::c_int, arg3: *mut *const raw::c_char) -> raw::c_int; }
extern "C" { pub fn Sscheme_program(arg1: *const raw::c_char, arg2: raw::c_int, arg3: *mut *const raw::c_char) -> raw::c_int; }
extern "C" { pub fn Sscheme_deinit(); }
