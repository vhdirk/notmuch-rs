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

        impl NotmuchType for $name_alias {
            type NotmuchT = $name;

            fn from_notmuch_t(notmuch_t: $name) -> Self {
                match notmuch_t {
                    $($name::$variant => $name_alias::$variant_alias),*
                }
            }

            fn to_notmuch_t(self) -> $name {
                match self {
                    $($name_alias::$variant_alias => $name::$variant),*
                }
            }
        }
    }
}
