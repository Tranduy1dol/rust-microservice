/// Validates that a password contains at least one uppercase letter and at least one ASCII digit.
///
/// # Returns
///
/// `Ok(())` if the password contains at least one uppercase letter and at least one ASCII digit, `Err(validator::ValidationError)` otherwise.
///
/// # Examples
///
/// ```
/// use crate::validator::validate_password_strength;
///
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
