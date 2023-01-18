//! A simple implementation of a bit vector built on top of a `Vec<u8>`.
//!
//! The [vecbool::VecBool] was implemented to replace a `Vec<bool>`
//! while retaining similar performance and reducing memory usage - since a `u8` can pack 8 `bool`, while a `Vec<bool>` uses one
//! byte to store one `bool`.
//!
//! **NOTE**: This is mostly a toy project used for learning purposes. If you want a more robust alternative
//! check the crate `bitvec` (https://docs.rs/bitvec/latest/bitvec/index.html).
//! If you found something which can be improved, feel free to create an issue on https://github.com/mymatsubara/vecbool.
//! Any feedback is more than welcomed!
//!
//! # Examples
//!
//! ```
//! use vecbool::VecBool;
//!
//! let mut vecbool = VecBool::new();
//!
//! assert_eq!(vecbool.get(0), None);
//!
//! vecbool.push(true);
//! vecbool.push(false);
//! assert_eq!(vecbool.get(0), Some(true));
//! assert_eq!(vecbool.get(1), Some(false));
//!
//! let vec: Vec<_> = vecbool.iter().collect();
//! assert_eq!(vec, vec![true, false]);
//!
//! assert_eq!(vecbool.pop(), Some(false));
//! assert_eq!(vecbool.pop(), Some(true));
//! assert_eq!(vecbool.pop(), None);
//! ```
//!

mod vecbool;
pub use crate::vecbool::*;
