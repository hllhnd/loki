//! Types for the C programming language

/// Representation of ISO standards for C, optionally with GNU extension support.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Standard {
    /// The C89 standard, published as ISO/IEC 9899:1990.
    ///
    /// This corresponds to `-std=c89` for GCC(-compatibles).
    #[default]
    C89,
    /// The C99 standard, published as ISO/IEC 9899:1999.
    ///
    /// This corresponds to `-std=c99` for GCC(-compatibles).
    C99,
    /// The C11 standard, published as ISO/IEC 9899:2011.
    ///
    /// This corresponds to `-std=c11` for GCC(-compatibles).
    C11,
    /// The C17 standard, published as ISO/IEC 9899:2018.
    ///
    /// This corresponds to `-std=c17` for GCC(-compatibles).
    C17,
    /// The C23 standard, which will be published as ISO/IEC 9899:2024.
    ///
    /// This corresponds to `-std=c2x` for GCC(-compatibles).
    C23,
    /// The C89 standard, published as ISO/IEC 9899:1990, with GNU C extensions.
    ///
    /// This corresponds to `-std=gnu89` for GCC(-compatibles).
    Gnu89,
    /// The C99 standard, published as ISO/IEC 9899:1999, with GNU C extensions.
    ///
    /// This corresponds to `-std=gnu99` for GCC(-compatibles).
    Gnu99,
    /// The C11 standard, published as ISO/IEC 9899:2011, with GNU C extensions.
    ///
    /// This corresponds to `-std=gnu11` for GCC(-compatibles).
    Gnu11,
    /// The C17 standard, published as ISO/IEC 9899:2018, with GNU C extensions.
    ///
    /// This corresponds to `-std=gnu17` for GCC(-compatibles).
    Gnu17,
    /// The C23 standard, which will be published as ISO/IEC 9899:2024, with GNU C extensions.
    ///
    /// This corresponds to `-std=gnu2x` for GCC(-compatibles).
    Gnu23,
}

impl Standard {
    /// Convert `self` to a GCC-compatible compiler argument setting the represented C standard.
    pub fn to_arg(self) -> &'static str {
        #[rustfmt::skip]
        match self {
            Standard::C89   => "-std=c89",
            Standard::C99   => "-std=c99",
            Standard::C11   => "-std=c11",
            Standard::C17   => "-std=c17",
            Standard::C23   => "-std=c23",
            Standard::Gnu89 => "-std=gnu89",
            Standard::Gnu99 => "-std=gnu99",
            Standard::Gnu11 => "-std=gnu11",
            Standard::Gnu17 => "-std=gnu17",
            Standard::Gnu23 => "-std=gnu23",
        }
    }
}
