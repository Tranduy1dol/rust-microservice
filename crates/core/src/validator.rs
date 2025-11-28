use rust_decimal::Decimal;
use validator::ValidationError;

/// Ensures a password contains at least one uppercase letter and at least one ASCII digit.
///
/// # Returns
///
/// `Ok(())` if the password contains at least one uppercase letter and at least one ASCII digit, `Err(validator::ValidationError)` otherwise.
///
/// # Examples
///
/// ```ignore
/// use crate::validator::validate_password_strength;
///
/// assert!(validate_password_strength("Abc1").is_ok());
/// assert!(validate_password_strength("abc1").is_err()); // missing uppercase
/// assert!(validate_password_strength("Abcd").is_err()); // missing digit
/// ```ignore
pub fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if has_uppercase && has_digit {
        Ok(())
    } else {
        Err(validator::ValidationError::new("password_strength"))
    }
}

/// Ensures a price value is strictly greater than zero.
///
/// Returns `Ok(())` when `price` is greater than zero; returns a `ValidationError` with the
/// code `"price_must_be_positive"` otherwise.
///
/// # Examples
///
/// ```ignore
/// use rust_decimal::Decimal;
///
/// let positive = Decimal::new(100, 2); // 1.00
/// assert!(validate_price_positive(&positive).is_ok());
///
/// let negative = Decimal::new(-50, 2); // -0.50
/// assert!(validate_price_positive(&negative).is_err());
/// ```ignore
pub fn validate_price_positive(price: &Decimal) -> Result<(), ValidationError> {
    if *price > Decimal::ZERO {
        Ok(())
    } else {
        Err(ValidationError::new("price_must_be_positive"))
    }
}
