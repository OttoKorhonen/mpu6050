use core::{error::Error, fmt};

#[derive(Debug)]
pub enum MPU6050Error<E> {
    I2CError(E),
}

impl<E> fmt::Display for MPU6050Error<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I2CError(e) => write!(f, "I2c error: {:?}", e)
        }
    }
}

impl<E> Error for MPU6050Error<E> where E: fmt::Debug {}

impl <E> From<E> for MPU6050Error<E> {
    fn from(e: E) -> Self {
        Self::I2CError(e)
    }
}