# VaultFive Risk Register v1.1

## Document Control

| Field | Details |
|---|---|
| Project | VaultFive |
| Team | Team 4 |
| Document | Risk Register |
| Version | 1.1 |
| Status | Post-Sprint 2 Risk Review |
| Prepared by | Caleb |
| Review Status | Updated following Sprint 2 |

## 1. Purpose

This risk register identifies and evaluates risks that may affect the successful delivery of VaultFive.

Risks are reviewed throughout the project. New risks may be added, existing ratings may change, and mitigation actions may be updated as the project progresses.

This version records the risk review completed following Sprint 2 implementation and testing. Previous risk ratings are retained through version-control history as project-management evidence.

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
| R-01 | Security-sensitive functionality may be implemented incorrectly, exposing stored credentials. | 2 | 5 | 10 | High | Established cryptographic libraries have been used. Security-related implementation and locked-vault protection have been tested. Repeat relevant security tests if the implementation changes. | Technical / QA | Reduced |
| R-02 | Additional features may expand the scope and prevent completion of the core product. | 1 | 4 | 4 | Low | Must-have requirements have been prioritised and the main product scope has been implemented. Avoid unnecessary new functionality before submission. | Caleb | Reduced |
| R-03 | Integration problems may occur between the frontend, Rust backend and SQLite storage. | 1 | 4 | 4 | Low | Frontend, Rust backend and SQLite storage are now integrated and the main workflows have passed testing. Repeat regression testing if integration code changes. | Technical | Reduced |
| R-04 | The accelerated September schedule may leave insufficient time for development, testing and documentation. | 2 | 5 | 10 | High | Core implementation and Sprint 2 testing are complete. Remaining time should be protected for documentation, evidence, final review and submission readiness. | Caleb | Reduced |
| R-05 | Limited availability of team members may delay allocated tasks. | 4 | 4 | 16 | High | Continue tracking availability, identify blocked work early, reallocate critical final-stage tasks where necessary and retain accurate evidence of actual contributions. | Caleb | Active |
| R-06 | Concentration of work on the project manager may create a workload bottleneck and reduce time available for coordination and review. | 4 | 4 | 16 | High | Prioritise submission-critical work, avoid unnecessary development, document genuine task takeover and obtain team review where available. | Caleb | Active |
| R-07 | Insufficient testing time may allow defects to remain in the submitted product. | 1 | 5 | 5 | Medium | All 18 planned tests have been executed and passed. Relevant regression tests should be repeated if application code changes before submission. | QA / Team | Reduced |
| R-08 | Individual contribution evidence may be insufficient if team participation remains limited. | 4 | 4 | 16 | High | Continue using individual GitHub accounts, issues, commits and genuine meeting/evidence records. Do not attribute work to members who did not perform it. | Caleb / Team | Active |
| R-09 | Version-control conflicts or accidental changes may damage working project files. | 2 | 3 | 6 | Medium | Continue using meaningful commits, check repository status before commits, pull remote changes before editing shared work and avoid destructive Git operations. | Team | Open |
| R-10 | Accessibility may receive insufficient attention because of schedule pressure. | 1 | 3 | 3 | Low | Basic keyboard accessibility testing has been completed successfully. Maintain existing accessibility behaviour and repeat checks if the interface changes. | UI/UX / QA | Reduced |
| R-11 | Requirements may change during development and cause rework. | 1 | 3 | 3 | Low | Maintain the requirements baseline and avoid unnecessary scope changes during the final project stage. Assess any essential change against design, testing and submission impact. | Caleb | Reduced |
| R-12 | Documentation may be left until too late, resulting in missing lifecycle evidence. | 2 | 4 | 8 | Medium | Requirements, design, planning, testing and Sprint review documentation have been maintained. Complete the final report and evidence review before submission. | Caleb / Team | Reduced |

## 4. Current Priority Risks

Following the Sprint 2 review, the most significant current risks are:

- R-05 — Team-member availability
- R-06 — Workload concentration
- R-08 — Individual contribution evidence

These remain active because the project has entered the final documentation and submission-readiness stage and genuine team participation and contribution evidence remain important.

R-04 has reduced because the core implementation and planned testing are complete, although schedule pressure remains relevant until final submission.

R-07 has also reduced because all 18 planned test cases have now been executed and passed.

## 5. Risk Review Process

The risk register should continue to be reviewed during the final project stage.

During each review, the team should consider:

- Whether a new risk has appeared
- Whether likelihood has changed
- Whether impact has changed
- Whether mitigation has been effective
- Whether an existing risk has become an active issue
- Whether a risk can be closed or reduced

Changes should be retained as project-management evidence rather than rewriting the project history.

Version-control history should therefore preserve the original v1.0 baseline and the subsequent risk review.

## 6. Current Risk Response

The current project response has moved from core implementation towards final quality assurance and submission readiness.

All 18 planned test cases have been executed and passed, reducing the immediate functional-testing risk.

The project should now protect the tested application from unnecessary changes and focus on documentation, evidence, final review and submission preparation.

If application code is changed before submission, relevant regression tests should be repeated.

Where planned responsibilities cannot be completed because of member availability, critical work may still be temporarily reallocated. Actual contribution records must continue to reflect who performed the work.

## 7. Post-Sprint 2 Risk Review

The Sprint 2 review resulted in the following changes:

- R-01 reduced following implementation and security verification.
- R-02 reduced because the core scope is implemented and unnecessary additional features can now be avoided.
- R-03 reduced because frontend, Rust and SQLite integration is working and has been tested.
- R-04 reduced because core implementation and Sprint 2 testing are complete.
- R-05 remains active because team-member availability continues to affect project coordination.
- R-06 remains active because workload remains concentrated on the project manager.
- R-07 reduced following completion of all 18 planned test cases.
- R-08 remains active because individual contribution evidence remains important for the assessment.
- R-09 remains open because version-control problems remain possible until submission.
- R-10 reduced following successful basic keyboard accessibility testing.
- R-11 reduced because the project is now moving into finalisation and major requirement changes should be avoided.
- R-12 reduced because lifecycle documentation has been maintained throughout development.

No new critical technical risk was identified during the Sprint 2 review.

## 8. Review Status

Risk Register v1.1 represents the post-Sprint 2 risk review.

The original v1.0 baseline is preserved through Git version history.

The main technical and testing risks have reduced following implementation and successful execution of all 18 planned test cases.

R-05, R-06 and R-08 remain the main active project-management risks and should continue to be monitored during final documentation and submission preparation.