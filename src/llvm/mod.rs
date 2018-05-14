//! Contains wrappers for `llvm-sys`.

// Have to write the macro before the list of modules.

/// Wraps an `LLVM{thing}Ref` to automatically call `Create` and
/// `Dispose` methods on `Drop` with a reference counted inner value.
#[macro_export]
macro_rules! llvm_wrapped {
    ($($(#[$attr:meta])*
     pub struct $wrapped_name:ident {
         value: $value:ty,
         dispose: $dispose:ident
     })+) => {
        $($(#[$attr])*
        pub struct $wrapped_name {
            inner: $value
        }

        impl Clone for $wrapped_name {
            fn clone(&self) -> $wrapped_name {
                $wrapped_name {
                    inner: self.inner
                }
            }
        }

        impl $wrapped_name {
            /// Wrap an existing LLVM value.
            pub fn from_ref(value: $value) -> $wrapped_name {
                if value.is_null() {
                    panic!("Attempt to construct a null {}",
                        stringify!($value));
                }
                $wrapped_name {
                    inner: value
                }
            }
        }

        impl ::std::fmt::Debug for $wrapped_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, stringify!($value))
            }
        }

        impl ::std::ops::Deref for $wrapped_name {
            type Target = $value;
            fn deref(&self) -> &$value {
                &self.inner
            }
        }

        impl ::std::ops::DerefMut for $wrapped_name {
            fn deref_mut(&mut self) -> &mut $value {
                &mut self.inner
            }
        }

        impl Drop for $wrapped_name {
            // In some cases LLVM will manage memory for us, such as for
            // Value and Type references. In those cases we pass `drop` to the
            // macro and the compiler will elide away drop logic.
            #[inline]
            fn drop(&mut self) {
                unsafe {
                    $dispose(self.inner);
                }
            }
        })+
    };
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
mod types;
pub use self::types::Type;
