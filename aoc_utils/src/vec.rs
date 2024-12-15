use std::{fmt::Debug, ops::Add};

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

/// Implements the `Add` trait for `Vec2<T>`.
///
/// This allows adding two instances of `Vec2<T>` element-wise, returning a new `Vec2<T>`.
///
/// # Generic Requirements
///
/// - `T` must implement the following traits:
///   - `TryFrom<i64>`: Ensures conversion from `i64` when needed.
///   - `TryInto<usize>`: Supports casting to `usize` for compatibility.
///   - `Add<Output = T>`: Supports addition.
///   - `Debug`: Enables debug output.
///   - `Copy`: Allows value copying.
///
/// - The associated error types for `TryFrom<i64>` and `TryInto<usize>` must also implement `Debug`.
///
/// # Element-Wise Addition
///
/// - `x` is added with `other.x`.
/// - `y` is added with `other.y`.
///
/// # Example
///
/// ```rust
/// use aoc_utils::vec::Vec2;
///
/// let v1: Vec2<i32> = Vec2 { x: 10, y: 20 };
/// let v2: Vec2<i32> = Vec2 { x: 5, y: 15 };
/// let result = v1 + v2;
///
/// assert_eq!(result, Vec2 { x: 15, y: 35 });
/// ```
impl<T> Add for Vec2<T>
where
    T: TryFrom<i64> + TryInto<usize> + Add<Output = T> + Debug + Copy,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    /// Adds two `Vec2<T>` instances element-wise.
    ///
    /// - Returns a new `Vec2<T>` where:
    ///   - `x = self.x + other.x`
    ///   - `y = self.y + other.y`
    ///
    /// # Panics
    ///
    /// This function **does not panic**, assuming `T` supports safe addition.
    fn add(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implements the `Add` trait for adding a tuple `(T, T)` to a `Vec2<T>`.
///
/// This allows adding a 2-tuple directly to a `Vec2<T>` instance, performing element-wise addition.
///
/// # Generic Requirements
///
/// - `T` must implement the following traits:
///   - `TryFrom<i64>`: Enables conversion from `i64`.
///   - `TryInto<usize>`: Supports casting to `usize`.
///   - `Add<Output = T>`: Supports addition.
///   - `Debug`: Required for debug output.
///   - `Copy`: Required for copying values.
///
/// - The associated error types for `TryFrom<i64>` and `TryInto<usize>` must also implement `Debug`.
///
/// # Element-Wise Addition
///
/// - `x` is added with `other.0`
/// - `y` is added with `other.1`
///
/// The result is a new `Vec2<T>` containing the summed values.
///
/// # Example
///
/// ```rust
/// use aoc_utils::vec::Vec2;
///
/// let vec = Vec2 { x: 10, y: 20 };
/// let tuple = (5, 15);
/// let result = vec + tuple;
///
/// assert_eq!(result, Vec2 { x: 15, y: 35 });
/// ```
///
/// # Panics
///
/// This implementation **does not panic**, assuming `T` supports safe addition.
impl<T> Add<(T, T)> for Vec2<T>
where
    T: TryFrom<i64> + TryInto<usize> + Add<Output = T> + Debug + Copy,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    /// Adds a tuple `(T, T)` to a `Vec2<T>` element-wise.
    ///
    /// - Returns a new `Vec2<T>` where:
    ///   - `x = self.x + other.0`
    ///   - `y = self.y + other.1`
    ///
    /// # Panics
    ///
    /// This function **does not panic**, assuming `T` supports safe addition.
    fn add(self, other: (T, T)) -> Self::Output {
        Vec2 {
            x: self.x + other.0,
            y: self.y + other.1,
        }
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

/// Implements the `Add` trait for `Vec3<T>`.
///
/// This enables element-wise addition of two `Vec3<T>` instances, producing a new `Vec3<T>`.
///
/// # Generic Requirements
///
/// - `T` must implement the following traits:
///   - `TryFrom<i64>`: Allows type conversion from `i64`.
///   - `TryInto<usize>`: Enables casting to `usize`.
///   - `Add<Output = T>`: Supports addition.
///   - `Debug`: Required for debug output.
///   - `Copy`: Allows value copying.
///
/// - The associated error types for `TryFrom<i64>` and `TryInto<usize>` must also implement `Debug`.
///
/// # Element-Wise Addition
///
/// - `x` is added with `other.x`
/// - `y` is added with `other.y`
/// - `z` is added with `other.z`
///
/// The result is a new `Vec3<T>` containing the summed values.
///
/// # Example
///
/// ```rust
/// use aoc_utils::vec::Vec3;
///
/// let vec1 = Vec3 { x: 10, y: 20, z: 30 };
/// let vec2 = Vec3 { x: 5, y: 15, z: 25 };
/// let result = vec1 + vec2;
///
/// assert_eq!(result, Vec3 { x: 15, y: 35, z: 55 });
/// ```
///
/// # Panics
///
/// This implementation **does not panic**, provided `T` supports safe addition.
impl<T> Add for Vec3<T>
where
    T: TryFrom<i64> + TryInto<usize> + Add<Output = T> + Debug + Copy,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    /// Adds two `Vec3<T>` instances element-wise.
    ///
    /// - Returns a new `Vec3<T>` where:
    ///   - `x = self.x + other.x`
    ///   - `y = self.y + other.y`
    ///   - `z = self.z + other.z`
    ///
    /// # Panics
    ///
    /// This function **does not panic**, assuming `T` supports safe addition.
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Implements the `Add` trait for adding a tuple `(T, T, T)` to a `Vec3<T>`.
///
/// This allows adding a 3-tuple directly to a `Vec3<T>` instance, performing element-wise addition.
///
/// # Generic Requirements
///
/// - `T` must implement the following traits:
///   - `TryFrom<i64>`: Allows type conversion from `i64`.
///   - `TryInto<usize>`: Enables casting to `usize`.
///   - `Add<Output = T>`: Supports addition.
///   - `Debug`: Required for debug output.
///   - `Copy`: Required to allow value copying.
///
/// - The associated error types for `TryFrom<i64>` and `TryInto<usize>` must also implement `Debug`.
///
/// # Element-Wise Addition
///
/// - `x` is added with `other.0`
/// - `y` is added with `other.1`
/// - `z` is added with `other.2`
///
/// The result is a new `Vec3<T>` containing the summed values.
///
/// # Example
///
/// ```rust
/// use aoc_utils::vec::Vec3;
///
/// let vec = Vec3 { x: 10, y: 20, z: 30 };
/// let tuple = (5, 15, 25);
/// let result = vec + tuple;
///
/// assert_eq!(result, Vec3 { x: 15, y: 35, z: 55 });
/// ```
///
/// # Panics
///
/// This implementation **does not panic**, provided `T` supports safe addition.
impl<T> Add<(T, T, T)> for Vec3<T>
where
    T: TryFrom<i64> + TryInto<usize> + Add<Output = T> + Debug + Copy,
    <T as TryFrom<i64>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    /// Adds a tuple `(T, T, T)` to a `Vec3<T>` element-wise.
    ///
    /// - Returns a new `Vec3<T>` where:
    ///   - `x = self.x + other.0`
    ///   - `y = self.y + other.1`
    ///   - `z = self.z + other.2`
    ///
    /// # Panics
    ///
    /// This function **does not panic**, assuming `T` supports safe addition.
    fn add(self, other: (T, T, T)) -> Self::Output {
        Vec3 {
            x: self.x + other.0,
            y: self.y + other.1,
            z: self.z + other.2,
        }
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

    mod vec2_tests {
        use super::*;

        #[test]
        fn positive_i32_values() {
            let vec: Vec2<i32> = (10, 20).into();
            assert_eq!(vec, Vec2 { x: 10, y: 20 });
            assert_eq!(vec.i(), 20_usize);
            assert_eq!(vec.j(), 10_usize);
        }

        #[test]
        fn negative_i32_values() {
            let vec: Vec2<i32> = (-15, -25).into();
            assert_eq!(vec, Vec2 { x: -15, y: -25 });
        }

        #[test]
        fn positive_u32_values() {
            let vec: Vec2<u32> = (42, 100).into();
            assert_eq!(vec, Vec2 { x: 42, y: 100 });
            assert_eq!(vec.i(), 100_usize);
            assert_eq!(vec.j(), 42_usize);
        }

        #[test]
        #[should_panic(expected = "Failed to convert x")]
        fn negative_u32_values_panic() {
            let _vec: Vec2<u32> = (-5, 10).into();
        }

        #[test]
        #[should_panic(expected = "Failed to convert x to usize")]
        fn usize_conversion_panic() {
            let _vec: Vec2<i32> = (-5, 10).into();
            _vec.j();
        }

        #[test]
        fn large_usize_values() {
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

        // Add trait tests
        #[test]
        fn add_vec2() {
            let v1: Vec2<i32> = Vec2 { x: 10, y: 20 };
            let v2: Vec2<i32> = Vec2 { x: 5, y: 15 };
            let result = v1 + v2;
            assert_eq!(result, Vec2 { x: 15, y: 35 });
        }

        #[test]
        fn add_tuple_to_vec2() {
            let vec: Vec2<i32> = Vec2 { x: 10, y: 20 };
            let result = vec + (5, 15);
            assert_eq!(result, Vec2 { x: 15, y: 35 });
        }
    }

    mod vec3_tests {
        use super::*;

        #[test]
        fn positive_i32_values() {
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
        fn negative_i32_values() {
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
        fn out_of_range_panic() {
            let _vec: Vec3<i8> = (10, 20, 128).into(); // i8 can't hold 128
        }

        #[test]
        #[should_panic(expected = "Failed to convert z to usize")]
        fn usize_conversion_panic() {
            let _vec: Vec3<i32> = (10, 20, -128).into();
            _vec.k();
        }

        #[test]
        fn large_usize_values() {
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
        fn from_i64_values() {
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
        fn from_i32_values() {
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
        fn from_u32_values() {
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

        // Add trait tests
        #[test]
        fn add_vec3() {
            let v1: Vec3<i32> = Vec3 {
                x: 10,
                y: 20,
                z: 30,
            };
            let v2: Vec3<i32> = Vec3 { x: 5, y: 15, z: 25 };
            let result = v1 + v2;
            assert_eq!(
                result,
                Vec3 {
                    x: 15,
                    y: 35,
                    z: 55
                }
            );
        }

        #[test]
        fn add_tuple_to_vec3() {
            let vec: Vec3<i32> = Vec3 {
                x: 10,
                y: 20,
                z: 30,
            };
            let result = vec + (5, 15, 25);
            assert_eq!(
                result,
                Vec3 {
                    x: 15,
                    y: 35,
                    z: 55
                }
            );
        }
    }
}
