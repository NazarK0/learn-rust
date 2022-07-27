use std::{ ptr, mem, path::Path};
use std::os::raw::{c_int, c_char};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use super::raw;
use super::error::{Error, Result};
use super::oid::Oid;
use super::commit::Commit;

fn check(code: c_int) -> Result<c_int> {
  if code >= 0 {
    return Ok(code);
  }

  unsafe {
    let error = raw::giterr_last();

    // libgit2 ensures that (*error).message is always non-null and
    // null terminated, so this call is safe.
    let message = CStr::from_ptr((*error).message)
      .to_string_lossy()
      .into_owned();

    Err(Error::throw(
      code as i32,
      message,
      (*error).klass as i32
    ))
  }
}

/// A Git repository.
pub struct Repository {
  // This must always be a pointer to a live `git_repository` structure.
  // No other `Repository` may point to it.
  raw: *mut raw::git_repository
}

impl Repository {
  pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
    ensure_initialized();

    let path = path_to_cstring(path.as_ref())?;
    let mut repo = ptr::null_mut();

    unsafe {
      check(raw::git_repository_open(&mut repo, path.as_ptr()))?;
    }

    Ok(Repository {raw: repo })
  }

  pub fn reference_name_to_id(&self, name: &str) -> Result<Oid> {
    let name = CString::new(name)?;

    unsafe {
      let oid = {
        let mut oid = mem::MaybeUninit::uninit();

        check(raw::git_reference_name_to_id(
          oid.as_mut_ptr(), 
          self.raw, 
          name.as_ptr() as *const c_char))?;

        oid.assume_init()
      };

      Ok(Oid { raw: oid })
    }
  }

  pub fn find_commit(&self, oid: &Oid) -> Result<Commit> {
    let mut commit = ptr::null_mut();

    unsafe {
      check(raw::git_commit_lookup(
        &mut commit, self.raw, &oid.raw))?;
    }

    Ok(Commit::new(commit))
  }
}

impl Drop for Repository {
  fn drop(&mut self) {
    unsafe {
      raw::git_repository_free(self.raw);
    }
  }
}

fn ensure_initialized() {
  static ONCE: std::sync::Once = std::sync::Once::new();

  ONCE.call_once(|| {
    unsafe {
      check(raw::git_libgit2_init())
        .expect("initializing libgit2 failed");
        
      assert_eq!(libc::atexit(shutdown), 0);
    }
  })
}

extern fn shutdown() {
  unsafe {
    if let Err(e) = check(raw::git_libgit2_shutdown()) {
      eprintln!("Failed to shutdown libgit2: {e}");
      std::process::abort();
    }
  }
}

#[cfg(unix)]
fn path_to_cstring(path: &Path) -> Result<CString> {
  // The `as_bytes` method exists only on Unix-like systems.
  use std::os::unix::ffi::OsStrExt;

  Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(windows)]
fn path_to_cstring(path: &Path) -> Result<CString> {
  // Try to convert to UTF-8. If this fails, libgit2 can't handle the path
  // anyway.
  match path.to_str() {
    Some(s) => Ok(CString::new(s)?),
    None => {
      let message = format!("Couldn't convert path `{path}` to UTF-8",
        path = path.display());
      
      Err(message.into())
    }
  }
}
