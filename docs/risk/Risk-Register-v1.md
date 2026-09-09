# VaultFive Risk Register v1.0

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Document | Risk Register |
| Version | 1.0 |
| Status | Initial Risk Baseline |
| Prepared by | Caleb |
| Review Status | Pending Team Review |

## 1. Purpose

This risk register identifies and evaluates risks that may affect the
successful delivery of VaultFive.

Risks will be reviewed throughout the project. New risks may be added,
existing ratings may change, and mitigation actions will be updated as
the project progresses.

## 2. Risk Rating Method

Likelihood and impact are rated from 1 to 5.

**Risk Score = Likelihood × Impact**

| Score | Classification |
|---|---|
| 1–4 | Low |
| 5–9 | Medium |
| 10–16 | High |
| 17–25 | Critical |

## 3. Risk Register

| ID | Risk | Likelihood | Impact | Score | Rating | Mitigation / Response | Owner | Status |
|---|---|---:|---:|---:|---|---|---|---|
| R-01 | Security-sensitive functionality may be implemented incorrectly, exposing stored credentials. | 3 | 5 | 15 | High | Use established cryptographic libraries, avoid custom cryptography, review security design and execute security-oriented tests. | Technical / QA | Open |
| R-02 | Additional features may expand the scope and prevent completion of the core product. | 3 | 4 | 12 | High | Use MoSCoW priorities and complete Must-have requirements before Should/Could enhancements. | Caleb | Open |
| R-03 | Integration problems may occur between the frontend, Rust backend and SQLite storage. | 3 | 4 | 12 | High | Produce an early technical proof of concept and integrate components incrementally. | Technical | Open |
| R-04 | The accelerated September schedule may leave insufficient time for development, testing and documentation. | 4 | 5 | 20 | Critical | Follow the accelerated Gantt plan, prioritise the minimum viable product, review progress frequently and defer non-essential enhancements if required. | Caleb | Open |
| R-05 | Limited availability of team members may delay allocated tasks. | 5 | 4 | 20 | Critical | Track availability, identify blocked work early, reallocate critical tasks when necessary and retain accurate evidence of actual contributions. | Caleb | Active |
| R-06 | Concentration of work on the project manager may create a workload bottleneck and reduce time available for coordination and review. | 5 | 4 | 20 | Critical | Prioritise critical-path work, use documented task takeover where necessary, seek team review when members become available and avoid unnecessary functionality. | Caleb | Active |
| R-07 | Insufficient testing time may allow defects to remain in the submitted product. | 4 | 5 | 20 | Critical | Develop the test plan before implementation is complete, test incrementally, record defects and reserve time for regression testing. | QA / Team | Open |
| R-08 | Individual contribution evidence may be insufficient if team participation remains limited. | 4 | 4 | 16 | High | Use individual GitHub accounts, issues, commits and genuine meeting/evidence records. Do not attribute work to members who did not perform it. | Caleb / Team | Active |
| R-09 | Version-control conflicts or accidental changes may damage working project files. | 2 | 3 | 6 | Medium | Use meaningful commits, pull before editing shared work and use branches/pull requests when development activity increases. | Team | Open |
| R-10 | Accessibility may receive insufficient attention because of schedule pressure. | 3 | 3 | 9 | Medium | Include accessibility considerations in UI design and execute an accessibility/usability review before finalisation. | UI/UX / QA | Open |
| R-11 | Requirements may change during development and cause rework. | 3 | 3 | 9 | Medium | Maintain the requirements baseline, record significant changes and assess impact on design, schedule, risks and testing. | Caleb | Open |
| R-12 | Documentation may be left until too late, resulting in missing lifecycle evidence. | 3 | 4 | 12 | High | Maintain documentation alongside development and preserve milestone, testing, issue and decision evidence throughout the project. | Caleb / Team | Open |

## 4. Current Priority Risks

The most significant current risks are:

- R-04 — Accelerated project schedule
- R-05 — Team-member availability
- R-06 — Workload concentration
- R-07 — Insufficient testing time
- R-08 — Individual contribution evidence

These risks require regular review because they could directly affect
the ability to complete and evidence the project.

## 5. Risk Review Process

The risk register should be reviewed during project/sprint reviews.

During each review, the team should consider:

- Whether a new risk has appeared
- Whether likelihood has changed
- Whether impact has changed
- Whether mitigation has been effective
- Whether an existing risk has become an active issue
- Whether a risk can be closed

Changes should be retained as project-management evidence rather than
rewriting the project history.

## 6. Current Risk Response

The current project response is to protect the Must-have product scope,
progress critical dependencies, maintain accurate GitHub evidence and
reserve sufficient time for testing and defect correction.

Where planned responsibilities cannot be completed because of member
availability, critical work may be temporarily reallocated. Actual
contribution records must continue to reflect who performed the work.

## 7. Review Status

Risk Register v1.0 represents the initial project risk baseline.

The document remains subject to review and should be updated as the
project progresses.
