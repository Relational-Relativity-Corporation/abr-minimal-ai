# ABR Minimal AI

**Metatron Dynamics, Inc. — Minimal Mathematical Core for Relational Artificial Intelligence**

## Purpose

`abr-minimal-ai` develops and tests a minimal AI computational core grounded in **relation as the observable primitive**, using the established Metatron Dynamics relational operators.

The first objective is to implement and independently evaluate three functions commonly provided by neural architectures:

1. **Representation** — construct computational representations from declared observable relationships and their provenance.
2. **Transformation** — calculate changes in relational state through the frozen kernel operator process.
3. **Attention** — calculate task-relevant relational contributions without conventional query/key/value attention and softmax weighting.

These are **functional test targets**, not presumed one-to-one equivalents of kernel operators or a required sequence of neural-network-like layers.

The project begins with small, independent Rust demonstrations. Integration into a larger AI architecture follows only after the individual functions have reproducible evidence.

## Mathematical authority

- **Primitive:** relation is the only observable primitive.
- **Declaration:** no declaration, no evaluation. Each experiment identifies its domain `D`, admitted observables, mapping `M`, parameters, provenance, and evaluation criteria before operator execution.
- **Operators:** use the existing frozen Kernel V8 implementation; do not rederive, rename, silently substitute, or modify its operators.
- **Application:** operator activity must conform to the declared mapping and its scope.
- **Persistence:** where a task requires retained relational state, derive it through the declared operator process, including A→B→R when its prerequisites are satisfied. Do not substitute an independent state-update rule.
- **Boundaries:** record undeclared inputs, unavailable observations, and unresolved operator prerequisites as **OPEN**, not as implicit defaults.
- **Evaluation:** calculate from admitted relationships and return results to comparison with declared observables or task outcomes. Conformance to the kernel and success on the functional task are separate evaluations.

### Authoritative references

- [ABR IRRS](https://github.com/Relational-Relativity-Corporation/abr-irrs) — verification infrastructure, pinned kernel dependencies, and conformance process.
- Kernel V8 reference, derived invariants, and role-separation documents: use the versioned authoritative files and commit identifiers recorded in each experiment's evidence package. Do not assume a moving branch head is a frozen reference.
- [ABR Relational Attention](https://github.com/Relational-Relativity-Corporation/abr-relational-attention) — earlier relevant implementation; treat as research input, not automatically as a conformant dependency.

Before the first implementation, the Generator must locate and record the exact authorized kernel source revision and the applicable IRRS conformance contract. If either is unavailable, leave that dependency OPEN.

## V0.1 scope: three independent functions

| Module | Minimal behavior | First evidence requirement |
| --- | --- | --- |
| Representation | Admit observations and construct a declared relational representation with provenance | Reconstruct the representation from the same declared inputs; reject undeclared inputs |
| Transformation | Apply the frozen operator process to calculate a changed relational state | Trace inputs, mapping, operators, outputs, and any persistence requirements |
| Attention | Establish which admitted relationships contribute to a declared task output, and how those contributions change with observations | Compare output behavior against an explicit attention-related task without implementing Q/K/V or softmax as the relational solution |

The experiments must not rely on hard-coded expected answers, undeclared weights, or task-specific shortcuts disguised as relational mathematics.

Conventional reference implementations may be included **only as clearly separated baselines**. Equivalent input tasks, evaluation metrics, and computation accounting must be declared for comparisons.

## Development sequence

**M0 — Foundation and declarations**

- Initialize a small Rust workspace.
- Pin the authorized kernel and IRRS reference revisions.
- Establish machine-readable experiment declarations, deterministic fixtures, provenance records, and test/evidence conventions.
- Define the first representation task and its independent acceptance criteria.
- Run the applicable IRRS conformance gate before making claims of kernel conformance.

**M1 — Relational representation**

- Implement the smallest declared observable domain and representation.
- Demonstrate provenance, reproducibility, and response to changed observations.
- Publish tests and an evidence package.
- Stop for Origin review and independent Verifier disposition.

**M2 — Relational transformation**

- Add only the transformation behavior authorized by its declaration.
- Trace the frozen operator sequence and any required persistence.
- Publish tests, outputs, and evidence; stop for review.

**M3 — Relational attention**

- Define the attention-related functional task independently of its solution.
- Implement relational contribution calculation using admitted relationships and the frozen kernel.
- Compare with a separately labeled conventional baseline where useful.
- Publish tests, outputs, and evidence; stop for review.

**M4 — Minimal composition**

- Combine only the admitted functions.
- Demonstrate an end-to-end observation-to-output process.
- Report correctness, compute cost, memory use, and limits under the same declared task conditions.

## Proposed repository layout

```text
abr-minimal-ai/
├── README.md
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs
│   ├── representation.rs
│   ├── transformation.rs
│   └── attention.rs
├── declarations/
├── fixtures/
├── tests/
├── evidence/
└── docs/
    ├── decisions.md
    └── handoff.md
```

This is a proposed structure, not a claim that the files already exist.

## Triad workflow

- **Origin — human authorization:** chooses goals, declares or approves `D`, `M`, observables, evaluation criteria, and scope changes.
- **Generator — Claude Code:** implements the authorized task in the local clone, runs tests, collects reproducible evidence, and pushes reviewed work to GitHub.
- **Verifier — ChatGPT:** reads the pushed revision, independently checks the declaration, code paths, kernel conformance evidence, provenance, and functional results; returns **PASS / FAIL / OPEN** with reasons.
- **Origin** authorizes subsequent milestones. Neither Generator nor Verifier silently resolves open declarations.

**Standard handoff record:** commit SHA; authorized kernel/IRRS revisions; declaration and fixture hashes; exact commands; toolchain versions; test output; operator trace; functional metrics; PASS/FAIL/OPEN findings; unresolved questions; proposed next action.

Local test output must not be described as independently rerun by the Verifier unless the Verifier has actually executed it. GitHub Actions or another reproducible execution environment can supply independently inspectable run evidence.

## Repository as the persistent handoff

Keep `README.md` focused on stable project intent and governance. Maintain the latest operational state in `docs/handoff.md`, and log Origin decisions in `docs/decisions.md`. Commit small, auditable evidence packages under `evidence/` with references to exact code and kernel revisions.

**Security:** This repository is public. Never commit access tokens, credentials, private keys, confidential partner material, personal data, or proprietary datasets without explicit publication authorization. Use GitHub secrets or a separate approved private store for sensitive material. Git history is durable; deleting a secret in a later commit does not reliably remove its exposure.

## Immediate Generator handoff

**Authorized planning scope:** M0 only. Do not implement representation, transformation, or attention yet.

1. Inspect the repository and the authoritative IRRS/kernel references.
2. Propose the minimal Rust workspace and exact pinned dependencies.
3. Draft the first representation experiment's `D`, observable inputs, `M`, provenance schema, and acceptance criteria for Origin approval.
4. Establish reproducible test and evidence commands.
5. Report unresolved declarations as OPEN; do not invent them.
6. Return a proposed implementation plan for Origin authorization and Verifier review.

**Current status:** repository established; implementation milestones not yet admitted or executed.

---

Metatron Dynamics, Inc.  
Project: `abr-minimal-ai`  
Governing principle: **Relation as observable primitive. Function defines role. No declaration, no evaluation.**
