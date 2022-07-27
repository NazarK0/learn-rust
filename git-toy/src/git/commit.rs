mod signature;

use std::marker::PhantomData;
use super::{raw, repository::Repository};
use signature::{Signature, char_ptr_to_str};

pub struct Commit<'repo> {
  // This must always be a pointer to a usable `git_commit`
  // structure.
  raw: *mut raw::git_commit,
  _marker: PhantomData<&'repo Repository>
}

impl<'repo> Drop for Commit<'repo> {
  fn drop(&mut self) {
    unsafe {
      raw::git_commit_free(self.raw);
    }
  }
}

impl<'repo> Commit<'repo> {
  pub fn new(raw: *mut raw::git_commit) -> Commit<'repo> {
    Commit { raw, _marker: PhantomData }
  }
  pub fn author(&self) -> Signature {
    unsafe {
      Signature::new ( 
        raw::git_commit_author(self.raw),
      )
    }
  }

  pub fn message(&self) -> Option<&str> {
    unsafe {
      let message = raw::git_commit_message(self.raw);
      char_ptr_to_str(self, message)
    }
  }
}