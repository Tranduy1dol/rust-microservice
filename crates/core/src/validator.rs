use rust_decimal::Decimal;
use validator::ValidationError;

/// Validates that a password contains at least one uppercase letter and at least one ASCII digit.
///
/// # Examples
///
/// ```
/// assert!(validate_password_strength("Abc1").is_ok());
/// assert!(validate_password_strength("abc1").is_err()); // missing uppercase
/// assert!(validate_password_strength("Abcd").is_err()); // missing digit
/// ```
pub fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if has_uppercase && has_digit {
        Ok(())
    } else {
        Err(validator::ValidationError::new("password_strength"))
    }
}

/// Ensures the given `price` is strictly greater than zero.
///
/// Returns `Ok(())` when `price` is greater than zero; returns a `ValidationError` with the
/// code `"price_must_be_positive"` otherwise.
///
/// # Examples
///
/// ```
/// use rust_decimal::Decimal;
///
/// let positive = Decimal::new(100, 2); // 1.00
/// assert!(validate_price_positive(&positive).is_ok());
///
/// let zero = Decimal::ZERO;
/// assert!(validate_price_positive(&zero).is_err());
///
/// let negative = Decimal::new(-50, 2); // -0.50
/// assert!(validate_price_positive(&negative).is_err());
/// ```
pub fn validate_price_positive(price: &Decimal) -> Result<(), ValidationError> {
    if *price > Decimal::ZERO {
        Ok(())
    } else {
        Err(ValidationError::new("price_must_be_positive"))
    }
}
