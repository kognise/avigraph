#[macro_export]
macro_rules! autoparse_struct {
    (
        $(#[$meta:meta])*
        $vis:vis $Name:ident {
            $(
                $(#(skip $count:expr),)*
                $(#(continuation $cont_count:expr $(, skip $count2:expr)*),)?
                $field_vis:vis $field:ident: $Type:ty
            ),*
            $(,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis struct $Name {
            $($field_vis $field: $Type),*
        }

        impl $crate::arinc424::utils::parseable::Parseable for $Name {
            fn parse(reader: &mut $crate::arinc424::utils::string_reader::StringReader) -> Option<Self> {
                $(
                    $(reader.skip($count);)*
                    $(
                        // Super janky but no real continuation record parsing for now.
                        if reader.read($cont_count)?.parse::<u8>().ok()? != 0 {
                            println!("[skipping continuation record]");
                            return None;
                        } else {
                            $(reader.skip($count2);)*
                        }
                    )?

                    // For debugging:
                    // println!("{} -> {}", stringify!($Name), stringify!($Type));

                    let $field = <$Type>::parse(reader)?;
                )*
                Some(Self {
                    $($field),*
                })
            }
        }
    }
}
