macro_rules! value_enum {
    (
        $(#[$enum_attribs:meta])*
        $name:ident: $u_type:ty {
            $(
                $(#[$attribs:meta])*
                $variant:ident = $value:expr
            ),*
        }
    ) => {
        $(#[$enum_attribs])*
        pub enum $name {
            $(
                $(#[$attribs])*
                $variant
            ),*
        }

        impl TryFrom<$u_type> for $name {
            type Error = $u_type;

            fn try_from(value: $u_type) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant),)*
                    unknown => Err(unknown),
                }
            }
        }

        impl From<$name> for $u_type {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $value,)*
                }
            }
        }
    };

    (
        $(#[$enum_attribs:meta])*
        $name:ident: $u_type:ty {
            $(
                $(#[$attribs:meta])*
                $variant:ident = $value:expr
            ),*,
            [
                $(#[$default_attribs:meta])*
                $default:ident
            ]
        }
    ) => {
        $(#[$enum_attribs])*
        pub enum $name {
            $(
                $(#[$attribs])*
                $variant
            ),*,
            $(#[$default_attribs])*
            $default($u_type)
        }

        impl From<$u_type> for $name {
            fn from(value: $u_type) -> Self {
                match value {
                    $($value => Self::$variant,)*
                    other => Self::$default(other),
                }
            }
        }

        impl From<$name> for $u_type {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $value,)*
                    $name::$default(value) => value
                }
            }
        }
    };

    (
        $(#[$enum_attribs:meta])*
        $name:ident: $u_type:ty {
            $(#[$super_attribs:meta])*
            ...$super:ident,
            $(
                $(#[$attribs:meta])*
                $variant:ident = $value:expr
            ),*
        }
    ) => {
        $(#[$enum_attribs])*
        pub enum $name {
            $(#[$super_attribs])*
            $super($super),
            $(
                $(#[$attribs])*
                $variant
            ),*
        }

        impl TryFrom<$u_type> for $name {
            type Error = $u_type;
            fn try_from(value: $u_type) -> Result<Self, Self::Error> {
                Ok(match value {
                    $($value => Self::$variant,)*
                    other => Self::$super($super::try_from(other)?),
                })
            }
        }

        impl From<$name> for $u_type {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $value,)*
                    $name::$super(inner) => inner.into()
                }
            }
        }
    };
}

pub(crate) use value_enum;
