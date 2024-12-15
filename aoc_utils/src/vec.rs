use std::fmt::Debug;

/// A 2D vector with generic coordinates.
///
/// This struct represents a 2D point or vector with components `x` and `y`.
/// It supports various numeric types (e.g., `i32`, `i64`, `u32`, `usize`)
/// as long as they implement `TryFrom<i64>` and `Debug`.
///
/// # Construction
///
/// `Vec2` does not provide direct constructors. Instead, you can create a `Vec2`
/// from tuples using the `From` trait. Any pair of integers that can be
/// represented by `i64` and then converted into `T` can be used:
///
/// ```
/// # use aoc_utils::vec::Vec2;
/// // From a tuple of `i32`:
/// let v: Vec2<i32> = (10_i32, -20_i32).into();
/// assert_eq!(v.x, 10);
/// assert_eq!(v.y, -20);
///
/// // From a tuple of `u32`:
/// let v: Vec2<u32> = (42_u32, 100_u32).into();
/// assert_eq!(v.x, 42);
/// assert_eq!(v.y, 100);
/// ```
///
/// # Panics
///
/// The `From` conversions panic if the conversion from `i64` to `T` fails.
/// This can occur if the value is out of `T`'s representable range
/// (e.g., passing a negative number for an unsigned type or a large number
/// that doesn't fit in `T`).
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Vec2<T>
where
    T: TryFrom<i64> + TryInto<usize> + Debug,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    /// The x-coordinate.
    pub x: T,
    /// The y-coordinate.
    pub y: T,
}

impl<T> Vec2<T>
where
    T: TryFrom<i64> + TryInto<usize> + Debug + Copy,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    /// Returns the row index (`i`) as `usize`.
    ///
    /// This is a shorthand accessor for the row (y-coordinate).
    /// Equivalent to `self.y` cast to `usize`.
    ///
    /// # Panics
    ///
    /// Panics if the conversion to `usize` fails.
    pub fn i(&self) -> usize {
        self.y.try_into().expect("Failed to convert y to usize")
    }

    /// Returns the column index (`j`) as `usize`.
    ///
    /// This is a shorthand accessor for the column (x-coordinate).
    /// Equivalent to `self.x` cast to `usize`.
    ///
    /// # Panics
    ///
    /// Panics if the conversion to `usize` fails.
    pub fn j(&self) -> usize {
        self.x.try_into().expect("Failed to convert x to usize")
    }
}

/// A 3D vector with generic coordinates.
///
/// This struct represents a 3D point or vector with components `x`, `y`, and `z`.
/// It supports various numeric types (e.g., `i32`, `i64`, `u32`, `usize`)
/// as long as they implement `TryFrom<i64>` and `Debug`.
///
/// # Construction
///
/// `Vec3` does not provide direct constructors. Instead, you can create a `Vec3`
/// from tuples using the `From` trait. Any triplet of integers that can be
/// represented by `i64` and then converted into `T` can be used:
///
/// ```
/// # use aoc_utils::vec::Vec3;
/// // From a tuple of `i32`:
/// let v: Vec3<i32> = (10_i32, -20_i32, 30_i32).into();
/// assert_eq!(v.x, 10);
/// assert_eq!(v.y, -20);
/// assert_eq!(v.z, 30);
///
/// // From a tuple of `u64`:
/// let v: Vec3<i64> = (42_u64, 84_u64, 168_u64).into();
/// assert_eq!(v.x, 42);
/// assert_eq!(v.y, 84);
/// assert_eq!(v.z, 168);
/// ```
///
/// # Panics
///
/// The `From` conversions panic if the conversion from `i64` to `T` fails.
/// This might happen if `T` is an unsigned type and you provide a negative value,
/// or if the value exceeds `T`'s representable range.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Vec3<T>
where
    T: TryFrom<i64> + TryInto<usize> + Debug,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    /// The x-coordinate.
    pub x: T,
    /// The y-coordinate.
    pub y: T,
    /// The z-coordinate.
    pub z: T,
}

impl<T> Vec3<T>
where
    T: TryFrom<i64> + TryInto<usize> + Debug + Copy,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    /// Returns the row index (`i`) as `usize`.
    ///
    /// This is a shorthand accessor for the row (y-coordinate).
    /// Equivalent to `self.y` cast to `usize`.
    ///
    /// # Panics
    ///
    /// Panics if the conversion to `usize` fails.
    pub fn i(&self) -> usize {
        self.y.try_into().expect("Failed to convert y to usize")
    }

    /// Returns the column index (`j`) as `usize`.
    ///
    /// This is a shorthand accessor for the column (x-coordinate).
    /// Equivalent to `self.x` cast to `usize`.
    ///
    /// # Panics
    ///
    /// Panics if the conversion to `usize` fails.
    pub fn j(&self) -> usize {
        self.x.try_into().expect("Failed to convert x to usize")
    }

    /// Returns the depth index (`k`) as `usize`.
    ///
    /// This is a shorthand accessor for the depth (z-coordinate).
    /// Equivalent to `self.z` cast to `usize`.
    ///
    /// # Panics
    ///
    /// Panics if the conversion to `usize` fails.
    pub fn k(&self) -> usize {
        self.z.try_into().expect("Failed to convert z to usize")
    }
}

// Macros to implement From for multiple integer types
macro_rules! impl_from_tuple_for_vec2 {
    ($($from_type:ty),+) => {
        $(
            impl<T> From<($from_type, $from_type)> for Vec2<T>
            where
                T: TryFrom<i64> + TryInto<usize> + Debug,
                <T as TryFrom<i64>>::Error: Debug,
                <T as TryInto<usize>>::Error: Debug,
            {
                #[doc = concat!("Converts a tuple `(x, y)` of type `", stringify!($from_type), "` into a `Vec2<T>`.")]
                #[doc = ""]
                #[doc = "# Panics"]
                #[doc = ""]
                #[doc = "Panics if conversion from `i64` to `T` fails. For example:"]
                #[doc = "- If `T` is unsigned and `x` or `y` is negative."]
                #[doc = "- If `x` or `y` cannot fit into `T`."]
                #[doc = ""]
                #[doc = "# Examples"]
                #[doc = ""]
                #[doc = "```"]
                #[doc = "# use aoc_utils::vec::Vec2;"]
                #[doc = "let v: Vec2<i32> = (10_i32, -20_i32).into();"]
                #[doc = "assert_eq!(v.x, 10);"]
                #[doc = "assert_eq!(v.y, -20);"]
                #[doc = "```"]
                fn from((x, y): ($from_type, $from_type)) -> Self {
                    let x = x as i64;
                    let y = y as i64;

                    let x = T::try_from(x).expect("Failed to convert x");
                    let y = T::try_from(y).expect("Failed to convert y");
                    Vec2 { x, y }
                }
            }
        )+
    };
}

macro_rules! impl_from_tuple_for_vec3 {
    ($($from_type:ty),+) => {
        $(
            impl<T> From<($from_type, $from_type, $from_type)> for Vec3<T>
            where
                T: TryFrom<i64> + TryInto<usize> + Debug,
                <T as TryFrom<i64>>::Error: Debug,
                <T as TryInto<usize>>::Error: Debug,
            {
                #[doc = concat!("Converts a tuple `(x, y, z)` of type `", stringify!($from_type),"` into a `Vec3<T>`.")]
                #[doc = ""]
                #[doc = "# Panics"]
                #[doc = ""]
                #[doc = "Panics if conversion from `i64` to `T` fails. For example:"]
                #[doc = "- If `T` is unsigned and `x`, `y`, or `z` is negative."]
                #[doc = "- If `x`, `y`, or `z` cannot fit into `T`."]
                #[doc = ""]
                #[doc = "# Examples"]
                #[doc = ""]
                #[doc = "```"]
                #[doc = "# use aoc_utils::vec::Vec3;"]
                #[doc = "let v: Vec3<i64> = (10_i64, 20_i64, 30_i64).into();"]
                #[doc = "assert_eq!(v.x, 10);"]
                #[doc = "assert_eq!(v.y, 20);"]
                #[doc = "assert_eq!(v.z, 30);"]
                #[doc = "```"]
                fn from((x, y, z): ($from_type, $from_type, $from_type)) -> Self {
                    let x = x as i64;
                    let y = y as i64;
                    let z = z as i64;

                    let x = T::try_from(x).expect("Failed to convert x");
                    let y = T::try_from(y).expect("Failed to convert y");
                    let z = T::try_from(z).expect("Failed to convert z");
                    Vec3 { x, y, z }
                }
            }
        )+
    };
}

// Implement From for a variety of integral types
impl_from_tuple_for_vec2!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
impl_from_tuple_for_vec3!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    // Vec2 Tests
    #[test]
    fn test_vec2_i32_positive() {
        let vec: Vec2<i32> = (10, 20).into();
        assert_eq!(vec, Vec2 { x: 10, y: 20 });
        assert_eq!(vec.i(), 20_usize);
        assert_eq!(vec.j(), 10_usize);
    }

    #[test]
    fn test_vec2_i32_negative() {
        let vec: Vec2<i32> = (-15, -25).into();
        assert_eq!(vec, Vec2 { x: -15, y: -25 });
    }

    #[test]
    fn test_vec2_u32_positive() {
        let vec: Vec2<u32> = (42, 100).into();
        assert_eq!(vec, Vec2 { x: 42, y: 100 });
        assert_eq!(vec.i(), 100_usize);
        assert_eq!(vec.j(), 42_usize);
    }

    #[test]
    #[should_panic(expected = "Failed to convert x")]
    fn test_vec2_u32_negative_panic() {
        let _vec: Vec2<u32> = (-5, 10).into(); // Should panic
    }

    #[test]
    #[should_panic(expected = "Failed to convert x to usize")]
    fn test_vec2_usize_conversion_panic() {
        let _vec: Vec2<i32> = (-5, 10).into();
        _vec.j(); // Should panic
    }

    #[test]
    fn test_vec2_large_values() {
        let vec: Vec2<usize> = (1_000_000, 2_000_000).into();
        assert_eq!(
            vec,
            Vec2 {
                x: 1_000_000,
                y: 2_000_000
            }
        );
        assert_eq!(vec.i(), 2_000_000_usize);
        assert_eq!(vec.j(), 1_000_000_usize);
    }

    // Vec3 Tests
    #[test]
    fn test_vec3_i32_positive() {
        let vec: Vec3<i32> = (10, 20, 30).into();
        assert_eq!(
            vec,
            Vec3 {
                x: 10,
                y: 20,
                z: 30
            }
        );
        assert_eq!(vec.i(), 20_usize);
        assert_eq!(vec.j(), 10_usize);
        assert_eq!(vec.k(), 30_usize);
    }

    #[test]
    fn test_vec3_i32_negative() {
        let vec: Vec3<i32> = (-15, -25, -35).into();
        assert_eq!(
            vec,
            Vec3 {
                x: -15,
                y: -25,
                z: -35
            }
        );
    }

    #[test]
    #[should_panic(expected = "Failed to convert z")]
    fn test_vec3_out_of_range_panic() {
        let _vec: Vec3<i8> = (10, 20, 128).into(); // i8 can't hold 128, should panic
    }

    #[test]
    #[should_panic(expected = "Failed to convert z to usize")]
    fn test_vec3_usize_conversion_panic() {
        let _vec: Vec3<i32> = (10, 20, -128).into();
        _vec.k(); // Should panic
    }

    #[test]
    fn test_vec3_large_values() {
        let vec: Vec3<usize> = (1_000_000, 2_000_000, 3_000_000).into();
        assert_eq!(
            vec,
            Vec3 {
                x: 1_000_000,
                y: 2_000_000,
                z: 3_000_000
            }
        );
        assert_eq!(vec.i(), 2_000_000_usize);
        assert_eq!(vec.j(), 1_000_000_usize);
        assert_eq!(vec.k(), 3_000_000_usize);
    }

    #[test]
    fn test_vec3_from_i64() {
        let vec: Vec3<i64> = (10_i64, 20_i64, 30_i64).into();
        assert_eq!(
            vec,
            Vec3 {
                x: 10,
                y: 20,
                z: 30
            }
        );
    }

    #[test]
    fn test_vec3_from_i32() {
        let vec: Vec3<i32> = (10_i32, -20_i32, 30_i32).into();
        assert_eq!(
            vec,
            Vec3 {
                x: 10,
                y: -20,
                z: 30
            }
        );
    }

    #[test]
    fn test_vec3_from_u32() {
        let vec: Vec3<u32> = (42_u32, 84_u32, 168_u32).into();
        assert_eq!(
            vec,
            Vec3 {
                x: 42,
                y: 84,
                z: 168
            }
        );
    }
}
