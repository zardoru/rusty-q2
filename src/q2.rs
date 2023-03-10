use std::ffi::{c_char, CString, c_void};
use crate::{GI, Vec3, Trace, Edict, PMove};

pub const TAG_GAME: i32 = 768;

pub fn tag_malloc<T>(count: i32, tag: i32) -> *mut T {
    unsafe {
        let size = std::mem::size_of::<T>() * (count as usize);
        (GI.as_ref().unwrap_unchecked().tag_malloc)(size as i32, tag) as *mut T
    }
}

pub fn tag_free<T>(ptr: *mut T) {
    unsafe {
        (GI.as_ref().unwrap_unchecked().tag_free)(ptr as *mut c_void);
    }
}

pub fn dprintf(s: &str) {
    unsafe {
        let c_str = CString::new(s).unwrap();
        (GI.as_ref().unwrap_unchecked().dprintf)(c_str.as_ptr() as *const c_char);
    }
}

pub struct Cvar {
    key: String,
    value: String,
    flags: i32,
    ptr: *const super::Cvar
}

impl Cvar {
    pub fn new(key: &str, value: &str, flags: i32) -> Cvar {
        Cvar {
            key: String::from(key),
            value: String::from(value),
            flags,
            ptr: std::ptr::null()
        }
    }

    fn validate_ptr(&mut self) {
        if self.ptr == std::ptr::null() {
            let name = CString::new(&*self.key).unwrap();
            let value = CString::new(&*self.value).unwrap();
            unsafe {
                self.ptr = (GI.unwrap_unchecked().cvar)(
                    name.as_ptr() as *const c_char,
                    value.as_ptr() as *const c_char,
                    self.flags
                );
            }
        }
    }

    pub fn value_f32(&mut self) -> f32 {
        self.validate_ptr();
        unsafe { (*self.ptr).value }
    }

    pub fn value_i32(&mut self) -> i32 {
        self.validate_ptr();
        unsafe { (*self.ptr).value as i32 }
    }
}

pub fn error(msg: String) {
    unsafe {
        let name = CString::new(&*msg).unwrap();
        (GI.unwrap_unchecked().error)(name.as_ptr() as *const c_char)
    }
}


pub fn free_tags(tag: i32) {
    unsafe { (GI.unwrap_unchecked().free_tags)(tag) }
}

pub fn trace(start: &Vec3, end: &Vec3, mins: &Vec3, maxs: &Vec3, ignore: *const Edict, flags: i32) -> Trace {
    unsafe {
        (GI.unwrap_unchecked().trace)(start as *const Vec3, end as *const Vec3, mins as *const Vec3, maxs as *const Vec3, ignore, flags)
    }
}

pub fn point_contents(point: &Vec3) -> i32 {
    unsafe {
        (GI.unwrap_unchecked().pointcontents)(point as *const Vec3)
    }
}

pub fn pmove(p: &mut PMove) {
    unsafe {
        (GI.unwrap_unchecked().pmove)(p as *mut PMove)
    }
}