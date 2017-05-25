use std::marker;
use std::mem;
use std::slice;

use {raw, Oid, Object};
use util::Binding;

/// A structure to represent a git [blob][1]
///
/// [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
pub struct Blob<'repo> {
    raw: *mut raw::git_blob,
    _marker: marker::PhantomData<Object<'repo>>,
}

impl<'repo> Blob<'repo> {
    /// Get the id (SHA1) of a repository blob
    pub fn id(&self) -> Oid {
        unsafe { Binding::from_raw(raw::git_blob_id(&*self.raw)) }
    }

    /// Determine if the blob content is most certainly binary or not.
    pub fn is_binary(&self) -> bool {
        unsafe { raw::git_blob_is_binary(&*self.raw) == 1 }
    }

    /// Get the content of this blob.
    pub fn content(&self) -> &[u8] {
        unsafe {
            let data = raw::git_blob_rawcontent(&*self.raw) as *const u8;
            let len = raw::git_blob_rawsize(&*self.raw) as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Casts this Blob to be usable as an `Object`
    pub fn as_object(&self) -> &Object<'repo> {
        unsafe {
            &*(self as *const _ as *const Object<'repo>)
        }
    }

    /// Consumes Blob to be returned as an `Object`
    pub fn into_object(self) -> Object<'repo> {
        assert_eq!(mem::size_of_val(&self), mem::size_of::<Object>());
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<'repo> Binding for Blob<'repo> {
    type Raw = *mut raw::git_blob;

    unsafe fn from_raw(raw: *mut raw::git_blob) -> Blob<'repo> {
        Blob {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_blob { self.raw }
}


impl<'repo> Drop for Blob<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_blob_free(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use tempdir::TempDir;
    use Repository;

    #[test]
    fn buffer() {
        let td = TempDir::new("test").unwrap();
        let repo = Repository::init(td.path()).unwrap();
        let id = repo.blob(&[5, 4, 6]).unwrap();
        let blob = repo.find_blob(id).unwrap();

        assert_eq!(blob.id(), id);
        assert_eq!(blob.content(), [5, 4, 6]);
        assert!(blob.is_binary());

        repo.find_object(id, None).unwrap().as_blob().unwrap();
        repo.find_object(id, None).unwrap().into_blob().ok().unwrap();
    }

    #[test]
    fn path() {
        let td = TempDir::new("test").unwrap();
        let path = td.path().join("foo");
        File::create(&path).unwrap().write_all(&[7, 8, 9]).unwrap();
        let repo = Repository::init(td.path()).unwrap();
        let id = repo.blob_path(&path).unwrap();
        let blob = repo.find_blob(id).unwrap();
        assert_eq!(blob.content(), [7, 8, 9]);
        blob.into_object();
    }
}
