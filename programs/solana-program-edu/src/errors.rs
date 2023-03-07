use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorMessages {
    #[msg("Course name is too long")]
    CourseNameTooLong,
    #[msg("Course description is too long")]
    CourseDescriptionTooLong,
    #[msg("Course price must be greater than 0")]
    CoursePriceTooLow,
    #[msg("Invalid course account")]
    InvalidCourseAccount,
}
