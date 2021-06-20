use super::string_reader::StringReader;

pub trait Parseable: Sized {
    fn parse(reader: &mut StringReader) -> Option<Self>;
    fn parse_from(data: &str) -> Option<Self> {
        let ref mut reader = StringReader::new(data);
        Self::parse(reader)
    }
}

#[macro_export]
macro_rules! num_parseable {
    (
        $(#[$meta:meta])*
        $vis:vis $Name:ident $type:ty: $len:expr$(, mul $mul:expr)?$(, div $div:expr)?;
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis struct $Name($type);

        impl $crate::arinc424::utils::parseable::Parseable for $Name {
            fn parse(reader: &mut $crate::arinc424::utils::string_reader::StringReader) -> Option<Self> {
                Self::parse_from(reader.read($len)?)
            }

            fn parse_from(data: &str) -> Option<Self> {
                if data.trim().is_empty() {
                    Some(Self(0.0 as $type))
                } else {
                    Some(Self(data.parse::<$type>().ok()? $(* $mul)? $(/ $div)?))
                }
            }
        }
    };

    // Default to f32.
    (
        $(#[$meta:meta])*
        $vis:vis $Name:ident: $len:expr$(, mul $mul:expr)?$(, div $div:expr)?;
    ) => {
        num_parseable! {
            $(#[$meta])*
            $vis $Name f32: $len$(, mul $mul)?$(, div $div)?;
        }
    };
}

#[macro_export]
macro_rules! trimmed_str_parseable {
    (
        $(#[$meta:meta])*
        $vis:vis $Name:ident: $len:expr;
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis struct $Name(String);

        impl $crate::arinc424::utils::parseable::Parseable for $Name {
            fn parse(reader: &mut $crate::arinc424::utils::string_reader::StringReader) -> Option<Self> {
                Some(Self(reader.read($len)?.trim_end().to_owned()))
            }

            fn parse_from(data: &str) -> Option<Self> {
                Some(Self(data.trim_end().to_owned()))
            }
        }
    }
}

#[macro_export]
macro_rules! enum_parseable {
    (
        $(#[$meta:meta])*
        $vis:vis enum: $size:expr => $Name:ident {
            $(
                $(#[$item_meta:meta])*
                $Variant:ident = $value1:expr $(;| $other_value:expr)*
            ),*
            $(,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone)]
        $vis enum $Name {
            $(
                $(#[$item_meta])*
                $Variant
            ),*
        }

        impl $crate::arinc424::utils::parseable::Parseable for $Name {
            fn parse(reader: &mut $crate::arinc424::utils::string_reader::StringReader) -> Option<Self> {
                Self::parse_from(reader.read($size)?)
            }

            fn parse_from(data: &str) -> Option<Self> {
                match data {
                    $(
                        $value1 $(| $other_value)* => Some($Name::$Variant),
                    )*
                    _ => None
                }
            }
        }
    };

    // Default to size 1.
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $(
                $(#[$item_meta:meta])*
                $Variant:ident = $value1:expr $(;| $other_value:expr)*
            ),*
            $(,)+
        }
    ) => {
        enum_parseable! {
            $(#[$meta])*
            $vis enum: 1 => $Name {
                $(
                    $(#[$item_meta])*
                    $Variant = $value1 $(;| $other_value)*,
                )*
            }
        }
    };
}
