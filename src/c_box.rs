use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::drop_in_place;
use std::ptr::NonNull;

pub type FnFree = unsafe extern "C" fn(ptr: *const u8, cookie: *mut c_void);

/// A `free` "closure", i.e. a [`FnFree`] and an enclosed context [`Self::cookie`].
#[derive(Debug)]
pub struct Free {
    pub free: FnFree,
    pub cookie: *mut c_void,
}

impl Free {
    pub unsafe fn free(&self, ptr: *mut c_void) {
        (self.free)(ptr as *const u8, self.cookie)
    }
}

/// A C/custom [`Box`].
///
/// That is, it is analogous to a [`Box`],
/// but it lets you set a C-style `free` `fn` for deallocation
/// instead of the normal [`Box`] (de)allocator.
/// It can also store a normal [`Box`] as well.
#[derive(Debug)]
pub enum CBox<T: ?Sized> {
    Rust(Box<T>),
    C {
        /// # Safety:
        ///
        /// * Never moved.
        /// * Valid to dereference.
        /// * `free`d by the `free` `fn` ptr below.
        data: NonNull<T>,
        free: Free,
        _phantom: PhantomData<T>,
    },
}

impl<T: ?Sized> AsRef<T> for CBox<T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Rust(r#box) => r#box.as_ref(),
            Self::C { data, .. } => unsafe { data.as_ref() },
        }
    }
}

impl<T: ?Sized> Deref for CBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> Drop for CBox<T> {
    fn drop(&mut self) {
        match self {
            Self::Rust(_) => {} // Drop normally.
            Self::C { data, free, .. } => {
                let ptr = data.as_ptr();
                // Safety: See below.
                // The [`FnFree`] won't run Rust's `fn drop`,
                // so we have to do this ourselves first.
                unsafe { drop_in_place(ptr) };
                let ptr = ptr.cast();
                // Safety: See safety docs on [`Self::data`] and [`Self::from_c`].
                unsafe { free.free(ptr) }
            }
        }
    }
}

impl<T: ?Sized> CBox<T> {
    /// # Safety
    ///
    /// `data` must be valid to dereference
    /// until `free` is called on it, which must deallocate it.
    pub unsafe fn from_c(data: NonNull<T>, free: Free) -> Self {
        Self::C {
            data,
            free,
            _phantom: PhantomData,
        }
    }

    pub fn from_box(data: Box<T>) -> Self {
        Self::Rust(data)
    }
}
