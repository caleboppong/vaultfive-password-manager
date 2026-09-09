# VaultFive Requirements Baseline v1.0

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Document | Requirements Baseline |
| Version | 1.0 |
| Status | Initial Baseline |
| Prepared by | Caleb |
| Role | Project Manager / Scrum Lead |
| Date | 09 September 2026 |
| Review Status | Pending Team Review |

## 1. Purpose

This document defines the initial functional and non-functional
requirements for VaultFive.

VaultFive is a desktop password manager intended to allow a user to
authenticate and securely manage locally stored account credentials.

This baseline provides a reference for design, implementation,
testing and later requirements traceability.

---

## 2. Project Scope

### In Scope

The initial scope includes:

- Master-password/account setup
- User authentication
- Login and logout
- Local credential storage
- Adding credentials
- Viewing credentials
- Editing credentials
- Deleting credentials
- Password masking and reveal
- Input validation
- Secure handling of stored credential data

Subject to available development time, the project may also include:

- Credential search
- Password generation
- Password-strength feedback

### Out of Scope

The initial project does not intend to provide:

- Browser extension functionality
- Browser-based password management
- Cloud synchronisation
- Multi-device synchronisation
- Mobile applications
- Password sharing between users
- Enterprise administration
- Online account recovery

---

# 3. Functional Requirements

## FR-01 — Initial Vault Setup

**Requirement:**  
The system shall allow a first-time user to initialise a local
VaultFive vault and establish a master password.

**Acceptance Criteria:**

- A first-time user can initialise the vault.
- A master password is required.
- Invalid or insufficient input produces appropriate feedback.
- The application does not require a cloud account.

**Priority:** Must

---

## FR-02 — User Authentication

**Requirement:**  
The system shall require the user to authenticate using the master
password before access to stored credentials is granted.

**Acceptance Criteria:**

- A valid master password permits access.
- An invalid master password does not permit access.
- An appropriate error message is displayed after unsuccessful
  authentication.

**Priority:** Must

---

## FR-03 — Logout / Vault Lock

**Requirement:**  
The authenticated user shall be able to lock or log out of the vault.

**Acceptance Criteria:**

- The user can initiate logout/lock.
- Vault contents are no longer accessible after logout.
- Authentication is required before the vault can be accessed again.

**Priority:** Must

---

## FR-04 — Add Credential

**Requirement:**  
The authenticated user shall be able to create a credential record.

A credential record should support, where appropriate:

- Service/site name
- Username or email
- Password
- Optional notes

**Acceptance Criteria:**

- Required information is validated.
- A valid credential can be stored.
- The newly created credential becomes available in the vault.

**Priority:** Must

---

## FR-05 — View Credentials

**Requirement:**  
The authenticated user shall be able to view their stored credential
records.

**Acceptance Criteria:**

- Stored records are available only after authentication.
- Credential records can be selected for viewing.
- Password values are masked by default.

**Priority:** Must

---

## FR-06 — Edit Credential

**Requirement:**  
The authenticated user shall be able to modify an existing credential.

**Acceptance Criteria:**

- An existing record can be selected.
- Editable values can be changed.
- Valid changes can be saved.
- Saved changes are reflected when the credential is viewed again.

**Priority:** Must

---

## FR-07 — Delete Credential

**Requirement:**  
The authenticated user shall be able to delete an existing credential.

**Acceptance Criteria:**

- An existing record can be selected for deletion.
- The user receives confirmation before destructive deletion.
- A confirmed deletion removes the credential from the vault.

**Priority:** Must

---

## FR-08 — Search Credentials

**Requirement:**  
The authenticated user should be able to search stored credentials.

**Acceptance Criteria:**

- A search term can be entered.
- Matching credential records are displayed.
- A search with no matches produces an appropriate empty result.

**Priority:** Should

---

## FR-09 — Password Masking and Reveal

**Requirement:**  
Stored passwords shall be visually masked by default when displayed
in the application.

**Acceptance Criteria:**

- Passwords are not displayed as plain text by default.
- The user can intentionally reveal a password when required.
- The password can be hidden again.

**Priority:** Must

---

## FR-10 — Password Generator

**Requirement:**  
The system should provide a mechanism for generating a password for a
credential.

**Acceptance Criteria:**

- The user can request a generated password.
- The generated password can be used when creating or editing a
  credential.

**Priority:** Should

---

## FR-11 — Password Strength Feedback

**Requirement:**  
The system could provide basic feedback about the strength of a
password entered by the user.

**Acceptance Criteria:**

- Password input can be evaluated against documented criteria.
- Feedback is understandable to the user.

**Priority:** Could

---

# 4. Non-Functional Requirements

## NFR-01 — Secure Credential Storage

Sensitive credential information shall not be intentionally stored as
unencrypted plain text in persistent application storage.

**Priority:** Must

---

## NFR-02 — Master Password Protection

The application shall not intentionally persist the user's master
password in readable plain-text form.

**Priority:** Must

---

## NFR-03 — Authentication Protection

Unauthenticated users shall not be able to access the normal vault
interface or stored credential information through the application's
intended user workflow.

**Priority:** Must

---

## NFR-04 — Input Validation

The application shall validate relevant user input and provide
understandable feedback for invalid input.

**Priority:** Must

---

## NFR-05 — Usability

The main application workflows should be understandable without
specialist technical knowledge.

**Priority:** Should

---

## NFR-06 — Accessibility

The interface should consider:

- Keyboard navigation
- Readable labels
- Appropriate form labels
- Clear error feedback
- Adequate readability
- Avoidance of relying solely on colour to communicate important
  information

**Priority:** Should

---

## NFR-07 — Performance

Normal local operations such as opening the vault list, searching and
saving a credential should provide responsive feedback under expected
assessment-scale data volumes.

**Priority:** Should

---

## NFR-08 — Reliability

Valid saved credential records should remain available after the
application is closed and reopened, subject to successful
authentication.

**Priority:** Must

---

## NFR-09 — Maintainability

The application should maintain a reasonable separation between user
interface, application logic and data-storage responsibilities.

**Priority:** Should

---

## NFR-10 — Desktop Constraint

VaultFive shall be delivered as a desktop application rather than as a
browser-based password manager.

**Priority:** Must

---

# 5. Assumptions and Constraints

The initial baseline assumes that:

- VaultFive will operate as a desktop application.
- The assessment prioritises software engineering and project
  management rather than excessive functionality.
- Development time is limited.
- The application is being produced as an academic project.
- The initial implementation is intended for local use.
- Enhancements must not prevent completion of Must-have requirements.

---

# 6. Requirements Prioritisation

The project uses MoSCoW prioritisation:

**Must** — Required for the minimum viable product.

**Should** — Important but may be deferred if necessary to protect the
core project.

**Could** — Desirable enhancement implemented only when sufficient time
remains.

**Won't (for this release)** — Explicitly excluded from the current
project scope.

---

# 7. Requirements Change Control

This document represents Requirements Baseline v1.0.

Changes after the baseline should be recorded rather than silently
overwriting the original project requirements.

A requirement change should record:

- Date
- Requirement ID
- Proposed change
- Reason
- Impact
- Decision
- Person responsible

Significant scope changes should be reviewed against the project
schedule, risk register, design and testing requirements.

---

# 8. Review Status

This initial baseline has been prepared to prevent downstream project
activities from being blocked.

The requirements remain subject to review during the next appropriate
Team 4 project review.

Any accepted changes should result in an updated document version and
appropriate updates to related project artefacts.
