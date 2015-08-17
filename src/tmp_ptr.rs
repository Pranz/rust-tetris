use core::fmt;
use core::ops::Deref;

///Temporary pointer which is unsafe when moved in inappropriate places.
///There's nothing special about this type. It is only for making the search & replace easier when a better way of doing this is found.
///This is a temporary workaround for no support of `for<'l> Fn(&'l _)` closures. There may be another way of doing this that I don't know of
pub struct TmpPtr<T: ?Sized>(*const T);

impl<T: ?Sized> TmpPtr<T>{
	pub fn new<'l>(value: &'l T) -> Self{TmpPtr(&*value)}
}

impl<T: ?Sized> !Sync for TmpPtr<T>{}
impl<T: ?Sized> !Send for TmpPtr<T>{}
impl<T: ?Sized> Deref for TmpPtr<T>{
	type Target = T;

	fn deref<'l>(&'l self) -> &'l T{
		unsafe{&*self.0}
	}
}
impl<T: ?Sized> fmt::Debug for TmpPtr<T>{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		writeln!(f,"TmpPtr(...)")
	}
}

impl<T: ?Sized> Copy for TmpPtr<T>{}
impl<T: ?Sized> Clone for TmpPtr<T>{
	fn clone(&self) -> Self{
		*self
	}
}
