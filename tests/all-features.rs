// paasa/tests/all-features.rs

#[cfg(not(any(feature = "lua", feature = "rust")))]
compile_error!("atleast one feature needs to be enabled. available features: [lua, rust]");

use paasa::TokenTrait;


macro_rules! test_pair {
    ($feature:tt, $extension:expr) => {
        mod $feature {
            use paasa::{parse, parse_with_settings, ParseSettings};
            use paasa::$feature::Token::{self, *};


            #[test]
            fn test_parse() {
                assert_eq!(
                    parse::<Token>(
                        include_str!(
                            concat!(
                                "files/",
                                stringify!($feature),
                                "_input.",
                                $extension
                            )
                        )
                    ),
                    Ok(
                        super::remove_special(
                            include!(
                                concat!(
                                    "files/",
                                    stringify!($feature),
                                    "_output.rs"
                                )
                            )
                        )
                    )
                );
            }

            #[test]
            fn test_parse_with_settings() {
                assert_eq!(
                    parse_with_settings::<Token>(
                        include_str!(
                            concat!(
                                "files/",
                                stringify!($feature),
                                "_input.",
                                $extension
                            )
                        ),
                        ParseSettings::full()
                    ),
                    Ok(
                        include!(
                            concat!(
                                "files/",
                                stringify!($feature),
                                "_output.rs"
                            )
                        )
                    )
                );
            }
        }
    };
}

fn remove_special<T: TokenTrait>(tokens: Vec<T>) -> Vec<T> {
    tokens
        .into_iter()
        .filter(|token| !token.is_special())
        .collect()
}

#[cfg(feature = "lua" )] test_pair!(lua,  "lua");
#[cfg(feature = "rust")] test_pair!(rust, "rs");

