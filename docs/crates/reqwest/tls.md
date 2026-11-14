**reqwest > tls**

# Module: tls

## Contents

**Structs**

- [`Certificate`](#certificate) - Represents a server X509 certificate.
- [`CertificateRevocationList`](#certificaterevocationlist) - Represents a X509 certificate revocation list.
- [`Identity`](#identity) - Represents a private key and X509 cert as a client certificate.
- [`TlsInfo`](#tlsinfo) - Hyper extension carrying extra TLS layer information.
- [`Version`](#version) - A TLS protocol version.

---

## reqwest::tls::Certificate

*Struct*

Represents a server X509 certificate.

**Methods:**

- `fn from_der(der: &[u8]) -> crate::Result<Certificate>` - Create a `Certificate` from a binary DER encoded certificate
- `fn from_pem(pem: &[u8]) -> crate::Result<Certificate>` - Create a `Certificate` from a PEM encoded certificate
- `fn from_pem_bundle(pem_bundle: &[u8]) -> crate::Result<Vec<Certificate>>` - Create a collection of `Certificate`s from a PEM encoded certificate bundle.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## reqwest::tls::CertificateRevocationList

*Struct*

Represents a X509 certificate revocation list.

**Methods:**

- `fn from_pem(pem: &[u8]) -> crate::Result<CertificateRevocationList>` - Parses a PEM encoded CRL.
- `fn from_pem_bundle(pem_bundle: &[u8]) -> crate::Result<Vec<CertificateRevocationList>>` - Creates a collection of `CertificateRevocationList`s from a PEM encoded CRL bundle.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## reqwest::tls::Identity

*Struct*

Represents a private key and X509 cert as a client certificate.

**Methods:**

- `fn from_pem(buf: &[u8]) -> crate::Result<Identity>` - Parses PEM encoded private key and certificate.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## reqwest::tls::TlsInfo

*Struct*

Hyper extension carrying extra TLS layer information.
Made available to clients on responses when `tls_info` is set.

**Methods:**

- `fn peer_certificate(self: &Self) -> Option<&[u8]>` - Get the DER encoded leaf certificate of the peer.
- `fn version(self: &Self) -> Option<Version>` - Get the TLS protocol version negotiated with the peer.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`



## reqwest::tls::Version

*Struct*

A TLS protocol version.

**Tuple Struct**: `()`

**Methods:**


**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`



