# Episodic Memory Design

**Generator Command:** Generator Command — Relational State, B/R, and Memory Reconciliation §5, §6, §7A  
**Project:** `abr-minimal-ai`  
**Source:** Frozen Kernel V8 · `abr-irrs` commit `2a58e46`  
**Preceding specification:** `continuous_relational_evolution.md` · `abr-minimal-ai` commit `899fcdc`  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Mathematical specification only. No kernel modification. No implementation.

---

## 1. Purpose

This document specifies:
- The three forms of information and their distinct roles
- The minimal episodic record for the robotics domain
- The episodic admission criterion (what makes an interaction "consequential")
- The relational recall specification: comparison, relevance, retrieval, and admission
- How episodic information informs subsequent calculations without entering A, B, or R directly

---

## 2. Three Forms of Information

The following taxonomy applies to all information the system retains. Each form has a distinct role and must not be substituted for another.

### 2.1 Active State

Information required for the present relational evaluation. Everything in the active state directly participates in at least one declared operator.

| Variable | Operator dependency | Scope |
|----------|-------------------|-------|
| `tau_s_prior` | A (scalar persistence difference) | Single step |
| `tau_s_new` | A, M admission | Present observation |
| `theta_new` | C projection | Present observation |
| `sigma_v` | Sigma accumulation, C projection | Trajectory-persistent |
| `e_prior` (PersistenceState) | A_p (Phase 2 persistence) | Single step |

Active state is precise and minimal. Every variable in active state is provably required by a declared operator. Nothing is retained in active state because it might be useful later.

### 2.2 Episodic Memory

Selected records of past interactions and their observed outcomes. Episodic memory is retained because past configurations can inform decisions about similar present configurations — not because the operators require prior history.

**Episodic memory is categorically outside the active relational boundary.** It is not admitted into A, B, or R. It does not modify the EdgeField. It informs the C projection layer through a declared recall mechanism.

Episodic memory has no declared structure in Kernel V8. Its design is entirely at the domain layer and requires Origin authorization for the admission criterion and recall mechanism.

### 2.3 Reference Information

Immutable declared constants, topology, and operator provenance that are used at every step. Reference information does not change with observations and does not accumulate.

| Item | Type | Contents |
|------|------|---------|
| DeclaredRelations | Topology | 4 loci, 3 edges, provenance, adj⁺/adj⁻ |
| M constants | Declared constants | K_T, R_W, V_BUS, V_RAISE, TAU_ELEC_MAX |
| Kernel version | Provenance | "V8 (2026-09-27; discipline 2026-09-29)" |
| Domain commit | Provenance | `abr-robotics-demo@09ea3052` |

Reference information is reusable across all steps and does not vary per evaluation.

---

## 3. Minimal Episodic Record

### 3.1 Record Structure

An episodic record preserves the declared information required to remember a consequential interaction.

```
EpisodeRecord {
    // --- Temporal marker ---
    step_index: u64,           // Step number at which the episode was recorded

    // --- Initial relational configuration ---
    initial_tau_s: f64,        // tau_s_prior at the step of the episode (N·m)
    initial_theta: f64,        // theta at the step of the episode (rad)
    initial_sigma_v: f64,      // sigma_v before this step's update (V)

    // --- Significant observed change ---
    a_out: f64,                // A_out = tau_s_new − tau_s_prior (N·m)
    delta_v: f64,              // Δ_V = B[e_IV] = A_out × R_W/K_T (V)

    // --- Applied command ---
    v_cmd: f64,                // v_cmd issued at this step (V)

    // --- Resulting configuration (admitted subsequently) ---
    resulting_tau_s: f64,      // tau_s admitted at step_index + 1 (N·m); OPEN if not yet observed
    resulting_theta: f64,      // theta admitted at step_index + 1 (rad); OPEN if not yet observed

    // --- Outcome classification ---
    outcome: EpisodeOutcome,   // Disturbance | Normal | LimitApproach | OPEN

    // --- References ---
    kernel_version: &str,      // "V8 (2026-09-27)"
    domain_commit: &str,       // "abr-robotics-demo@09ea3052"
    declaration_ref: &str,     // "m_declaration.md"
}

enum EpisodeOutcome {
    Disturbance,               // |A_out| > ε_episode (declared threshold)
    Normal,                    // |A_out| ≤ ε_episode
    LimitApproach,             // |tau_s_new| approaching TAU_ELEC_MAX
    // Additional outcomes: Origin declaration required
}
```

**What the record does NOT store:**
- The full EdgeField — not required to remember the interaction
- Intermediate operator outputs (A, B) — derivable from initial_tau_s, a_out, declared constants
- Plant dynamics variables (MASS, ARM, G) — not in observable extent, not in active state
- All prior tau_s values — only the immediately relevant pair is stored

### 3.2 Episodic Admission Criterion

An interaction is recorded as an episode when it meets a declared threshold of consequence. The criterion must be declared through M.

**Current status: OPEN — OC-EM-1**

Candidate criterion: |A_out| > ε_episode, where ε_episode is a declared threshold in N·m.

Candidate values from existing declarations:
- ε_episode = TAU_STATIC = 14.709 N·m (characteristic gravitational torque at THETA_CROSS)
- ε_episode = 0.1 × DIST_MAG = 2.5 N·m (10% of declared disturbance magnitude)
- ε_episode based on a declared noise floor (not currently declared)

**OC-EM-1:** The episodic admission criterion ε_episode is not declared through M. Return to Origin with candidate analysis above.

Additional admission conditions that require Origin declaration:
- Maximum episode count (how many episodes are retained)
- Episode replacement policy when the record is full (FIFO, significance-ranked, etc.)
- Retention duration (episode lifetime in steps or time)

---

## 4. Relational Recall Specification

### 4.1 When Recall Is Triggered

Recall is triggered when the present relational configuration resembles a configuration for which an episode exists. The trigger criterion is a declared comparison relationship.

**Current status: OPEN — OC-EM-2**

Candidate trigger: |A_out(present)| > ε_recall_trigger, where ε_recall_trigger is a declared recall-trigger threshold (N·m).

### 4.2 Comparison Relationships

Episodic recall requires comparison relationships between the present configuration and stored episode fields. Each comparison must be declared.

| Comparison | Declared criterion | Physical meaning | Status |
|-----------|-------------------|-----------------|--------|
| Torque change similarity | |A_out(present) − episode.a_out| ≤ ε_A | Similar relational change magnitude | **OPEN — ε_A undeclared** |
| Initial torque similarity | |tau_s_new(present) − episode.initial_tau_s| ≤ ε_τ | Similar starting configuration | **OPEN — ε_τ undeclared** |
| Angle similarity | |theta(present) − episode.initial_theta| ≤ ε_θ | Similar joint position | **OPEN — ε_θ undeclared** |

All comparison thresholds (ε_A, ε_τ, ε_θ) must be declared through M. No candidate values are proposed here — their appropriate scales depend on the domain's operational requirements, which require Origin input.

**OC-EM-2:** Relational recall comparison thresholds are not declared through M. Return to Origin.

### 4.3 Declared Criterion for Relevance

An episode is relevant when ALL of the following hold (proposed; not yet declared):
1. The comparison criteria in §4.2 are met
2. The episode.outcome is not OPEN (the outcome was observed and classified)
3. The episode is from a step within a declared relevance window (recency criterion — OPEN)

**The relevance criterion is not fully declarable from existing domain information.** Return to Origin with the above structure as a candidate.

### 4.4 Information Returned from a Matching Episode

When a relevant episode is retrieved, the following information is returned to the C layer:

| Retrieved field | Use in C projection | Admission path |
|-----------------|-------------------|----------------|
| `episode.a_out` | Contextual confirmation of expected relational change | C layer only — not into A, B, or R |
| `episode.delta_v` | Prior Δ_V for similar configuration | C layer only |
| `episode.v_cmd` | Prior command issued in similar configuration | C layer reference |
| `episode.outcome` | Classification of prior interaction | C layer decision logic |
| `episode.resulting_tau_s` | Prior observed result of similar command | C layer only |

The retrieved information is used as contextual reference in the C projection. It is NOT:
- Re-admitted as a current observation through M
- Placed into the NodeField
- Fed into operator_a, operator_b, or operator_r

### 4.5 Admission of Retrieved Information into Subsequent Calculation

Retrieved episode information can inform v_cmd through the C projection only through a **declared episodic adjustment mechanism**. This mechanism must specify:

1. How the retrieved v_cmd (past) modifies the computed v_cmd (present)
2. What weight or confidence factor applies to the retrieved information
3. The adjustment formula (additive, multiplicative, threshold-based, etc.)

**This mechanism is not yet declared. OC-EM-3:** The episodic adjustment mechanism for the C projection is OPEN. Return to Origin.

Until OC-EM-3 is resolved, retrieved episode information is available for inspection and logging but does not enter the operator chain or command computation.

### 4.6 Distinguishing Past from Present

A remembered observation is distinguished from a current observation by:

| Attribute | Current observation | Recalled episode |
|-----------|-------------------|-----------------|
| Admitted through M | YES — checked against TAU_ELEC_MAX | NO — tau_s in episode is a historical record |
| Operator input | YES — enters NodeField → A | NO — never enters operator chain |
| Step index | `step_index_current` | `episode.step_index ≠ step_index_current` |
| Uncertainty | Bounded by sensor accuracy | Additional uncertainty from time elapsed |
| Dimensional status | OBSERVATION-GROUNDED | OBSERVATION-GROUNDED at time of recording |

The step_index provides an unambiguous temporal label. A recalled episode's `initial_tau_s` is not a "present τ_s" — it is `episode.initial_tau_s` with a historical provenance label.

---

## 5. Situational Relational Boundary and Episodic Memory (§7A)

From `relational_state_architecture.md` §3, episodic records are outside the active relational boundary. They interact with the boundary through a single interface: the declared recall mechanism in §4.

The key constraint from Generator Command §5: "Do not treat episodic memory as an input to R or B without an explicit domain mapping."

This constraint defines the interface precisely:
- Episodic memory may enter the C projection (command generation) through a declared mapping
- Episodic memory may NOT enter A, B, or R without a declared domain mapping that transforms the historical record into a current observable

A domain mapping that would admit episodic information into A/B/R would require:
1. A declared transformation from `episode.initial_tau_s` to a current NodeField value
2. Origin authorization
3. A physical justification that the episode represents a present observation

No such mapping is proposed or authorized at M0. Return to Origin if such a mapping is desired.

---

## 6. Open Conditions Summary

| Condition | Subject | Status |
|-----------|---------|--------|
| OC-EM-1 | Episodic admission criterion ε_episode | OPEN — return to Origin |
| OC-EM-2 | Recall comparison thresholds ε_A, ε_τ, ε_θ | OPEN — return to Origin |
| OC-EM-3 | Episodic adjustment mechanism for C projection | OPEN — return to Origin |
| Maximum episode count | Episode storage bound | OPEN — return to Origin |
| Episode replacement policy | Memory management | OPEN — return to Origin |
| Recency/relevance window | Temporal scope of recall | OPEN — return to Origin |

---

*Kernel V8 frozen at `abr-irrs` commit `2a58e46`. This document is historical evidence and must not be modified retroactively. Future declarations that resolve OPEN conditions shall reference this document by commit.*
