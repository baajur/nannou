//! Parameters which a **Drawing** instance may use to describe certain properties of a drawing.
//!
//! Each time a new method is chained onto a **Drawing** instance, it uses the given values to set
//! one or more properties for the drawing.
//!
//! Each **Drawing** instance is associated with a specific **Node** in the geometry graph and has
//! a unique **node::Index** to simplify this.

pub mod color;
pub mod fill;
pub mod spatial;
pub mod stroke;

pub use self::color::SetColor;
pub use self::fill::SetFill;
pub use self::spatial::dimension::SetDimensions;
pub use self::spatial::orientation::SetOrientation;
pub use self::spatial::position::SetPosition;
pub use self::stroke::SetStroke;

/// The scalar type used for the color channel values.
pub type ColorScalar = crate::color::DefaultScalar;

/// The RGBA type used by the `Common` params.
pub type Srgba = color::DefaultSrgba;

/// The RGBA type used by the `Common` params.
pub type LinSrgba = color::DefaultLinSrgba;
