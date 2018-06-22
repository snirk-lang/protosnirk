//! Contains wrappers for `llvm-sys`.

// Have to write the macro before the list of modules.

/// Implement basic methods `from_ref` and `ptr()`.
macro_rules! llvm_methods {
    ($name:ty => $wrapped:ty) => {
        /// Wrap an existing LLVM value.
        pub unsafe fn from_ref(ptr: $wrapped) -> $name {
            if cfg!(test) && ptr.is_null() {
                panic!("Attempt to construct {} with a null {}",
                    stringify!($name), stringify!($wrapped));
            }
            Self {
                ptr, _lt: ::std::marker::PhantomData
            }
        }

        /// Access the wrapped value
        pub fn ptr(&self) -> $wrapped {
            self.ptr
        }
    }
}

/// Implement fmt::Pointer for the wrapped LLVM value
macro_rules! impl_llvm_ptr_fmt {
    ($name:ident) => {
        impl ::std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:p}", self.ptr)
            }
        }
    };
    (<$lt:tt> $name:ident) => {
        impl<$lt> ::std::fmt::Pointer for $name<$lt> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:p}", self.ptr)
            }
        }
    }
}

macro_rules! llvm_passthrough {
    ($(#[$attr:meta])* pub fn $fn_name:ident( $($arg_name:ident : $arg_ty:ty),* ) => $wrapped_name:ident; $($rest:tt)*) => {
        $(#[$attr])*
        pub fn $fn_name(&self
                        $(
                            , $arg_name : $arg_ty
                        )*
                       ) {
            unsafe {
                $wrapped_name(self.ptr()
                    $(
                        , $arg_name.ptr()
                    )*
                );
            }
        }
        llvm_passthrough!($($rest)*);
    };
    ($(#[$attr:meta])* pub fn $fn_name:ident( $($arg_name:ident : $arg_ty:ty),* )
                           -> $ret_ty:ident <$lt:tt> => $wrapped_name:ident; $($rest:tt)*) => {
        $(#[$attr])*
        pub fn $fn_name(&self $(, $arg_name : $arg_ty)* ) -> $ret_ty  {
            unsafe {
                $ret_ty::from_ref($wrapped_name(self.ptr()
                    $(
                        , $arg_name.ptr()
                    )*
                ))
            }
        }

        llvm_passthrough!($($rest)*);
    };
    () => {};
}

mod util;
pub mod module;
pub use self::module::Module;
pub mod context;
pub use self::context::Context;
pub mod builder;
pub use self::builder::Builder;
pub mod basic_block;
pub use self::basic_block::BasicBlock;
pub mod value;
pub use self::value::Value;
pub mod types;
pub use self::types::Type;
pub mod pass_manager;
pub use self::pass_manager::{PassManager, FunctionPassManager};
