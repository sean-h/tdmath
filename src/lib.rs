pub mod vector2;
pub mod vector3;
pub mod vector4;
pub mod matrix4;
pub mod quaternion;
pub mod ray;

pub use self::vector2::*;
pub use self::vector3::*;
pub use self::vector4::*;
pub use self::matrix4::*;
pub use self::quaternion::*;
pub use self::ray::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
