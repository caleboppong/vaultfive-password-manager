\# VaultFive Test Results v1



\*\*Project:\*\* VaultFive Password Manager  

\*\*Team:\*\* Team 4  

\*\*Test Phase:\*\* Sprint 2 Functional and Security Testing  

\*\*Test Environment:\*\* Windows Desktop / Tauri / Svelte / Rust / SQLite  

\*\*Test Type:\*\* Manual functional testing and implementation verification  

\*\*Prepared by:\*\* Caleb  

\*\*Status:\*\* In Progress



\---



\## 1. Purpose



This document records the actual test results for the VaultFive Password Manager during Sprint 2.



The tests are based on the test cases defined in `Test-Plan-v1.md`. Only tests that have actually been executed are recorded as PASS or FAIL. Tests that have not yet been executed remain NOT RUN.



The purpose of this testing phase is to verify the core authentication, credential management, search, password visibility, validation and persistence functions before the final project review.



\---



\## 2. Test Environment



The application was tested as a Tauri desktop application on Windows.



The implementation uses:



\- Svelte and TypeScript for the user interface.

\- Rust for the Tauri backend.

\- SQLite for local credential storage.

\- Argon2id for master password protection and encryption key derivation.

\- XChaCha20Poly1305 authenticated encryption for sensitive credential data.



Test credentials were used during testing and were not real account credentials.



\---



\## 3. Test Results



| Test ID | Test | Expected Result | Actual Result | Status |

|---|---|---|---|---|

| TC01 | Initialise Vault | A new vault should be created when valid setup details are provided. | Not executed during this formal test session because an existing vault was already configured. | NOT RUN |

| TC02 | Reject Invalid Vault Setup Input | Invalid initial vault setup input should be rejected. | Not executed during this formal test session because the vault was already configured. | NOT RUN |

| TC03 | Authenticate Valid Master Password | The correct master password should unlock the vault. | Correct master password successfully unlocked the vault and allowed access to stored credentials. | PASS |

| TC04 | Reject Incorrect Master Password | An incorrect master password should not unlock the vault. | Incorrect master password was rejected and the message "Incorrect master password." was displayed. | PASS |

| TC05 | Lock / Logout | Locking the vault should return the user to the authentication screen. | Lock Vault returned the application to the locked authentication screen. | PASS |

| TC06 | Add Valid Credential | A valid credential should be encrypted and saved. | GitHub and Microsoft test credentials were successfully added and displayed in the vault. | PASS |

| TC07 | Reject Invalid Credential Input | Credential input with required information missing should be rejected. | A credential with an empty Service field was rejected and "Service name is required." was displayed. | PASS |

| TC08 | View Stored Credential | Saved credentials should be displayed after they are added. | Saved GitHub and Microsoft credentials were displayed correctly in the vault dashboard. | PASS |

| TC09 | Edit Credential | A stored credential should be editable and the new values should be displayed. | The GitHub username was changed from `caleb@example.com` to `caleb.github@example.com`. The updated value was displayed successfully. | PASS |

| TC10 | Delete Credential | The selected credential should be removed after deletion is confirmed. | The Microsoft test credential was deleted successfully and the vault count returned to one credential. | PASS |

| TC11 | Search Credentials | Searching for an existing credential should return matching results. | Searching for `GitHub` returned the GitHub credential and showed one matching result. | PASS |

| TC12 | Search No Match | Searching for a value that does not exist should show no matching results. | Searching for `Netflix` displayed "No matching credentials". | PASS |

| TC13 | Password Masked by Default | Stored passwords should not be visible by default. | Credential passwords were displayed as masked characters when credentials were loaded. | PASS |

| TC14 | Reveal / Re-hide Password | The user should be able to reveal a password and hide it again. | The GitHub test password was successfully revealed and the control changed to Hide. The password could then be hidden again. | PASS |

| TC15 | Password Generator | The application should generate a password when requested. | Password generator has not yet been implemented or formally tested. | NOT RUN |

| TC16 | Credential Persistence | Saved credentials should remain available after the vault is locked and unlocked again. | After locking and successfully unlocking the vault again, the edited GitHub credential remained available with the updated username. | PASS |

| TC17 | Unauthenticated Vault Access | Automated Rust tests verified the vault session-state protection. When the encryption key was absent, session key access returned "Vault is locked.". When a valid in-memory key was present, session key access succeeded. cargo test completed with 2 passed and 0 failed. | PASS |

| TC18 | Basic Accessibility / Keyboard Review | Core application controls were successfully navigated using the Tab key without relying on the mouse. Keyboard focus moved through the authentication screen, dashboard controls, credential actions and Add Credential form controls. Labels and controls remained understandable during keyboard-only navigation. | PASS |



\---



\## 4. Test Summary



Total planned test cases: \*\*18\*\*



Passed: \*\*13\*\*



Failed: \*\*0\*\*



Not Run: \*\*5\*\*



Tests currently passing:



\- TC03

\- TC04

\- TC05

\- TC06

\- TC07

\- TC08

\- TC09

\- TC10

\- TC11

\- TC12

\- TC13

\- TC14

\- TC16



Tests remaining:



\- TC01

\- TC02

\- TC15

\- TC17

\- TC18



No failures were observed during the executed Sprint 2 functional test sequence.



\---



\## 5. Security Verification



The credential-management implementation uses the secure storage foundation developed earlier in the project.



The master password is not stored as plaintext. It is protected using Argon2id.



After successful authentication, an encryption key is derived and retained in application memory while the vault is unlocked.



Sensitive credential data is encrypted before being written to SQLite using XChaCha20Poly1305 authenticated encryption.



During the earlier secure-storage implementation check, test credential data was encrypted, persisted to SQLite, read back and successfully decrypted. The temporary secure-storage test record was removed after verification.



The current CRUD implementation also requires the vault encryption key before credential operations can access encrypted credential data.



A formal direct test of unauthenticated backend credential access remains pending under TC17.



\---



\## 6. Functional Testing Observations



The completed manual test sequence demonstrated that the main VaultFive workflow operates correctly.



A user can unlock the vault, add credentials, view stored credentials, edit existing information, delete credentials and search the vault.



Passwords remain masked by default and can be temporarily revealed and hidden again.



Required credential fields are validated before a credential is submitted.



The lock function returns the application to the authentication screen. An incorrect master password is rejected.



After the vault was unlocked again using the correct master password, the previously edited GitHub credential remained available. This demonstrated persistence of the encrypted credential record across the lock and unlock cycle.



\---



\## 7. Outstanding Testing



The following work remains before the final testing phase can be considered complete:



\*\*TC01 and TC02\*\* require testing the first-time vault setup workflow in a clean test environment.



\*\*TC15\*\* requires implementation and testing of the password generator if this Should requirement remains within the final project scope.



\*\*TC17\*\* requires a formal test demonstrating that credential operations are rejected when the backend vault state is locked.



\*\*TC18\*\* requires a basic keyboard and accessibility review of the completed user interface.



These tests will not be recorded as PASS until they have actually been executed.



\---



\## 8. Conclusion



The Sprint 2 test session successfully verified the main credential-management workflow of VaultFive.



Thirteen of the eighteen planned test cases have currently passed, with no failures recorded during the executed test sequence.



The results provide evidence that authentication, locking, credential creation, viewing, editing, deletion, search, input validation, password masking, password reveal/hide and credential persistence are functioning as expected.



The remaining test cases will be addressed during the final testing and review stage before submission readiness.

