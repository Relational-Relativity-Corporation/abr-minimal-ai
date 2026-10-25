# State Retention Audit

**Generator Command:** Generator Command — Relational State, B/R, and Memory Reconciliation §8  
**Project:** `abr-minimal-ai`  
**Source:** Frozen Kernel V8 · `abr-irrs` commit `2a58e46`  
**Preceding specification:** `continuous_relational_evolution.md` · `abr-minimal-ai` commit `899fcdc`  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Mathematical specification only. No kernel modification. No implementation.

---

## 1. Purpose

This document audits every stored variable in the proposed minimal relational architecture. For each variable: provenance, mathematical or control function, required lifetime, and eliminability. The document closes with a complete worked example tracing one disturbance-frame step from observation through episodic record to later recall.

---

## 2. Variable Audit

### 2.1 Active State Variables

| # | Variable | Type | Provenance | Mathematical/Control function | Required lifetime | Eliminable? |
|---|----------|------|-----------|------------------------------|------------------|------------|
| 1 | `tau_s_prior` | f64 (N·m) | Admitted through M at previous step | Input to A operator: A_out = tau_s_new − tau_s_prior | From step k admission until A is evaluated at step k+1 | **NO** — required for Phase 1 A operator |
| 2 | `tau_s_new` | f64 (N·m) | Admitted through M at current step; passes M admission check | Present observation; source of A_out, NodeField values | Present step evaluation only; becomes tau_s_prior at step k+1 | **NO** — required for A, NodeField, M check |
| 3 | `theta` | f64 (rad) | Admitted through M at current step | C projection: theta_error = THETA_TARGET − theta | Present step; updated at each admission | **NO** — required for C projection |
| 4 | `sigma_v` | f64 (V) | Running sum of Δ_V = B[e_IV] from step 0 | Sigma accumulator: feeds C projection as v_cmd baseline | Entire trajectory — accumulates from first step | **NO** — required for C projection |
| 5 | `e_prior` (in PersistenceState) | EdgeField | `PersistenceState::from_prior(E_current)` constructed at previous step | Input to A_p: A_p[c][e] = E_current[c][e] − e_prior[c][e] | From previous step evaluation until A_p is computed at current step | **Conditionally NO** — required for Phase 2 (A_p→B_p→R_p). If Phase 2 is not evaluated, can be omitted; but Phase 2 is declared in `operator_e_v5`. At M0 with R OPEN, Phase 2 produces OPEN outputs. Eliminate only if Phase 2 is explicitly withdrawn from the architecture. |

### 2.2 Intermediate Variables (Computed, Not Retained)

These are computed during evaluation and released at step completion.

| # | Variable | Type | Source | Released after |
|---|----------|------|--------|---------------|
| 6 | `A_out` | f64 (N·m) | operator_a on NodeField | B computation |
| 7 | `Δ_I` | f64 (A) | A_out / K_T (domain coupling) | B input construction |
| 8 | `Δ_V` (= B[e_IV]) | f64 (V) | A_out × R_W / K_T (or B terminal output) | Sigma update |
| 9 | `B_EdgeField` | EdgeField | operator_b(A_EdgeField, DeclaredRelations) | PersistenceState construction (stores as e_prior); Sigma update |
| 10 | `rho` | Vec<f64> | compute_rho(A_EdgeField, ...) — OPEN | R operator — OPEN |
| 11 | `R_EdgeField` | EdgeField | operator_r(...) — OPEN | Not computed at M0 |
| 12 | `A_p_output` | Vec<Vec<f64>> | operator_a_persistence(E_current, e_prior) | B_p computation |
| 13 | `B_p_output` | Vec<Vec<f64>> | operator_b_persistence(A_p, DeclaredRelations) | R_p computation — OPEN |
| 14 | `rho_p` | Vec<f64> | compute_rho_persistence(...) — OPEN | R_p — OPEN |
| 15 | `R_p_output` | Vec<Vec<f64>> | operator_r_persistence(...) — OPEN | Not computed at M0 |
| 16 | `v_cmd` | f64 (V) | C projection: sigma_v_new + frac × V_RAISE | Applied to motor; not retained |

### 2.3 Reference Information (Immutable, Not Counted as State)

| # | Variable | Source | Shared across steps |
|---|----------|--------|---------------------|
| 17 | `DeclaredRelations` | M declaration | YES — fixed topology |
| 18 | `K_T = 1.0 N·m/A` | M constant | YES |
| 19 | `R_W = 0.7 Ω` | M constant | YES |
| 20 | `V_BUS = 48.0 V` | M constant | YES |
| 21 | `V_RAISE = 19.0 V` | M constant | YES |
| 22 | `TAU_ELEC_MAX = 68.57 N·m` | M constant | YES |
| 23 | `THETA_TARGET` | M declaration | YES |

### 2.4 Episodic Memory Variables (Optional, Outside Active State)

| # | Variable | Source | Lifetime | Eliminable from active state? |
|---|----------|--------|---------|-------------------------------|
| 24 | `episode_records: Vec<EpisodeRecord>` | Selected steps where |A_out| > ε_episode (OC-EM-1) | Until explicitly released; OC-EM-1 applies | YES — active state does not depend on episodic memory |

### 2.5 Minimal Active State Count

**4 variables are strictly required for continuous relational evolution:**

| Variable | Required by |
|----------|------------|
| `tau_s_prior` | A operator |
| `theta` | C projection |
| `sigma_v` | Sigma accumulation, C projection |
| `e_prior` (PersistenceState) | Phase 2 A_p |

Plus `tau_s_new` at each step, which transitions to `tau_s_prior` at the next step (it is the same variable in a persistent system).

---

## 3. Complete Worked Example — Disturbance Frame

This example traces the step at which the 25.0 N·m disturbance is admitted. It follows the canonical sequence:

**Observation → A → B → R status → present state → command → subsequent observation → episodic record → later recall**

All values are exact (no floating-point accumulation error shown).

---

### 3.1 Prior State (Step k−1 Completed)

Before step k, the system holds:

| Variable | Value | Provenance |
|----------|-------|-----------|
| `tau_s_prior` | 2.553 N·m | Admitted at step k−1; equals MASS×G×ARM×sin(THETA_CROSS) |
| `theta` | 0.17453 rad (10°) | Admitted at step k−1; arm at THETA_CROSS |
| `sigma_v` | 1.787 V | Accumulated Σ Δ_V from steps 0 to k−1. By telescoping: [τ_s(k−1) − τ_s(0)] × R_W/K_T = [2.553 − 0] × 0.7 = 1.787 V |
| `e_prior` | B EdgeField from step k−1 | spatial[0] = { e_persist: A_out_{k-1}, e_τI: Δ_I_{k-1}, e_IV: Δ_V_{k-1} } at prior step magnitudes |

---

### 3.2 Observation Admitted Through M

**New physical measurement at step k:**
- Disturbance fires: τ_disturbance = 25.0 N·m added to gravity torque
- τ_s(k) = τ_gravity(θ_k) + τ_disturbance ≈ 2.553 + 25.000 = 27.553 N·m
- θ(k) = 0.17453 rad (approximately unchanged in one step at 24 Hz)

**M admission check:**
```
|tau_s_new| = 27.553 N·m ≤ TAU_ELEC_MAX = 68.57 N·m   → ADMIT
```

**Status:** Observation admitted. ✓

---

### 3.3 A Operator

**Input:** tau_s_prior = 2.553 N·m, tau_s_new = 27.553 N·m  
**Source:** `operators.rs` lines 455–478

```
A_out = tau_s_new − tau_s_prior
      = 27.553 − 2.553
      = 25.000 N·m
```

**Coupling transformations (domain layer):**
```
Δ_I = A_out / K_T = 25.000 / 1.0 = 25.000 A
Δ_V = Δ_I × R_W  = 25.000 × 0.7 = 17.500 V
```

**A EdgeField (spatial component, k=1):**
```
A.spatial[0][e_persist] = +25.000 N·m   (persistence edge: tau_s change)
A.spatial[0][e_τI]      = +25.000        (coupling edge: Δ_I in A)
A.spatial[0][e_IV]      = +17.500        (coupling edge: Δ_V in A)
```

*Note: A[e_τI] and A[e_IV] represent domain-layer coupling values. If computed by operator_a on the 4-locus NodeField with K_T=1.0, A[e_τI] = τ_s_new − τ_s_new/K_T = 0. The values 25.000 and 17.500 are placed in the EdgeField through the domain-layer coupling path, not through operator_a on the coupling edges. This is the source of OC-M0R-6.*

---

### 3.4 B Operator

**Input:** A EdgeField from §3.3  
**Source:** `operators.rs` lines 480–500

```
B[e_persist] = A[e_persist] + A[e_τI]    (adj⁺(e_persist) = {e_τI})
             = 25.000 + 25.000
             = 50.000 [N·m + A = ABSTRACTION — OC-M0R-6]

B[e_τI]      = A[e_τI] + A[e_IV]         (adj⁺(e_τI) = {e_IV})
             = 25.000 + 17.500
             = 42.500 [A + V = ABSTRACTION — OC-M0R-6]

B[e_IV]      = A[e_IV]                    (adj⁺(e_IV) = {} — terminal)
             = 17.500 V                    [COHERENT — V]
```

**Full B EdgeField preserved per kernel contract. Only B[e_IV] = 17.500 V has a declared dimensional interpretation.**

---

### 3.5 R Status

**R is NOT EVALUATED.**

| Blocking condition | Status |
|-------------------|--------|
| OC-FD-1 (ρ_base) | OPEN — not declared through M |
| OC-FD-2 (χ₀) | OPEN — not declared through M |
| OC-M0R-6 (dimensional resolution) | OPEN — interior B edges incoherent |

**E_current = B EdgeField (M0 proxy for authorized E_current = R(B(A)))**

R status at this step: **OPEN**. No simulated R output. No substitute value.

---

### 3.6 Phase 2 — A_p → B_p → R_p

**A_p (persistence difference):**
```
A_p[c][e] = E_current[c][e] − e_prior[c][e]

A_p[0][e_persist] = 25.000 − A_out_{k-1}   (changes with each step)
A_p[0][e_τI]      = 25.000 − Δ_I_{k-1}
A_p[0][e_IV]      = 17.500 − Δ_V_{k-1}
```

At the disturbance step, A_out_{k-1} was small (near-zero in steady state before disturbance). A_p captures the step-to-step relational change — here a large jump.

**B_p, R_p:** Both OPEN (ρ_base, χ₀ undeclared). Not computed at M0.

---

### 3.7 Present Relational State After Step k

**Sigma update:**
```
sigma_v_new = sigma_v + B[e_IV]
            = 1.787 + 17.500
            = 19.287 V
```

**Command projection (C):**
```
theta_error = THETA_TARGET − theta_k
            = 0.34907 − 0.17453 rad
            = 0.17453 rad (arm still at 10°, target is 20°)

frac        = clamp(theta_error / THETA_TARGET, −1.0, 1.0)
            = clamp(0.17453 / 0.34907, −1.0, 1.0)
            = clamp(0.500, −1.0, 1.0)
            = 0.500

v_cmd       = sigma_v_new + frac × V_RAISE
            = 19.287 + 0.500 × 19.0
            = 19.287 + 9.500
            = 28.787 V
```

**New active state:**

| Variable | New value | Replaced |
|----------|-----------|---------|
| `tau_s_prior` | 27.553 N·m | Was 2.553 N·m |
| `theta` | 0.17453 rad | Unchanged (same step) |
| `sigma_v` | 19.287 V | Was 1.787 V |
| `e_prior` | B EdgeField at step k | Was B EdgeField at step k−1 |

**Released:** tau_s_prior(k−1) = 2.553 N·m, e_prior(k−1), A EdgeField, intermediate B (kept as e_prior), v_cmd = 28.787 V

---

### 3.8 Subsequent Observation (Step k+1)

v_cmd = 28.787 V is applied to the motor at step k. The plant evolves continuously. At step k+1:

- A new τ_s(k+1) is observed by the torque sensor (value depends on plant response to 28.787 V)
- A new θ(k+1) is observed

**M admission check:**
```
|tau_s(k+1)| ≤ 68.57 N·m   → ADMIT (expected, as motor works against disturbance)
```

The step-k+1 computation proceeds from the new active state established above.

**The exact value of τ_s(k+1) is a future observation — not computable from the current state without plant simulation. It will be admitted when measured.**

---

### 3.9 Episodic Record (Step k)

Step k is recorded as an episode because the disturbance produces |A_out| = 25.0 N·m, which is expected to exceed ε_episode (OC-EM-1 pending).

**Episode recorded at step k:**
```
EpisodeRecord {
    step_index:       k,

    // Initial configuration (before disturbance)
    initial_tau_s:    2.553 N·m,
    initial_theta:    0.17453 rad (10°),
    initial_sigma_v:  1.787 V,

    // Observed change
    a_out:            25.000 N·m,
    delta_v:          17.500 V,

    // Command issued
    v_cmd:            28.787 V,

    // Resulting configuration (admitted at step k+1 — OPEN until observed)
    resulting_tau_s:  OPEN — not yet admitted,
    resulting_theta:  OPEN — not yet admitted,

    // Outcome
    outcome:          Disturbance,   // |A_out| = 25.0 > ε_episode (OC-EM-1 pending)

    // References
    kernel_version:   "V8 (2026-09-27)",
    domain_commit:    "abr-robotics-demo@09ea3052",
    declaration_ref:  "m_declaration.md",
}
```

When step k+1 is admitted, `resulting_tau_s` and `resulting_theta` are filled in and the episode is complete.

**Episodic memory after step k:**
```
episode_records: [EpisodeRecord { step_index: k, ... }]
```

This record is OUTSIDE the active relational boundary. It does not enter A, B, or R at any subsequent step.

---

### 3.10 Later Recall

At some future step t > k, the arm encounters a similar disturbance condition.

**Present configuration at step t (hypothetical):**
- tau_s_prior = 2.800 N·m (near-static, similar to initial_tau_s = 2.553)
- tau_s_new = 28.100 N·m (similar disturbance)
- A_out = 25.300 N·m (similar to episode.a_out = 25.000)

**Recall trigger:** |A_out(t)| > ε_recall_trigger (OC-EM-2 pending)

**Comparison (OC-EM-2 pending thresholds):**
```
|A_out(t) − episode.a_out| = |25.300 − 25.000| = 0.300 < ε_A  → RELEVANT
|tau_s_new(t) − episode.initial_tau_s| = |28.100 − 2.553| ≈ 25.5  (comparing present vs episode prior — should compare initial configurations)
|tau_s_prior(t) − episode.initial_tau_s| = |2.800 − 2.553| = 0.247 < ε_τ → RELEVANT
```

**Retrieved information (to C layer only):**
```
episode.a_out    = 25.000 N·m   → confirms similar disturbance magnitude
episode.delta_v  = 17.500 V     → confirms expected Δ_V
episode.v_cmd    = 28.787 V     → prior command in similar situation
episode.outcome  = Disturbance  → confirms disturbance type
```

**Admission to subsequent calculation:**

This information goes to the C projection layer as contextual reference (OC-EM-3 pending). It does NOT:
- Enter tau_s_new or tau_s_prior
- Modify the NodeField
- Enter A, B, or R
- Replace the present σ_v accumulation

The present computation at step t proceeds independently through M → A → B → [R: OPEN] → Sigma → C, with the recalled episode available as a reference in C. The episode confirms that a similar prior situation produced outcome: Disturbance with v_cmd = 28.787 V, informing (but not determining) the present command.

**Episodic status remains: outside the active relational boundary.**

---

## 4. Worked Example — PASS/FAIL/OPEN Summary

| Step | Action | Status |
|------|--------|--------|
| Observation τ_s = 27.553 N·m | Admitted: 27.553 ≤ 68.57 | **PASS** |
| A_out = 25.000 N·m | Exact arithmetic | **PASS** |
| Δ_I = 25.000 A | Domain coupling ÷K_T | **PASS** |
| Δ_V = 17.500 V | Domain coupling ×R_W | **PASS** |
| B[e_IV] = 17.500 V | Terminal edge, coherent | **PASS** |
| B[e_τI] = 42.500 | Dimensional sum A+V | **OPEN** — OC-M0R-6 |
| R evaluation | ρ_base, χ₀ undeclared | **OPEN** — OC-FD-1/2 |
| sigma_v = 1.787 + 17.500 = 19.287 V | Sigma telescoping | **PASS** |
| v_cmd = 28.787 V | C projection | **PASS** |
| Episode recorded | |A_out| = 25.0 (ε_episode OPEN) | **OPEN** — OC-EM-1 |
| Recall comparison | Thresholds ε_A, ε_τ OPEN | **OPEN** — OC-EM-2 |
| Episodic information admitted to A/B/R | Not admitted | **PASS** (constraint respected) |

---

## 5. State Retention Summary

**Minimum variables for one continuous relational step: 4**

```
{ tau_s_prior, theta, sigma_v, e_prior }
```

**Eliminable from active state with declared change:**
- `e_prior`: eliminable if Phase 2 (A_p→B_p→R_p) is withdrawn from the architecture
- `theta`: eliminable only if C projection is redesigned without angular feedback

**Cannot be eliminated without changing the declared architecture:**
- `tau_s_prior`: required for A operator
- `sigma_v`: required for C projection as the integral accumulation of relational change

**Not in active state (episodic memory):**
- `episode_records`: optional, outside active boundary, informing C layer only through declared recall (OC-EM-1/2/3 pending)

---

*Kernel V8 frozen at `abr-irrs` commit `2a58e46`. This document is historical evidence and must not be modified retroactively. Future declarations that resolve OPEN conditions shall reference this document by commit.*
