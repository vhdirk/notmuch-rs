#[macro_escape]
macro_rules! notmuch_enum {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident => $name_alias:ident {
            $($variant:ident => $variant_alias:ident),*
        }
    ) => {
        $(#[$enum_attr])*
        pub enum $name {
            $($variant),*
        }

        $(#[$enum_attr])*
        pub enum $name_alias {
            $($variant_alias),*
        }

        impl From<$name> for $name_alias {
            fn from(t: $name) -> Self {
                match t {
                    $($name::$variant => $name_alias::$variant_alias),*
                }
            }
        }

        impl Into<$name> for $name_alias {
            fn into(self) -> $name {
                match self {
                    $($name_alias::$variant_alias => $name::$variant),*
                }
            }
        }
    }
}
