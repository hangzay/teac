//! Type definitions used throughout the AST.
//!
//! This module defines the source-position alias and all type-specifier
//! nodes that appear in variable declarations, function parameters, and
//! return-type annotations.

/// Byte offset (or character index) into the source text.
/// Used to track where each AST node originated for error reporting.
pub type Pos = usize;

/// Built-in primitive types supported by the language.
#[derive(Debug, Clone)]
pub enum BuiltIn {
    /// The 32-bit signed integer type (`i32`).
    Int,
    /// The 32-bit floating-point type (`f32`).
    Float,
}

/// A fixed-length array type specifier, e.g. `[i32; 4]` or `[[i32; 4]; 3]`.
#[derive(Debug, Clone)]
pub struct ArrayTypeSpec {
    /// The element type (may itself be an array type for multi-dimensional arrays).
    pub element_type: Box<TypeSpecifier>,
    /// The number of elements.
    pub len: usize,
}

/// The inner representation of a type specifier, distinguishing between
/// built-in primitives, user-defined composite types, reference types, and array types.
#[derive(Debug, Clone)]
pub enum TypeSpecifierInner {
    /// A primitive type such as `i32` or `f32`.
    BuiltIn(BuiltIn),
    /// A user-defined struct or composite type, identified by name.
    Composite(String),
    /// A reference to a slice of another type (e.g., `&[i32]`).
    Reference(Box<TypeSpecifier>),
    /// A fixed-length array type (e.g., `[i32; 4]` or `[[i32; 4]; 3]`).
    Array(Box<ArrayTypeSpec>),
}

/// A fully-annotated type specifier, pairing the type's inner representation
/// with the source position where it appears.
#[derive(Debug, Clone)]
pub struct TypeSpecifier {
    /// Source position of this type specifier.
    pub pos: Pos,
    /// The actual type information (built-in, composite, or reference).
    pub inner: TypeSpecifierInner,
}
