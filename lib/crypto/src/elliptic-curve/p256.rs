//! NIST P-256 elliptic curve.
//!
//! This curve is also known as prime256v1 (ANSI X9.62) and secp256r1 (SECG)
//! and is specified in [NIST SP 800-186]: Recommendations for Discrete
//! Logarithm-based Cryptography: Elliptic Curve Domain Parameters.
//!
//! It's included in the US National Security Agency's "Suite B" and is widely
//! used in protocols like TLS and the associated X.509 PKI.
//!
//! Its equation is `y² = x³ - 3x + b` over a ~256-bit prime field where `b` is
//! the "verifiably random" constant:
//!
//! ```text
//! b = 41058363725152142129326129780047268409114441015993725554835256314039467401291
//! ```
//!
//! NOTE: the specific origins of this constant have never been fully disclosed
//!   (it is the SHA-1 digest of an unknown NSA-selected constant)*
//!
//! [NIST SP 800-186]: https://csrc.nist.gov/publications/detail/sp/800-186/final

use bigint::U256;

use super::{
    curve::{Curve, PrimeCurveParams},
    field::FieldElement,
};

/// The prime `p` that specifies the size of the finite field `𝔽p`.
const ORDER_HEX: &str =
    "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff";

/// Order of NIST P-256's elliptic curve group (i.e. scalar modulus) serialized
/// as hexadecimal.
///
/// ```text
/// n = FFFFFFFF 00000000 FFFFFFFF FFFFFFFF BCE6FAAD A7179E84 F3B9CAC2 FC632551
/// ```
///
/// # Calculating the order
///
/// One way to calculate the order is with `Pari/GP`:
///
/// ```text
/// p = (2^224) * (2^32 - 1) + 2^192 + 2^96 - 1
/// b = 41058363725152142129326129780047268409114441015993725554835256314039467401291
/// E = ellinit([Mod(-3, p), Mod(b, p)])
/// default(parisize, 120000000)
/// n = ellsea(E)
/// isprime(n)
/// ```
const SUBGROUP_ORDER_HEX: &str =
    "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";

/// See the [module documentation][self].
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct P256;

impl Curve for P256 {
    /// Constant representing the modulus
    /// `p = 2^{224}(2^{32} − 1) + 2^{192} + 2^{96} − 1`
    const ORDER: U256 = U256::from_be_hex(ORDER_HEX);
}

/// Adapted from [NIST SP 800-186] § G.1.2: Curve P-256.
///
/// [NIST SP 800-186]: https://csrc.nist.gov/publications/detail/sp/800-186/final
impl PrimeCurveParams for P256 {
    type FieldElement = FieldElement;

    /// a = -3
    const EQUATION_A: FieldElement = FieldElement::from_hex(
        "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
    );
    const EQUATION_B: FieldElement = FieldElement::from_hex(
        "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
    );
    /// Base point of P-256.
    ///
    /// Defined in NIST SP 800-186 § G.1.2:
    ///
    /// ```text
    /// Gₓ = 6b17d1f2 e12c4247 f8bce6e5 63a440f2 77037d81 2deb33a0 f4a13945 d898c296
    /// Gᵧ = 4fe342e2 fe1a7f9b 8ee7eb4a 7c0f9e16 2bce3357 6b315ece cbb64068 37bf51f5
    /// ```
    const GENERATOR: (FieldElement, FieldElement) = (
        FieldElement::from_hex(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
        ),
        FieldElement::from_hex(
            "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
        ),
    );
    /// Order of NIST P-256's elliptic curve subgroup generated by
    /// [`Self::GENERATOR`].
    const SUBGROUP_ORDER: U256 = U256::from_be_hex(SUBGROUP_ORDER_HEX);
}
