use std::cell::{Ref, RefCell, RefMut};
use std::mem;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::thread::{self, ThreadId};

use anyhow::{anyhow, Result};
use gdnative::export::user_data::{Map, MapMut, UserData};
use gdnative::libc;
use gdnative::prelude::NativeClass;

/// Just like gdnative::LocalCellData<T> except you can borrow it directly instead of using map()
#[derive(Debug)]
pub struct GoodCellData<T> {
	inner: Arc<GoodCell<T>>,
}

impl<T> GoodCellData<T> {
	#[inline]
	pub fn try_borrow(&self) -> Result<Ref<ManuallyDrop<T>>> {
		self.inner.try_borrow()
	}

	#[inline]
	pub fn try_borrow_mut(&self) -> Result<RefMut<ManuallyDrop<T>>> {
		self.inner.try_borrow_mut()
	}
}

#[derive(Debug)]
pub struct GoodCell<T> {
	thread_id: ThreadId,
	cell: RefCell<ManuallyDrop<T>>,
}

impl<T> Drop for GoodCell<T> {
	fn drop(&mut self) {
		if self.thread_id == thread::current().id() {
			unsafe {
				ManuallyDrop::drop(self.cell.get_mut());
			}
		}
	}
}

impl<T> GoodCell<T> {
	#[inline]
	pub fn new(val: T) -> Self {
		GoodCell {
			thread_id: thread::current().id(),
			cell: RefCell::new(ManuallyDrop::new(val)),
		}
	}

	#[inline]
	fn inner(&self) -> Result<&RefCell<ManuallyDrop<T>>> {
		let current = thread::current().id();

		if self.thread_id == current {
			Ok(&self.cell)
		} else {
			Err(anyhow!(
				"Attempted to access GoodCell from a different thread. \n\
				 Expected thread id: {:?} \n\
				 Current thread id: {current:?}", self.thread_id))
		}
	}

	#[inline]
	pub fn try_borrow(&self) -> Result<Ref<ManuallyDrop<T>>> {
		let inner = self.inner()?;
		inner.try_borrow()
		     .map_err(|err| anyhow!("Borrow failed: {err}"))
	}

	#[inline]
	pub fn try_borrow_mut(&self) -> Result<RefMut<ManuallyDrop<T>>> {
		let inner = self.inner()?;
		inner.try_borrow_mut()
		     .map_err(|err| anyhow!("Borrow failed: {err}"))
	}
}

// Implementing Send + Sync is ok because the cell is guarded from access outside the
// original thread.
unsafe impl<T> Send for GoodCell<T> {}

unsafe impl<T> Sync for GoodCell<T> {}

unsafe impl<T: NativeClass> UserData for GoodCellData<T> {
	type Target = T;

	#[inline]
	fn new(val: Self::Target) -> Self {
		GoodCellData {
			inner: Arc::new(GoodCell::new(val)),
		}
	}

	#[inline]
	fn into_user_data(self) -> *const libc::c_void {
		Arc::into_raw(self.inner) as *const libc::c_void
	}

	#[inline]
	unsafe fn consume_user_data_unchecked(ptr: *const libc::c_void) -> Self {
		GoodCellData {
			inner: Arc::from_raw(ptr as *const GoodCell<T>),
		}
	}

	#[inline]
	unsafe fn clone_from_user_data_unchecked(ptr: *const libc::c_void) -> Self {
		let borrowed = Arc::from_raw(ptr as *const GoodCell<T>);
		let arc = borrowed.clone();
		mem::forget(borrowed);
		GoodCellData { inner: arc }
	}
}

impl<T> Clone for GoodCellData<T> {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
		}
	}
}

impl<T: NativeClass> Map for GoodCellData<T> {
	type Err = anyhow::Error;

	#[inline]
	fn map<F: FnOnce(&Self::Target) -> U, U>(&self, op: F) -> Result<U> {
		self.inner.try_borrow().map(|r| op(&r))
	}
}

impl<T: NativeClass> MapMut for GoodCellData<T> {
	type Err = anyhow::Error;

	#[inline]
	fn map_mut<F: FnOnce(&mut Self::Target) -> U, U>(&self, op: F) -> Result<U> {
		self.inner.try_borrow_mut().map(|mut w| op(&mut w))
	}
}