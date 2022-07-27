use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ffi::CStr;
use crate::git::raw;

pub struct Signature<'text> {
  raw: *const raw::git_signature,
  _marker: PhantomData<&'text str>
}

impl<'text> Signature<'text> {
  pub fn new(raw: *const raw::git_signature) -> Signature<'text> {
    Signature { 
        raw,
        _marker: PhantomData
      }
  }

  /// Return the author's name as a`&str`,
  /// or `None` if it is not well-formated
  /// UTF-8. 
  pub fn name(&self) -> Option<&str> {
    unsafe {
      char_ptr_to_str(self, (*self.raw).name)
    }
  }

  /// Return the author's email as a`&str`,
  /// or `None` if it is not well-formated
  /// UTF-8. 
  pub fn email(&self) -> Option<&str> {
    unsafe {
      char_ptr_to_str(self, (*self.raw).email)
    }
  }
}

/// Try to borrow a `&str` from `ptr`, given that `ptr` may be
/// null or refer to ill-formed UTF-8. Give the result a 
/// lifetime as if it were borrowed from `_owner`.
/// 
/// Safety: if `ptr` is non-null, it must point to a null-terminated
/// C string that is safe to access for at least as long as the lifetime
/// of `_owner`.
pub unsafe fn char_ptr_to_str<T>(_owner: &T, ptr: *const c_char) -> Option<&str> {
  if ptr.is_null() {
    return None;
  } else {
    CStr::from_ptr(ptr).to_str().ok()
  }
}