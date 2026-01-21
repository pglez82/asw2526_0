use crate::error::ErrorResponse;

pub fn check_api_version(version: &str) -> Result<(), ErrorResponse> {
    const SUPPORTED_VERSION: &str = "v1";
    if version != SUPPORTED_VERSION {
        Err(ErrorResponse::error(
            &format!(
                "Unsupported API version: {}. Supported version is {}",
                version, SUPPORTED_VERSION
            ),
            Some(version.to_string()),
            None,
        ))
    } else {
        Ok(())
    }
}
