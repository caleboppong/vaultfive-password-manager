# VaultFive Test Plan v1.0

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Document | Test Plan |
| Version | 1.0 |
| Status | Initial Test Plan |
| Prepared by | Caleb |
| Review Status | Pending Team Review |

## 1. Purpose

This document defines the initial testing and quality-assurance
approach for VaultFive.

Testing will be based on the documented functional and non-functional
requirements. The objective is to identify defects, verify expected
behaviour and provide evidence of product quality.

Actual test results will only be recorded after the corresponding
functionality has been implemented and the test has been executed.

---

## 2. Testing Objectives

Testing will aim to:

- Verify Must-have functional requirements.
- Verify important non-functional requirements where practical.
- Test valid and invalid user input.
- Identify defects before final submission.
- Verify interaction between application components.
- Evaluate important security-related behaviours.
- Consider usability and accessibility.
- Retest corrected defects.
- Execute regression tests after significant changes.

---

## 3. Testing Types

### 3.1 Functional Testing

Functional testing will verify that implemented features behave
according to the requirements and acceptance criteria.

Examples include:

- Vault setup
- Authentication
- Logout/lock
- Adding credentials
- Viewing credentials
- Editing credentials
- Deleting credentials
- Password masking/reveal
- Search
- Password generation

### 3.2 Negative Testing

Negative testing will verify that inappropriate or invalid input is
handled safely.

Examples include:

- Incorrect master password
- Missing required fields
- Invalid credential input
- Attempting protected actions without authentication

### 3.3 Boundary Testing

Where appropriate, boundary testing will consider input at or around
defined limits.

Examples may include:

- Empty input
- Minimum accepted input
- Long text input
- Password-length boundaries

Specific boundaries must be based on the implemented validation rules.

### 3.4 Integration Testing

Integration testing will verify interaction between major application
components, including:

- Svelte user interface
- Tauri desktop layer
- Rust backend
- SQLite database

### 3.5 Security-Oriented Testing

Security-oriented testing will consider behaviours such as:

- Invalid authentication attempts
- Vault access before authentication
- Vault access after logout
- Password masking
- Persistence of sensitive information
- Storage protection

Testing does not by itself prove that the application is completely
secure.

### 3.6 Usability and Accessibility Testing

Testing will consider:

- Understandable navigation
- Form labels
- Error messages
- Keyboard interaction
- Readability
- Password visibility controls
- Confirmation of destructive actions

### 3.7 Regression Testing

After defects or significant changes are introduced, previously
successful tests relevant to the changed area will be repeated to
check that existing functionality has not been broken.

---

# 4. Test Case Format

Each executed test should contain:

| Field | Purpose |
|---|---|
| Test ID | Unique test identifier |
| Requirement ID | Requirement being verified |
| Test Type | Functional, negative, integration, etc. |
| Description | What is being tested |
| Pre-condition | Required starting state |
| Test Data | Input used during execution |
| Steps | Actions performed |
| Expected Result | Behaviour expected before execution |
| Actual Result | Behaviour actually observed |
| Status | PASS / FAIL / BLOCKED |
| Defect / Evidence | Related issue, screenshot or other evidence |

Expected results must be defined before execution where practical.

Actual results must reflect what genuinely occurred.

---

# 5. Initial Planned Test Cases

## TC-01 — Initialise Vault

**Requirement:** FR-01  
**Type:** Functional

**Description:** Verify that a first-time user can initialise a vault.

**Pre-condition:** VaultFive has no existing initialised vault.

**Test Data:** Valid master password meeting implemented validation rules.

**Expected Result:** The vault is successfully initialised and the user
can proceed according to the implemented workflow.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-02 — Reject Invalid Vault Setup Input

**Requirement:** FR-01, NFR-04  
**Type:** Negative

**Description:** Verify that invalid initial vault setup input is rejected.

**Expected Result:** The application rejects the invalid input and
provides understandable feedback.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-03 — Authenticate With Valid Master Password

**Requirement:** FR-02  
**Type:** Functional

**Expected Result:** Correct authentication grants access to the vault.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-04 — Reject Incorrect Master Password

**Requirement:** FR-02, NFR-03  
**Type:** Negative / Security-oriented

**Expected Result:** Access is denied and appropriate feedback is
displayed.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-05 — Lock / Logout

**Requirement:** FR-03  
**Type:** Functional / Security-oriented

**Expected Result:** Vault contents become inaccessible and
authentication is required to regain access.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-06 — Add Valid Credential

**Requirement:** FR-04  
**Type:** Functional

**Expected Result:** The credential is saved and becomes available in
the vault.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-07 — Reject Invalid Credential Input

**Requirement:** FR-04, NFR-04  
**Type:** Negative

**Expected Result:** Invalid input is rejected and understandable
feedback is displayed.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-08 — View Stored Credential

**Requirement:** FR-05  
**Type:** Functional

**Expected Result:** The selected credential can be viewed after
authentication.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-09 — Edit Credential

**Requirement:** FR-06  
**Type:** Functional

**Expected Result:** Valid changes are saved and displayed when the
credential is viewed again.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-10 — Delete Credential

**Requirement:** FR-07  
**Type:** Functional

**Expected Result:** The user is asked to confirm deletion and a
confirmed deletion removes the credential.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-11 — Search Credentials

**Requirement:** FR-08  
**Type:** Functional

**Expected Result:** Matching credentials are displayed for a valid
search term.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-12 — Search With No Match

**Requirement:** FR-08  
**Type:** Negative / Functional

**Expected Result:** No matching credentials are displayed and the
application provides appropriate empty-result feedback.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-13 — Password Masked by Default

**Requirement:** FR-09  
**Type:** Functional / Security-oriented

**Expected Result:** Stored password values are visually masked by
default.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-14 — Reveal and Re-hide Password

**Requirement:** FR-09  
**Type:** Functional

**Expected Result:** The user can intentionally reveal a password and
hide it again.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-15 — Password Generator

**Requirement:** FR-10  
**Type:** Functional

**Expected Result:** The application generates a password that can be
used in the credential workflow.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-16 — Credential Persistence

**Requirement:** NFR-08  
**Type:** Integration / Reliability

**Expected Result:** A valid saved credential remains available after
the application is closed and reopened and the user successfully
authenticates.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-17 — Unauthenticated Vault Access

**Requirement:** NFR-03  
**Type:** Security-oriented

**Expected Result:** The normal application workflow does not expose
vault contents without successful authentication.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

## TC-18 — Basic Accessibility / Keyboard Review

**Requirement:** NFR-06  
**Type:** Accessibility / Usability

**Expected Result:** Core implemented workflows can be reviewed for
keyboard interaction, understandable labels, readable feedback and
other documented accessibility considerations.

**Actual Result:** Not yet executed.

**Status:** NOT RUN

---

# 6. Defect Management

When a test fails:

1. Record the observed result accurately.
2. Mark the test as FAIL.
3. Create or reference a defect where appropriate.
4. Record the affected requirement.
5. Investigate and correct the defect.
6. Repeat the failed test.
7. Perform relevant regression testing.
8. Retain evidence of the original failure and subsequent result.

A failed test must not be changed to PASS unless the relevant test is
actually executed again successfully.

---

# 7. Test Evidence

Evidence may include:

- Completed test records
- GitHub issues
- Screenshots
- Relevant commit references
- Defect records
- Retest results
- Regression results

Evidence should be stored or referenced in a way that allows the test
result to be traced back to the corresponding requirement.

---

# 8. Entry and Exit Criteria

## Entry Criteria

Testing of a feature may begin when:

- The relevant requirement is sufficiently defined.
- The feature has been implemented sufficiently for testing.
- Required test data or preconditions are available.

## Exit Criteria

The product may be considered ready for final evaluation when:

- Must-have functionality has been tested.
- Significant known defects have been reviewed.
- Critical unresolved defects are clearly documented.
- Relevant failed tests have been retested after fixes.
- Appropriate regression testing has been completed.
- Available test evidence has been retained.

---

# 9. Review Status

Test Plan v1.0 establishes the initial QA approach.

The planned test cases currently contain no fabricated execution
results. Actual Result and final Status fields will be updated only
when the application is available and the corresponding tests have
actually been performed.
