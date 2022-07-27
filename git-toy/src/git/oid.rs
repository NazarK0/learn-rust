use super::raw;

/// The identifier of some sort of object stored in
/// the Git object database: a commit, tree, blob, tag, etc.
/// This is a wide hash of the object's contents.
pub struct Oid {
  pub raw: raw::git_oid
}