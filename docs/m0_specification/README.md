# M0 Mathematical Specification — Minimal Relational AI Computational Core

**Repository:** abr-minimal-ai  
**Project:** Metatron Dynamics × Relational Relativity Corporation  
**Kernel authority:** Metatron Dynamics Kernel V8 (2026-09-27; discipline 2026-09-29) — frozen  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Verifier:** ChatGPT (independent)  
**Origin:** Human authorization  
**Date:** 2026-10-25  
**Status:** M0 complete — awaiting independent Verifier review  

---

## Objective

Establish whether representation, transformation, and attention can be accomplished directly through relational mathematics, beginning with **relation as the only observable primitive**.

This document is the M0 output: a mathematical specification only. No implementation has been produced.

---

## Contents

- [`m0_spec.html`](m0_spec.html) — Full specification document (rendered)  
- Published artifact: https://claude.ai/artifact/H7oE3CXMByaHYUhqakZoth

---

## Task summary

| Task | Subject | Status |
|------|---------|--------|
| 1 | Minimal observational domain (3-node path topology, interior-locus) | Complete |
| 2 | Relational representation via A→B→R operator chain | Complete |
| 3 | Independent evaluation criteria (A.1–A.9), Kernel V8 conformance vs functional success | Complete |
| 4 | Path to transformation and attention without new operators | Complete |
| 5 | Evidence package: declaration, worked example, acceptance criteria, proposed Rust plan | Complete |

---

## Kernel V8 references

All mathematical content derives from the frozen Kernel V8 in [`abr-irrs`](https://github.com/Relational-Relativity-Corporation/abr-irrs):

| File | Content |
|------|---------|
| `irrs-kernel/src/operators.rs` | A, B, R operators; ρ computation |
| `irrs-kernel/src/derived_invariants.rs` | DETECTION_TOLERANCE = 1e-12 |
| `irrs-kernel/src/region.rs` | RegionBuilder; interior-locus condition |
| `irrs-kernel/src/lib.rs` | Foundational primitive; three characteristics |
| `docs/v9_provenance/phase1_gap_matrix.md` | V9 gap matrix (base commit 1400d3bb) |

---

## OPEN conditions

Four conditions remain open at M0; none were resolved by this specification:

1. **RhoSaturatingForm** (§11.2, §18) — functional form of ρ(ρ_base, χ, χ₀) not authorized
2. **RhoMaxSelectionRule** (§11.2, §18) — max-selection rule for χ not authorized  
3. **χ[e] for acyclic path topology** — definition of antisymmetric circulation on a path graph (no cycles) not established
4. **Illustrative parameter provenance** — all declared values are ParameterProvenance::Illustrative; no EvidenceRecord or VerificationRecord submitted

---

## Stop condition

M0 is complete. No implementation modules have been created. No AI architecture has been introduced. No M1 work has been initiated. Origin authorization is required before any of these proceed.

---

## Governing rules (verbatim from Generator Instruction)

1. Relation is the only observable primitive.
2. No declaration, no evaluation.
3. Function defines role.
4. Every calculation must remain within a declared domain D and mapping M.
5. Use frozen Kernel V8 as the operator authority. Do not modify, reinterpret, or rederive its operators.
6. Preserve all operator prerequisites, provenance requirements, and OPEN conditions.
7. Calculated results must return to comparison with declared observables.
