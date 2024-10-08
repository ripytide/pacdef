use crate::prelude::*;

macro_rules! any {
    ($($backend:ident),*) => {
        #[derive(Debug, Copy, Clone, derive_more::FromStr, derive_more::Display)]
        pub enum AnyBackend {
            $($backend,)*
        }
    };
}
apply_public_backends!(any);
