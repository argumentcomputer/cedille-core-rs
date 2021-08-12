/// Quantitative Type Theory usage multiplicity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Uses {
  /// erased multiplicity: ⁰x
  None,
  /// linear multiplicity: ¹x
  Once,
  /// affine (used at most once) multiplicity ˚x
  Affi,
  /// unrestricted multiplictiy ⁺x
  Many,
}
