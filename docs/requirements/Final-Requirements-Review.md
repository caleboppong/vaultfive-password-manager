# VaultFive Final Requirements Review

## Reviewer

**Name:** Fardin Arik  
**Role:** Requirements & Business Analysis  
**Date:** 16 September 2026

## 1. Purpose of the Review

The purpose of this final review is to assess whether the completed VaultFive desktop password manager satisfies the functional and non-functional requirements agreed during the project.

The review considers the final VaultFive application alongside the Requirements Baseline, User Stories and MoSCoW prioritisation, and the completed testing evidence.

## 2. Functional Requirements Review

The final VaultFive application implements the main functional requirements identified during the requirements stage.

The completed functionality includes:

- Initial vault setup using a master password
- Authentication using the master password
- Vault lock/logout
- Adding credentials
- Viewing stored credentials
- Editing credentials
- Deleting credentials
- Searching stored credentials
- Password masking by default
- Password reveal and re-hide
- Secure password generation

The Must requirements were prioritised before optional functionality. This helped ensure that the essential password-management features were completed within the available project schedule.

The password generator, which was identified as a Should requirement, was also successfully implemented.

## 3. Requirements and Testing

The project requirements were supported by a structured test plan containing 18 test cases.

Testing covered:

- Valid and invalid vault setup
- Correct and incorrect authentication
- Vault locking
- Credential CRUD operations
- Search functionality
- Password masking and reveal
- Password generation
- Credential persistence
- Unauthenticated access
- Keyboard accessibility

The completed test results recorded all 18 planned test cases as PASS.

This provides evidence that the main functional requirements were not only implemented but were also checked against expected outcomes.

## 4. Non-Functional Requirements Review

### 4.1 Security

VaultFive uses Argon2id for master-password protection and XChaCha20Poly1305 authenticated encryption for stored credential information.

Credential information is stored locally using SQLite rather than being stored as readable plaintext.

### 4.2 Usability

The application provides a clear workflow for vault setup, login and credential management.

Forms include appropriate labels, validation feedback and clear actions for adding, editing and deleting credentials.

### 4.3 Accessibility

Keyboard navigation was reviewed as part of TC18. Form labels and visible feedback messages are also used throughout the application to support accessibility and usability.

### 4.4 Reliability

Credential persistence was tested to confirm that stored credential information remains available after the application is closed and reopened.

### 4.5 Maintainability

The project separates the Svelte/TypeScript frontend from the Rust backend. Supporting project documentation is also organised into appropriate folders within the GitHub repository.

This separation supports clearer maintenance and future development.

## 5. Requirements Traceability

The final project demonstrates traceability across the development process:

**Requirements → User Stories → Implementation → Test Cases → Test Results**

This provides a clear connection between what the application was required to do, what was implemented and how the completed functionality was verified.

## 6. Deferred and Future Improvements

Although the main requirements have been delivered, possible future improvements include:

- Additional usability testing with independent users
- More advanced password-strength feedback
- Backup and recovery functionality
- Further security review
- Possible browser or cloud integration, subject to additional security and privacy analysis

These improvements are outside the core requirements of the current project but could be considered in future versions of VaultFive.

## 7. Personal Review

As part of my review, I checked the Requirements Baseline, User Stories, MoSCoW prioritisation and the completed test results against the final VaultFive application.

Based on this review, I believe the final application reflects the main requirements agreed by the team, particularly the Must requirements for secure authentication and credential management.

A key strength is the clear traceability between the requirements, implementation and testing, with all 18 planned test cases recorded as passed.

One improvement I would recommend is carrying out additional usability and accessibility testing with independent users to identify improvements that may not have been identified during internal testing.

## 8. Final Conclusion

Based on my final requirements review, the completed VaultFive application meets the main functional and non-functional requirements agreed for the project.

The core password-management functionality has been implemented and tested successfully, while the identified future improvements provide opportunities for further development beyond the current project scope.

---

**Reviewed by:** Fardin Arik  
**Project Role:** Requirements & Business Analysis  
**Date:** 16 September 2026
