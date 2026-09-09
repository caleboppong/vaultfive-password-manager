# VaultFive Technical Architecture v1.0

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Document | Technical Architecture |
| Version | 1.0 |
| Prepared by | Caleb |
| Status | Initial Architecture |
| Review Status | Pending Review |

## 1. Purpose

This document defines the initial technical architecture proposed for
the VaultFive desktop password manager.

The architecture is intended to support the documented functional and
non-functional requirements while maintaining a clear separation
between the user interface, application logic and persistent storage.

---

## 2. Proposed Technology Stack

| Layer | Proposed Technology | Purpose |
|---|---|---|
| Desktop Framework | Tauri 2 | Packages and runs VaultFive as a desktop application |
| Frontend | Svelte | User-interface components and screens |
| Frontend Language | TypeScript | Client-side application logic |
| Backend | Rust | Application and security-sensitive backend logic |
| Database | SQLite | Local persistent storage |

The final libraries and versions used should be recorded after the
development environment has been established.

---

## 3. High-Level Architecture

VaultFive will initially follow this structure:

User
  ↓
Svelte / TypeScript UI
  ↓
Tauri command interface
  ↓
Rust application logic
  ↓
Security / data-access layer
  ↓
SQLite local database

The frontend is responsible primarily for presentation and user
interaction.

Rust is intended to handle backend operations, including communication
with persistent storage and security-sensitive processing.

SQLite provides local persistent storage.

---

## 4. Main Components

### 4.1 User Interface

The frontend will provide screens/components for:

- Initial vault setup
- Login
- Vault/dashboard
- Add credential
- View credential
- Edit credential
- Delete confirmation
- Password masking/reveal
- Search, if implemented
- Password generation, if implemented

### 4.2 Authentication

Authentication will use the user's master password to control access
to the local vault.

The master password must not intentionally be stored as readable
plain text.

The exact password-hashing/key-derivation implementation will be
confirmed during implementation and documented once selected.

### 4.3 Credential Management

The backend will provide operations corresponding to the core
credential lifecycle:

- Create
- Read
- Update
- Delete

These operations correspond primarily to FR-04 through FR-07.

### 4.4 Data Storage

SQLite is proposed for local persistence.

A credential record is expected to contain information such as:

- Credential ID
- Service/site name
- Username/email
- Protected password value
- Optional notes
- Created/updated metadata where useful

The exact schema will be confirmed during implementation.

### 4.5 Security Layer

Sensitive credential information must not intentionally be persisted
as unencrypted plain text.

Established cryptographic libraries should be used rather than
designing custom cryptographic algorithms.

The exact encryption and key-derivation libraries will be recorded
after implementation decisions have been verified.

---

## 5. Initial Data Flow

### Authentication

User enters master password
        ↓
UI sends authentication request
        ↓
Rust backend validates authentication
        ↓
Success → vault access
Failure → access denied

### Add Credential

User enters credential
        ↓
UI validates required input
        ↓
Data sent to Rust backend
        ↓
Sensitive data protected
        ↓
Record written to SQLite
        ↓
Result returned to UI

### View Credential

Authenticated user selects credential
        ↓
UI requests record
        ↓
Rust reads protected data
        ↓
Required data is made available to the application
        ↓
Password remains visually masked by default

---

## 6. Proposed Database Model

### Vault Metadata

Possible fields:

- ID
- Password verification information
- Salt / security metadata
- Created timestamp

### Credentials

Possible fields:

- ID
- Service name
- Username
- Protected password
- Protected or appropriately handled notes
- Created timestamp
- Updated timestamp

The final schema will be documented after the SQLite implementation is
established.

---

## 7. Architecture Principles

The implementation should follow these principles:

- Desktop-first application
- Local-first storage
- Separation of UI, backend and persistence concerns
- No custom cryptographic algorithms
- Minimum necessary functionality before enhancements
- Requirement-driven development
- Testable components and workflows
- Clear error handling
- Maintainable source structure

---

## 8. Proof-of-Concept Objective

Before implementing the complete password manager, the project should
demonstrate:

1. The Tauri desktop application launches.
2. The Svelte interface renders successfully.
3. The frontend can invoke a Rust command.
4. Rust can return data to the frontend.
5. SQLite integration can be established.
6. A simple database create/read operation can be demonstrated.

This proof of concept reduces the risk of discovering major integration
problems late in development.

---

## 9. Relationship to Requirements

The architecture supports the requirements baseline, particularly:

- FR-01 — Initial Vault Setup
- FR-02 — Authentication
- FR-03 — Logout / Vault Lock
- FR-04–FR-07 — Credential CRUD
- FR-09 — Password Masking/Reveal
- NFR-01 — Secure Credential Storage
- NFR-02 — Master Password Protection
- NFR-03 — Authentication Protection
- NFR-08 — Reliability
- NFR-09 — Maintainability
- NFR-10 — Desktop Constraint

---

## 10. Open Technical Decisions

The following decisions remain to be verified during implementation:

- Exact Rust SQLite library
- Exact password-hashing/key-derivation library
- Exact authenticated-encryption library
- Final database schema
- Application directory structure
- Error-handling strategy
- Session/vault-lock state management

These items should not be recorded as final decisions until they have
been investigated and implemented.

---

## 11. Review Status

Technical Architecture v1.0 represents the initial proposed
architecture.

The document will be updated if implementation or proof-of-concept
work demonstrates that architectural changes are required.
