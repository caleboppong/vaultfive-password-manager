# VaultFive Sprint 2 Review

**Project:** VaultFive Password Manager  

**Team:** Team 4  

**Sprint:** Sprint 2  

**Prepared by:** Caleb  

**Status:** Completed


---

## 1. Sprint 2 Purpose

The purpose of Sprint 2 was to progress VaultFive from the initial secure application foundation into a working password manager with the main credential-management functions implemented and tested.

The sprint focused on integrating the authentication and encryption foundation with credential storage, the vault dashboard, credential management and formal functional and security testing.


---

## 2. Sprint 2 Objectives

The main objectives for Sprint 2 were:

- Complete the master password and vault authentication workflow.
- Implement secure encrypted credential storage.
- Implement credential Create, Read, Update and Delete operations.
- Develop the main VaultFive dashboard.
- Implement credential searching.
- Implement password masking and reveal/hide controls.
- Implement credential input validation.
- Implement secure password generation.
- Verify credential persistence.
- Verify locked-vault protection.
- Perform basic keyboard accessibility testing.
- Execute and document the planned functional and security tests.


---

## 3. Work Completed

The following functionality was completed during Sprint 2.

### Vault Setup and Authentication

VaultFive supports first-time vault creation using a master password.

The master password is protected using Argon2id rather than being stored as plaintext.

The application supports authentication using the correct master password and rejects incorrect master passwords.

The user can lock the vault, which returns the application to the authentication screen.


### Secure Credential Storage

Credential data is stored locally using SQLite.

Sensitive credential information is encrypted using XChaCha20Poly1305 authenticated encryption before being written to the database.

Credential operations require access to the active vault encryption key.


### Credential Management

The application supports the main credential-management operations:

- Add a credential.
- View stored credentials.
- Edit an existing credential.
- Delete a credential.

Required credential fields are validated before submission.


### Vault Dashboard

The VaultFive dashboard provides access to stored credentials and the main vault functions.

The dashboard includes:

- Credential listing.
- Credential count.
- Search functionality.
- Add Credential functionality.
- Edit and Delete controls.
- Password masking.
- Password Show/Hide control.
- Lock Vault control.


### Search

Users can search for stored credentials.

Testing confirmed that searching for an existing credential returns the expected result.

Searching for a credential that does not exist displays a no-match message.


### Password Generator

A secure password generator was added to the credential form.

The generator creates a 20-character password containing:

- Uppercase letters.
- Lowercase letters.
- Numbers.
- Symbols.

The generated password is populated into the password field and works with the existing Show/Hide control.


### Keyboard Accessibility

A basic keyboard accessibility review was completed.

The main application controls were navigated using the Tab key without relying on the mouse.

Keyboard navigation was verified across the authentication screen, dashboard controls, credential actions and Add Credential form.


---

## 4. Testing Results

Sprint 2 included functional, security and basic accessibility verification.

The final test results were:

**Total planned test cases:** 18  

**Passed:** 18  

**Failed:** 0  

**Not Run:** 0  

The completed tests covered:

- First-time vault setup.
- Invalid setup input.
- Valid authentication.
- Invalid authentication.
- Vault locking.
- Adding credentials.
- Credential input validation.
- Viewing credentials.
- Editing credentials.
- Deleting credentials.
- Credential searching.
- Search with no matching result.
- Password masking.
- Password reveal and re-hide.
- Password generation.
- Credential persistence.
- Locked-vault session protection.
- Basic keyboard accessibility.

Detailed results are recorded in:

`docs/testing/Test-Results-v1.md`


---

## 5. Security Testing

Security verification remained an important part of Sprint 2.

The application uses Argon2id for master password protection and encryption key derivation.

Sensitive credential data is encrypted using XChaCha20Poly1305 authenticated encryption before being stored in SQLite.

Credential operations require access to the active in-memory vault encryption key.

Additional automated Rust testing was introduced to verify the locked-vault session behaviour.

The automated tests confirmed that:

- Session key access is rejected when the vault has no active encryption key.
- Session key access succeeds when a valid in-memory encryption key is available.

The automated Rust test run completed with:

**2 passed**

**0 failed**


---

## 6. Defect Review and Triage

Development and testing identified an implementation issue with the password generator during Sprint 2.

### Password Generator Variable Binding

**Issue:**  
During implementation, the password generator initially referenced a frontend variable that did not match the password variable used by the credential form.

**Impact:**  
The generated password would not have been correctly connected to the existing credential password field.

**Priority:**  
Medium

**Resolution:**  
The generator was corrected to assign the generated password to the existing `password` variable used by the credential form.

**Verification:**  
The corrected generator successfully populated the credential password field with a generated 20-character password.

**Status:**  
Resolved


No critical or high-priority product defects remained open following the completed Sprint 2 test cycle.


---

## 7. Risk Review

Sprint 2 reduced several technical risks because the main VaultFive functionality was implemented and formally tested.

The successful execution of all 18 planned test cases reduced the risk of major undiscovered functional problems in the current tested version.

The automated locked-vault test also provided additional evidence that access to the session encryption key is rejected when the vault is locked.

Project-management risks relating to team availability and concentration of workload remain relevant and should continue to be monitored during the final project stage.

Any further changes to the application before submission may introduce regression risk. Relevant tests should therefore be repeated if the implementation is changed.


---

## 8. Scope Review

The main Must requirements required for the current VaultFive implementation have been progressed and tested.

The Should requirement for password generation was also implemented and tested during Sprint 2.

Password strength feedback remains a Could requirement and is not required for the current core implementation.

This allows the final project stage to focus on quality assurance, documentation, evidence and submission readiness rather than introducing unnecessary additional functionality.


---

## 9. Sprint 2 Outcome

Sprint 2 achieved its main objective of producing an integrated and tested VaultFive password manager.

The application now provides a working flow from vault authentication through encrypted credential management.

The completed implementation includes authentication, vault locking, encrypted storage, credential CRUD operations, search, password masking, password reveal/hide, password generation, validation and persistence.

All 18 planned test cases passed.

No failed or NOT RUN test cases remained at the end of the Sprint 2 test cycle.


---

## 10. Actions for Final Project Stage

The next project stage will focus on:

- Final regression checks if the application code changes.
- Review of project documentation.
- Review of GitHub issues and project evidence.
- Final risk review.
- Final assessment report preparation.
- Collection and organisation of appropriate screenshots and evidence.
- Final build and submission-readiness checks.

New functionality should only be introduced where it provides clear assessment value and does not create unnecessary risk before submission.


---

## 11. Conclusion

Sprint 2 successfully delivered and tested the core VaultFive password-management workflow.

The sprint produced an integrated desktop application with secure vault authentication, encrypted credential storage, credential management, search, password controls and basic accessibility support.

Testing resulted in 18 PASS, 0 FAIL and 0 NOT RUN test cases.

The password-generator implementation issue identified during development was corrected and verified.

VaultFive can now progress to the final project stage, with the main focus moving from feature development to final quality assurance, documentation and submission readiness.
