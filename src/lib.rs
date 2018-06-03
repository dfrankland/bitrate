//! Extension traits that add convenience methods for working with bitrates and frequencies.

/// Bits per second
#[derive(Clone, Copy, Debug)]
pub struct Bps<T>(pub T);

/// Hertz
#[derive(Clone, Copy, Debug)]
pub struct Hertz<T>(pub T);

/// KiloHertz
#[derive(Clone, Copy, Debug)]
pub struct KiloHertz<T>(pub T);

/// MegaHertz
#[derive(Clone, Copy, Debug)]
pub struct MegaHertz<T>(pub T);

/// Macro to add extension traits for integer types.
#[macro_export]
macro_rules! bitrate {
    ($(
        ($TXExt:ident, $tx:ident, $txstring:expr),
    )+) => {
        $(
            #[doc = "Extension trait that adds convenience methods to the `"]
            #[doc = $txstring]
            #[doc = "` type."]
            pub trait $TXExt<T> {
                /// Wrap in `Bps`
                fn bps(self) -> Bps<T>;

                /// Wrap in `Hertz`
                fn hz(self) -> Hertz<T>;

                /// Wrap in `KiloHertz`
                fn khz(self) -> KiloHertz<T>;

                /// Wrap in `MegaHertz`
                fn mhz(self) -> MegaHertz<T>;
            }

            impl $TXExt<$tx> for $tx {
                fn bps(self) -> Bps<$tx> {
                    Bps(self)
                }

                fn hz(self) -> Hertz<$tx> {
                    Hertz(self)
                }

                fn khz(self) -> KiloHertz<$tx> {
                    KiloHertz(self)
                }

                fn mhz(self) -> MegaHertz<$tx> {
                    MegaHertz(self)
                }
            }

            impl Into<Hertz<$tx>> for KiloHertz<$tx> {
                fn into(self) -> Hertz<$tx> {
                    Hertz(self.0 * 1_000)
                }
            }

            impl Into<Hertz<$tx>> for MegaHertz<$tx> {
                fn into(self) -> Hertz<$tx> {
                    Hertz(self.0 * 1_000_000)
                }
            }

            impl Into<KiloHertz<$tx>> for MegaHertz<$tx> {
                fn into(self) -> KiloHertz<$tx> {
                    KiloHertz(self.0 * 1_000)
                }
            }
        )+
    }
}

bitrate! {
    (U32BitrateExt, u32, "u32"),
    (U64BitrateExt, u64, "u64"),
    (U128BitrateExt, u128, "u128"),
    (USizeBitrateExt, usize, "usize"),
    (I32BitrateExt, i32, "i32"),
    (I64BitrateExt, i64, "i64"),
    (I128BitrateExt, i128, "i128"),
    (ISizeBitrateExt, isize, "isize"),
}
