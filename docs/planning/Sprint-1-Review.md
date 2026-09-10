# VaultFive Sprint 1 Review

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Sprint | Sprint 1 — Project Baseline & Technical Proof |
| Review Date | 10 September 2026 |
| Prepared by | Caleb |
| Status | Sprint Review |

## 1. Sprint Objective

The objective of Sprint 1 was to establish the initial project
baseline, organise the project-management environment and demonstrate
that the proposed desktop technology stack was technically viable.

The sprint focused on planning, requirements, risk, testing, design
and an early technical proof of concept.

---

## 2. Work Completed

The following work was produced during Sprint 1:

### Project Management

- GitHub repository established.
- Team members invited as collaborators.
- GitHub issues created for Sprint 1 activities.
- Labels and Sprint 1 milestone established.
- VaultFive Development Board established.
- Accelerated project schedule/Gantt created.
- Current project status documented.

### Requirements

- Requirements Baseline v1.0 created.
- Functional requirements FR-01 to FR-11 documented.
- Non-functional requirements NFR-01 to NFR-10 documented.
- User stories created.
- MoSCoW prioritisation established.
- Initial requirements traceability established.

### Risk Management

- Risk Register v1.0 created.
- Initial project risks identified.
- Likelihood and impact assessed.
- Mitigation actions documented.
- Current active risks identified.

### Quality Assurance

- Test Plan v1.0 created.
- Initial testing strategy documented.
- 18 planned test cases defined.
- Functional, negative, integration, security-oriented,
  accessibility and regression testing considered.
- Test execution remains pending implementation.

### Technical Architecture

- Technical Architecture v1.0 created.
- Tauri + Svelte + TypeScript + Rust architecture documented.
- Desktop Tauri application successfully launched.
- Svelte frontend successfully invoked a Rust command.
- Rust response successfully returned to the frontend.
- SQLite integration established using rusqlite.
- SQLite proof-of-concept create/write/read operation completed
  successfully.

### UI/UX

- Initial six-screen UI/UX wireframe produced.
- UI/UX Design v1.0 documented.
- Core workflow represented from vault setup through vault locking.
- Accessibility and security-related UX considerations documented.

---

## 3. Sprint 1 Evidence

| Area | Evidence |
|---|---|
| Planning | `docs/planning/Project-Status.md` |
| Schedule | `docs/planning/VaultFive_Accelerated_Gantt_09-23_Sep_2026.xlsx` |
| Requirements | `docs/requirements/Requirements-Baseline-v1.md` |
| User Stories | `docs/requirements/User-Stories-and-MoSCoW-v1.md` |
| Risk | `docs/risk/Risk-Register-v1.md` |
| Testing | `docs/testing/Test-Plan-v1.md` |
| Architecture | `docs/design/Technical-Architecture-v1.md` |
| UI/UX Design | `docs/design/UI-UX-Design-v1.md` |
| Wireframes | `docs/design/VaultFive-UI-Wireframes-v1.png` |
| Application PoC | `app/` |
| Activity Evidence | GitHub issues, commits and project board |

---

## 4. Requirements Status

The requirements baseline is sufficiently established to begin
implementation.

Must-have requirements will take priority during the next development
phase.

Should-have and Could-have requirements may be deferred if they
threaten completion of the Must-have product scope.

---

## 5. Technical Proof-of-Concept Outcome

The proposed architecture has passed the initial technical
proof-of-concept stage.

The following path has been demonstrated:

Svelte / TypeScript UI
        ↓
Tauri
        ↓
Rust backend
        ↓
SQLite
        ↓
Rust response
        ↓
Svelte UI

This reduces the initial integration risk associated with the selected
technology stack.

The proof of concept does not demonstrate that the final password
manager or its security controls are complete.

---

## 6. Testing Status

The initial test plan has been prepared but the planned product test
cases have not yet been executed.

Test results must only be recorded after the relevant functionality
has been implemented and genuinely tested.

Failed tests and defects should be retained as part of the development
evidence and retested following correction.

---

## 7. Current Risks

Particular attention remains necessary for:

- Accelerated project schedule
- Limited team-member availability
- Concentration of workload
- Security-sensitive implementation
- Insufficient testing time
- Evidence of actual individual contribution

These risks remain subject to ongoing monitoring and mitigation.

---

## 8. Collaboration and Contribution Status

Initial responsibilities were allocated across Team 4.

Due to current availability constraints, the Project Manager has
progressed several critical-path activities to prevent the project
from becoming blocked.

GitHub records should continue to represent actual contributions
accurately. Work must not be attributed to a team member unless that
member genuinely contributed to it.

Where team members become available, planned responsibilities and
review activities can be resumed.

---

## 9. Items Carried Forward

The following activities move into the next project phase:

- Implement the VaultFive application interface.
- Implement initial vault setup.
- Implement authentication and vault locking.
- Implement credential CRUD functionality.
- Implement secure credential storage.
- Develop the final database schema.
- Execute planned tests against implemented functionality.
- Record defects and retesting.
- Review accessibility/usability.
- Continue updating project documentation and risks.
- Continue collecting genuine collaboration and development evidence.

---

## 10. Sprint Review Outcome

Sprint 1 established a usable project baseline and demonstrated the
technical viability of the proposed desktop architecture.

The project can therefore progress into core implementation while the
existing requirements, design, risk and testing artefacts remain under
controlled review.

Sprint 1 documentation may be updated where genuine review identifies
necessary changes, with significant changes retained through version
control.
