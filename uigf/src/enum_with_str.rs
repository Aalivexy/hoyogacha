#[macro_export]
macro_rules! enum_with_str {
    ($(#[$enum_meta:meta])* $name:ident { $($(#[$meta:meta])* $variant:ident => $str_val:expr),* $(,)? }) => {
        $(#[$enum_meta])*
        #[non_exhaustive]
        #[derive(::serde::Serialize,
                 ::serde::Deserialize,
                 ::core::fmt::Debug,
                 ::core::marker::Copy,
                 ::core::clone::Clone,
                 ::core::cmp::PartialEq,
                 ::core::hash::Hash)]
        pub enum $name {
            $(
                $(#[$meta])*
                #[serde(rename = $str_val)]
                $variant,
            )*
        }

        impl $name {
            pub fn all_variants() -> &'static [$name] {
                &[$($name::$variant),*]
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let s = match self {
                    $(
                        $name::$variant => $str_val,
                    )*
                };
                write!(f, "{}", s)
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = ::std::boxed::Box<dyn ::std::error::Error>;

            fn from_str(input: &str) -> ::core::result::Result<$name, Self::Err> {
                match input {
                    $(
                        $str_val => Ok($name::$variant),
                    )*
                    _ => Err("invalid enum string".into()),
                }
            }
        }

        impl ::core::convert::TryFrom<::std::string::String> for $name {
            type Error = ::std::boxed::Box<dyn ::std::error::Error>;

            fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, Self::Error> {
                match value.as_str() {
                    $(
                        $str_val => Ok($name::$variant),
                    )*
                    _ => Err("invalid enum string".into()),
                }
            }
        }
    };
}
