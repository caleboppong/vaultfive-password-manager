# VaultFive User Stories & MoSCoW Prioritisation v1.0

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Document | User Stories & MoSCoW Prioritisation |
| Version | 1.0 |
| Prepared by | Caleb |
| Role | Project Manager / Scrum Lead |
| Status | Initial Draft |
| Review Status | Pending Team Review |

## 1. Purpose

This document translates the VaultFive requirements baseline into
user-focused stories and establishes priorities using the MoSCoW
method.

The priorities are intended to protect the minimum viable product
from unnecessary scope expansion while allowing useful enhancements
where sufficient development time remains.

---

# 2. User Stories

## US-01 — Initialise Vault

**As a** first-time user,  
**I want** to initialise a local VaultFive vault and establish a
master password,  
**so that** I can begin using the password manager.

**Related Requirements:** FR-01, NFR-02

### Acceptance Criteria

- A first-time user can initialise the vault.
- A master password is required.
- Invalid input produces appropriate feedback.
- The application does not require a cloud account.

**Priority:** Must

---

## US-02 — Unlock Vault

**As a** returning user,  
**I want** to authenticate using my master password,  
**so that** only an authorised user can access my stored credentials.

**Related Requirements:** FR-02, NFR-03

### Acceptance Criteria

- A valid master password unlocks the vault.
- An invalid master password does not unlock the vault.
- Failed authentication produces understandable feedback.

**Priority:** Must

---

## US-03 — Lock Vault

**As an** authenticated user,  
**I want** to lock or log out of VaultFive,  
**so that** my stored credentials are not left accessible.

**Related Requirements:** FR-03, NFR-03

### Acceptance Criteria

- The user can initiate logout/lock.
- Vault contents become inaccessible.
- Authentication is required to regain access.

**Priority:** Must

---

## US-04 — Add Credential

**As an** authenticated user,  
**I want** to add an account credential,  
**so that** I can store login information for later use.

**Related Requirements:** FR-04, NFR-01, NFR-04

### Acceptance Criteria

- The user can enter credential information.
- Required fields are validated.
- A valid credential can be saved.
- The saved credential appears in the vault.

**Priority:** Must

---

## US-05 — View Credential

**As an** authenticated user,  
**I want** to view my stored credentials,  
**so that** I can retrieve account information when required.

**Related Requirements:** FR-05, FR-09, NFR-03

### Acceptance Criteria

- Stored credentials are available after authentication.
- A credential can be selected.
- Password values are masked by default.

**Priority:** Must

---

## US-06 — Edit Credential

**As an** authenticated user,  
**I want** to edit a stored credential,  
**so that** I can keep my account information up to date.

**Related Requirements:** FR-06

### Acceptance Criteria

- An existing credential can be selected.
- Editable values can be changed.
- Valid changes can be saved.
- Saved changes remain after reopening the record.

**Priority:** Must

---

## US-07 — Delete Credential

**As an** authenticated user,  
**I want** to delete a credential I no longer require,  
**so that** obsolete information is removed from my vault.

**Related Requirements:** FR-07

### Acceptance Criteria

- A credential can be selected for deletion.
- Confirmation is requested before deletion.
- Confirmed deletion removes the credential.

**Priority:** Must

---

## US-08 — Reveal Password

**As an** authenticated user,  
**I want** to temporarily reveal a masked password,  
**so that** I can read it when necessary without having passwords
displayed continuously.

**Related Requirements:** FR-09

### Acceptance Criteria

- Passwords are masked by default.
- The user can intentionally reveal a password.
- The password can be hidden again.

**Priority:** Must

---

## US-09 — Search Credentials

**As an** authenticated user,  
**I want** to search my stored credentials,  
**so that** I can quickly locate a particular account.

**Related Requirements:** FR-08

### Acceptance Criteria

- The user can enter a search term.
- Matching credentials are displayed.
- No-match searches provide appropriate feedback.

**Priority:** Should

---

## US-10 — Generate Password

**As an** authenticated user,  
**I want** VaultFive to generate a password,  
**so that** I can create passwords without having to devise them
manually.

**Related Requirements:** FR-10

### Acceptance Criteria

- The user can request a generated password.
- The generated password can be used when adding or editing a
  credential.

**Priority:** Should

---

## US-11 — Password Strength Feedback

**As a** user,  
**I want** understandable feedback about password strength,  
**so that** I can identify potentially weak passwords.

**Related Requirements:** FR-11

### Acceptance Criteria

- Password input can be evaluated using documented criteria.
- Feedback is understandable to the user.

**Priority:** Could

---

# 3. MoSCoW Prioritisation

## Must Have

| ID | Requirement / Feature | Reason |
|---|---|---|
| FR-01 | Initial vault setup | Required to establish the local vault |
| FR-02 | Authentication | Required to control vault access |
| FR-03 | Logout / vault lock | Required to end authenticated access |
| FR-04 | Add credential | Core password-manager functionality |
| FR-05 | View credentials | Core password-manager functionality |
| FR-06 | Edit credential | Required for maintaining stored data |
| FR-07 | Delete credential | Required for managing obsolete data |
| FR-09 | Password masking/reveal | Protects passwords from continuous visual exposure |
| NFR-01 | Secure credential storage | Core security requirement |
| NFR-02 | Master password protection | Core authentication-security requirement |
| NFR-03 | Authentication protection | Prevents normal unauthenticated vault access |
| NFR-04 | Input validation | Required for reliable operation |
| NFR-08 | Reliability | Saved credentials must persist correctly |
| NFR-10 | Desktop constraint | Required by the project brief |

## Should Have

| ID | Requirement / Feature | Reason |
|---|---|---|
| FR-08 | Credential search | Improves usability but is not essential to the minimum vault |
| FR-10 | Password generator | Useful password-manager enhancement |
| NFR-05 | Usability | Important to the quality of the product |
| NFR-06 | Accessibility | Improves inclusive use of the interface |
| NFR-07 | Performance | Important for satisfactory local operation |
| NFR-09 | Maintainability | Supports controlled development and modification |

## Could Have

| ID | Requirement / Feature | Reason |
|---|---|---|
| FR-11 | Password strength feedback | Useful enhancement but not required for the core workflow |

## Won't Have in This Release

The following features are excluded from the current project scope:

- Browser extension
- Browser-based password manager
- Cloud synchronisation
- Multi-device synchronisation
- Mobile application
- Credential sharing between users
- Enterprise administration
- Online account recovery

These exclusions protect the project from scope expansion and allow
development effort to remain focused on the required desktop password
manager.

---

# 4. Minimum Viable Product

For VaultFive to satisfy the planned minimum product scope, the team
should prioritise the following workflow:

1. Initialise vault
2. Authenticate
3. Unlock vault
4. Add credential
5. View credential
6. Edit credential
7. Delete credential
8. Mask/reveal passwords
9. Lock/logout
10. Persist protected credential data locally

Should-have and Could-have functionality should not delay completion
or testing of this core workflow.

---

# 5. Traceability Summary

| User Story | Primary Requirement(s) | Priority |
|---|---|---|
| US-01 | FR-01, NFR-02 | Must |
| US-02 | FR-02, NFR-03 | Must |
| US-03 | FR-03, NFR-03 | Must |
| US-04 | FR-04, NFR-01, NFR-04 | Must |
| US-05 | FR-05, FR-09, NFR-03 | Must |
| US-06 | FR-06 | Must |
| US-07 | FR-07 | Must |
| US-08 | FR-09 | Must |
| US-09 | FR-08 | Should |
| US-10 | FR-10 | Should |
| US-11 | FR-11 | Could |

---

# 6. Review and Change Control

These priorities represent the initial project position.

If schedule, technical or testing risks threaten completion of the
Must-have functionality, Should-have and Could-have functionality may
be deferred.

Any significant change to these priorities should be recorded and
considered against:

- Requirements Baseline
- Project schedule
- Risk Register
- Technical design
- Test Plan

This document remains subject to project review.
