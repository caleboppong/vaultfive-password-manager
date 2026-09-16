# VaultFive Final QA and Risk Review

## Reviewer

**Name:** Eray  
**Role:** QA & Risk  
**Date:** 16 September 2026

## 1. Purpose of the Review

The purpose of this final review is to evaluate the quality assurance, testing and risk management activities completed during the VaultFive project.

As part of my QA & Risk role, I participated in testing the VaultFive application with the Project Manager during the testing stage. This review considers the completed Test Plan, Test Results, automated Rust test evidence, Final Build Verification and Risk Register v1.1.

## 2. Functional Testing Review

VaultFive was tested using a structured test plan containing 18 test cases covering functional, negative, security, persistence and basic accessibility requirements.

The completed test cases were:

- TC01 – Initialise Vault
- TC02 – Reject Invalid Vault Setup Input
- TC03 – Authenticate Valid Master Password
- TC04 – Reject Incorrect Master Password
- TC05 – Lock/Logout
- TC06 – Add Valid Credential
- TC07 – Reject Invalid Credential Input
- TC08 – View Stored Credential
- TC09 – Edit Credential
- TC10 – Delete Credential
- TC11 – Search Credentials
- TC12 – Search No Match
- TC13 – Password Masked by Default
- TC14 – Reveal/Re-hide Password
- TC15 – Password Generator
- TC16 – Credential Persistence
- TC17 – Unauthenticated Vault Access
- TC18 – Basic Accessibility/Keyboard Review

The final recorded results show that all 18 planned test cases passed.

Testing included both positive and negative scenarios. These included valid and invalid vault setup, correct and incorrect master passwords, credential validation, CRUD operations, successful and unsuccessful searches, password masking and reveal, password generation, persistence, locked-vault protection and keyboard navigation.

## 3. Security and Automated Testing Review

Security-related testing covered:

- Incorrect master-password rejection
- Vault locking
- Password masking by default
- Password reveal and re-hide
- Unauthenticated credential access protection

The Rust backend also contains two automated tests relating to vault session-state behaviour.

The final automated test run recorded:

- **2 tests passed**
- **0 tests failed**

These tests verify that a locked vault rejects session-key access and an unlocked vault allows session-key access.

The automated tests complement the manual functional testing by providing additional verification of security-related backend behaviour.

## 4. Build and Regression Verification

Final verification confirmed successful completion of:

- Rust tests
- Rust compilation/check
- Frontend production build
- Tauri release build
- Windows executable generation
- MSI installer generation
- NSIS installer generation

This provides additional assurance that the tested version of VaultFive could also be successfully packaged for Windows deployment.

## 5. QA Assessment

The completed testing provides coverage across the main VaultFive requirements, including:

- Vault creation and authentication
- Lock/logout
- Credential CRUD operations
- Search
- Input validation
- Password masking and reveal
- Password generation
- Credential persistence
- Unauthenticated access protection
- Basic keyboard accessibility

The use of test descriptions, test data or steps, expected results, actual results and PASS/FAIL outcomes provided a clear method for recording whether each test behaved as expected.

This also provides traceability between the project requirements and the testing used to verify the completed application.

## 6. Risk Management Review

Risk management was maintained throughout the project and subsequently updated through Risk Register v1.1.

Some risks were reduced following implementation, testing and project-management actions. However, genuine remaining risks continued to be recorded as active rather than being marked as resolved without sufficient evidence.

### 6.1 R-05 – Team Availability

**Residual Rating:** High / Active

Team availability remained a concern because limited availability could affect coordination, reviews and completion of allocated responsibilities.

### 6.2 R-06 – Project Manager Workload Concentration

**Residual Rating:** High / Active

A significant concentration of responsibility with the Project Manager increased workload and created additional schedule and delivery pressure.

### 6.3 R-08 – Individual Contribution Evidence

**Residual Rating:** High / Active

Maintaining clear evidence of individual contribution remained important because collaboration and engagement form a significant part of the project assessment.

Keeping these risks active provides a more realistic representation of the project's final risk position.

## 7. Quality Strengths

The final QA evidence includes:

- A documented Test Plan
- 18 completed test cases
- Expected and actual results
- Positive and negative testing
- Security-related testing
- Automated Rust tests
- Requirements-to-testing traceability
- Final build verification
- Risk monitoring and review
- Windows release-package verification

Together, these provide evidence that quality assurance was considered throughout the development and final verification stages rather than only at project completion.

## 8. Future QA Improvements

Future development could strengthen the QA process through:

- Increased automated test coverage
- Independent usability testing
- More extensive accessibility evaluation
- Additional security assessment
- Testing across a wider range of Windows environments
- Longer-term reliability testing

These activities would provide additional assurance if VaultFive were developed beyond the current academic project scope.

## 9. My Final QA and Risk Review

I participated in testing the VaultFive application with the Project Manager and helped check that the main features worked as expected.

I believe the 18 test cases provided good coverage of the main functions, including authentication, credential management, search, security and accessibility. One thing that worked particularly well was testing both valid and invalid scenarios, as this helped confirm that the application handled incorrect input appropriately.

I also reviewed the remaining project risks and believe it was appropriate to keep the high residual risks active where they had not been fully resolved.

For future development, I would recommend increasing the number of automated tests and carrying out more testing with independent users.

## 10. Final Conclusion

Based on my QA and risk review, I believe the completed VaultFive application demonstrates that the main project requirements are working as intended.

The successful completion of all 18 planned test cases, together with the automated tests and final build verification, provides sufficient evidence of the quality and functionality of the final application.

---

**Reviewed by:** Eray  
**Project Role:** QA & Risk  
**Date:** 16 September 2026
