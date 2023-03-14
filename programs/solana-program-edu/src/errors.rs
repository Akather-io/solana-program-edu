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
    #[msg("Not enough funds to enroll in course")]
    NotEnoughFunds,
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("Enrollment not completed")]
    EnrollmentNotCompleted,
    #[msg("Certificate already issued")]
    CertificateAlreadyIssued,
}
