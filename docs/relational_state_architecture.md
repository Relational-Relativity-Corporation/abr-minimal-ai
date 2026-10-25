# Relational State Architecture

**Generator Command:** Generator Command — Relational State, B/R, and Memory Reconciliation §4, §7, §7A  
**Project:** `abr-minimal-ai`  
**Source:** Frozen Kernel V8 · `abr-irrs` commit `2a58e46` · `operators.rs`  
**Preceding specification:** `continuous_relational_evolution.md` · `abr-minimal-ai` commit `899fcdc`  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Mathematical specification only. No kernel modification. No implementation.

---

## 1. Purpose

This document establishes:
- The complete state-transition specification for one relational evaluation step
- What information must be retained, calculated, replaced, and released
- The situational relational boundary: which relationships are active at any moment
- The R parameter provenance matrix (§7A)
- Corrections to the sampling-independence specifications distinguishing fixed-trajectory invariance from closed-loop behavior (§7)

---

## 2. State-Transition Specification

### 2.1 Required Prior State

Before a new observation is admitted, the system holds exactly:

| Variable | Type | Content | Provenance |
|----------|------|---------|------------|
| `e_prior` | `PersistenceState { e_prior: EdgeField }` | Full EdgeField from previous completed evaluation (Phase 1 output) | `PersistenceState::from_prior(e_previous)` |
| `tau_s_prior` | `f64` | Previously admitted torque sensor value | Admitted through M; stored for scalar A input |
| `sigma_v` | `f64` | Accumulated Sigma | Running sum of Δ_V = B[e_IV] per step |
| `theta` | `f64` | Most recently admitted joint angle | Admitted through M |

**No cold-start constructor exists.** `PersistenceState::from_prior` is the only constructor (`operators.rs` lines 194–200). The persistence path (Phase 2) is unavailable at the first observation — only Phase 1 (A→B→R) runs.

### 2.2 Admission of a New Observation

New observations must pass M's admission criteria before entering the operator chain:

```
|τ_s_new| ≤ TAU_ELEC_MAX = 68.57 N·m        → ADMIT
|τ_s_new| > TAU_ELEC_MAX                      → REJECT (not admitted)
```

If rejected: the prior state is unchanged. No operator is evaluated.

### 2.3 Present Spatial Calculation (Phase 1: A → B → R)

**Step 1 — Construct NodeField from admitted observations:**

The NodeField must carry values at all declared loci. For the current M0 topology (4 loci):

| Locus | NodeField value | Units | Declared basis |
|-------|----------------|-------|----------------|
| v_τ₀ | tau_s_prior | N·m | Previously admitted observation |
| v_τ₁ | tau_s_new | N·m | Currently admitted observation |
| v_ΔI | tau_s_new / K_T | A | Domain-layer coupling transformation — **OC-M0R-6 applies** |
| v_ΔV | tau_s_new × R_W / K_T | V | Domain-layer coupling transformation — **OC-M0R-6 applies** |

The values at v_ΔI and v_ΔV are derived from tau_s_new via physical coupling constants, not independently observed. Their inclusion in the NodeField is an architectural decision that carries the dimensional inconsistency analyzed in `br_operator_reconciliation.md` §2.

**Step 2 — A operator:**
```
A = operator_a(NodeField, DeclaredRelations, pairs=[])
A.spatial[0][e_persist] = tau_s_prior − tau_s_new    (or reversed by edge direction)
A.spatial[0][e_τI] = NodeField[v_τ₁] − NodeField[v_ΔI]
A.spatial[0][e_IV] = NodeField[v_ΔI] − NodeField[v_ΔV]
A.comp = []  (no declared pairs)
```

Note: A[e_τI] = tau_s_new − tau_s_new/K_T. With K_T=1.0, this is 0.0. The operative A output for domain calculations is A[e_persist] = A_out.

The domain-layer coupling values (Δ_I, Δ_V) are commonly placed directly into the EdgeField rather than derived through operator_a on the 4-locus NodeField, precisely because operator_a on the coupling edges does not produce the physical transformation outputs. Either construction yields the same numerical result when K_T=1.0 but the physical interpretation differs.

**Step 3 — ρ computation:**
```
χ[v_τ₀] = max |A[c][e]| over incident edges = |A[e_persist]| = |A_out|
χ[v_τ₁] = max(|A[e_persist]|, |A[e_τI]|) = |A_out|   (if K_T=1.0, A[e_τI]=0)
χ[v_ΔI]  = max(|A[e_τI]|, |A[e_IV]|)
χ[v_ΔV]  = |A[e_IV]|

ρ[e] = ρ_base × χ[src(e)] / (χ₀ + χ[src(e)])    — OPEN: OC-FD-1, OC-FD-2
```

**Step 4 — B operator:**
```
B = operator_b(A, DeclaredRelations)
B[e_persist] = A[e_persist] + A[e_τI]    (incoherent — OC-M0R-6)
B[e_τI]      = A[e_τI] + A[e_IV]        (incoherent if A[e_τI]=Δ_I, A[e_IV]=Δ_V)
B[e_IV]      = A[e_IV]                  (terminal — coherent if A[e_IV] = Δ_V)
```

**Step 5 — R operator:**
```
R = operator_r(B, DeclaredRelations, rho, cc=[])   — OPEN: OC-FD-1, OC-FD-2, OC-M0R-6
```

R is NOT EVALUATED at M0. The full EdgeField from Phase 1 is B (not R) when ρ is undeclared.

**E_current = B EdgeField** (M0 approximation; authorized E_current = R(B(A(x))) is OPEN)

### 2.4 Temporal Persistence Calculation (Phase 2: A_p → B_p → R_p)

Phase 2 requires `PersistenceState { e_prior }` from the previous step and E_current from Phase 1.

```
A_p = operator_a_persistence(e_current, persistence_state.e_prior)
A_p[c][e] = E_current[c][e] − e_prior[c][e]     (operators.rs lines 573–587)
```

**Dependency:** Phase 2 requires E_current from a completed Phase 1. Since Phase 1's authorized output (R) is OPEN, Phase 2 operates on the B-output proxy for E_current. This is noted but does not block Phase 2 numerically.

```
B_p = operator_b_persistence(A_p, DeclaredRelations)
ρ_p = compute_rho_persistence(A_p, DeclaredRelations, ρ_base, χ₀)   — OPEN: OC-FD-1, OC-FD-2
R_p = operator_r_persistence(B_p, DeclaredRelations, ρ_p)             — OPEN: OC-FD-1, OC-FD-2
```

**At M0:** Phase 2 is structurally available but ρ_base/χ₀ are undeclared. Phase 2 numerical outputs are OPEN.

### 2.5 Present State After Phase 1

After completing Phase 1 (and optionally Phase 2), the new present state is:

**Sigma update:**
```
delta_V = B[e_IV]   (= A_out × R_W / K_T = 17.500 V in disturbance frame)
sigma_v_new = sigma_v + delta_V
```

**Command projection:**
```
theta_error = THETA_TARGET − theta_new
frac        = clamp(theta_error / THETA_TARGET, −1.0, 1.0)
v_cmd       = sigma_v_new + frac × V_RAISE
```

where V_RAISE = 19.0 V (declared electrical constant).

**New PersistenceState:**
```
persistence_state_new = PersistenceState::from_prior(E_current)
```

### 2.6 Retained, Calculated, Replaced, Released

| Information | Action | Lifetime |
|-------------|--------|---------|
| `tau_s_prior` → replaced by `tau_s_new` | REPLACED | Single step: released after A computation |
| `sigma_v` → updated to `sigma_v_new` | UPDATED | Persistent accumulator — retained for the trajectory |
| `e_prior` (previous EdgeField) → replaced by `E_current` | REPLACED | Single step: released once new PersistenceState constructed |
| `theta_new` → becomes `theta` | RETAINED | Retained for C projection; available at next step |
| `v_cmd` | CALCULATED and RELEASED | Computed and applied; not retained beyond step |
| A, B, ρ intermediate outputs | CALCULATED and RELEASED | Not retained after Phase 1 completes |

**Single prior EdgeField is sufficient** for the kernel's first-order persistence mechanism (`operator_e_v5` takes exactly one `PersistenceState { e_prior: EdgeField }`). A second prior EdgeField would implement second-order persistence — not declared in Kernel V8.

### 2.7 State Transition Diagram

```
Prior state:
  { tau_s_prior, theta, sigma_v, persistence_state { e_prior } }
         |
         | New observation admitted through M:
         |   tau_s_new (|tau_s_new| ≤ TAU_ELEC_MAX)
         |   theta_new
         ↓
Phase 1: A(NodeField) → B(A) → [R(B): OPEN]
  E_current = B(A)  (M0 approximation)
         |
Phase 2: A_p(E_current, e_prior) → B_p → [R_p: OPEN]
  (if first step: Phase 2 unavailable — no e_prior)
         |
Sigma: sigma_v_new = sigma_v + B[e_IV]
C: v_cmd = sigma_v_new + clamp(error/THETA_TARGET, −1,1) × V_RAISE
         |
         ↓
New state:
  { tau_s_new, theta_new, sigma_v_new, PersistenceState::from_prior(E_current) }
  Released: { tau_s_prior, e_prior, A, B, ρ, v_cmd }
```

---

## 3. Situational Relational Boundary (§7A)

### 3.1 Definition

The **situational relational boundary** is the set of declared relationships that are active in the current evaluation. It is not fixed — observations enter and leave as the system evolves.

### 3.2 What Is Inside the Boundary

At any step t, the active relational boundary contains:

| Relationship | Boundary status | Entry condition | Exit condition |
|-------------|----------------|-----------------|----------------|
| τ_s(t) — current admitted observation | INSIDE | Admitted through M | Replaced when τ_s(t+1) admitted |
| τ_s(t−1) — prior admitted observation | INSIDE | Retained as tau_s_prior for A | Released after A computation at step t+1 |
| θ(t) — current angle | INSIDE | Admitted through M | Replaced when θ(t+1) admitted |
| σ_v — accumulated Sigma | INSIDE | Running accumulator | Not released; persistent for trajectory lifetime |
| e_prior — EdgeField from step t−1 | INSIDE | Stored in PersistenceState | Released when PersistenceState updated |
| e_τI, e_IV, e_persist — declared edges | INSIDE | Fixed topology declaration | Not released; part of DeclaredRelations |
| K_T, R_W, V_RAISE — declared constants | INSIDE | M declaration | Not released; reference information |

### 3.3 What Is Outside the Boundary

| Information | Boundary status | Reason |
|-------------|----------------|--------|
| τ_s(t−2) and earlier | OUTSIDE | Released after two steps; available only in episodic memory (if recorded) |
| Plant dynamics (MASS, ARM, G, I_EFF, DAMPING) | OUTSIDE | Not in observable extent; not admitted through M |
| Motor current I (instantaneous) | OUTSIDE | Not in observable extent |
| Motor voltage V (instantaneous) | OUTSIDE | Not in observable extent |
| Future states | OUTSIDE | Not observations; remain projections |
| Episodic records | OUTSIDE active boundary | Retained in episodic memory; not admitted into A/B/R |

### 3.4 How Episodes Inform the Boundary Without Entering It

A recalled episode carries information about a past relational configuration and outcome. It can inform the **C projection** (command generation) as a contextual adjustment — for example, by confirming that a similar prior configuration required a specific v_cmd. But this information enters through the C layer, not through A, B, or R. The episode's τ_s value is never admitted as a current observation; the episode's outcome label never modifies the EdgeField.

The formal mechanism for this (a declared episodic relevance function and recall-to-C mapping) is OPEN — see `episodic_memory_design.md` §5.

### 3.5 How New Relationships Enter the Boundary

New relationships enter the boundary only through:
1. **M admission** — a new τ_s or θ value that passes the admission criterion
2. **M declaration** — an authorized new topology, coupling constant, or operator parameter

A new relationship cannot enter the boundary through:
- Inference from prior observations (unless explicitly declared as a derived quantity)
- Episodic memory (past observations are not re-admitted as current)
- Temporal extrapolation (future values are not observations)

---

## 4. R Parameter Provenance (§7A)

The following matrix states the observational and mathematical provenance required for each R parameter.

| Parameter | Mathematical function in R | Provenance type required | Source within declared domain | Authorized? |
|-----------|---------------------------|-------------------------|------------------------------|-------------|
| ρ_base | Saturation ceiling — sets maximum antisymmetric correction | M declaration with physical justification | No existing declared quantity provides this | **OPEN — OC-FD-1** |
| χ₀ | Characteristic scale for ρ saturation — sets the torque-change magnitude at which ρ = ρ_base/2 | M declaration with physical justification | Candidate: DIST_MAG = 25.0 N·m (declared disturbance magnitude) | **OPEN — OC-FD-2** |
| RhoSaturatingForm | ρ = ρ_base·χ/(χ₀+χ) | Kernel V8 source + Origin confirmation | `operators.rs` line 309 — stated but derivation not closed | **OPEN — OC-M0R-1** |
| RhoMaxSelectionRule | χ[i] = max over incident edges and components | Kernel V8 source + Origin confirmation | `operators.rs` lines 337–340 — stated but confirmation not closed | **OPEN — OC-M0R-2** |
| adj⁺, adj⁻ (topology) | Defines successor/predecessor sets for antisymmetric term | Kernel V8 source — established | `operators.rs` lines 100–109 | **ESTABLISHED — PASS** |
| pairs = [], cc = [] (cross-topology) | Cross-domain coupling coefficients | M declaration | Empty declaration — valid at M0 | **DECLARED — VALID** |

**Active state inventory required before R evaluation:**

| Required state | Currently available? |
|---------------|----------------------|
| EdgeField E_current (B output at M0) | YES |
| Declared topology (DeclaredRelations) | YES |
| ρ_base | NO — OC-FD-1 |
| χ₀ | NO — OC-FD-2 |
| OC-M0R-6 dimensional resolution | NO |
| pairs=[], cc=[] | YES |

---

## 5. Sampling Independence Corrections (§7)

This section corrects `continuous_relational_evolution.md` §3 and `sampling_independence_test_plan.md` §1–4. The corrections do not invalidate the telescoping convergence proof; they add required distinctions.

### 5.1 Six Concepts That Must Be Distinguished

**1. Fixed-trajectory mathematical invariance**

The claim established in `continuous_relational_evolution.md` §3.4:

```
σ_v(T) = [τ_s(T) − τ_s(0)] × R_W/K_T   (exact telescoping in exact arithmetic)
```

This is a property of the A→Sigma operator chain applied to a **fixed physical trajectory** observed at any sampling rate. The total accumulated Sigma over the trajectory is invariant to the observation schedule because A→Sigma telescopes to an exact two-point difference.

**This invariance holds only when the physical trajectory is fixed and independent of the sampling schedule.** It is a mathematical property of the operator, not a claim about physical behavior.

**2. Event-triggered telescoping accumulation**

When the observation schedule is event-triggered (admit when |τ_s(t) − τ_s(t_last)| > ε), the telescope still applies to the admitted observations. The accumulated Sigma over admitted events converges to the same endpoint value as clocked observation. However: gradual changes below ε are not admitted. The total Sigma may undercount the physical change if the physical trajectory contains sub-threshold variation. This is not a failure of the mathematics — it is a declared property of the admission criterion ε.

This is distinct from fixed-trajectory invariance. Fixed-trajectory invariance assumes all points on the trajectory are observed; event-triggered accumulation selects a subset.

**3. Sampling and event-detection latency**

In a real implementation, each observation involves:
- A physical measurement with some latency
- An M-admission check
- A computational evaluation

At higher sampling rates, observations are more frequent. Latency at each observation step accumulates differently. Mathematical rate-invariance (point 1) does not imply zero latency at any rate.

For event-triggered systems: the detection of |τ_s(t) − τ_s(t_last)| > ε requires a comparison at each candidate observation time. If the underlying sampling of τ_s is coarse, an event may be detected late. Mathematical rate-invariance does not guarantee zero detection latency.

**4. Open-loop replay**

The `sampling_independence_test_plan.md` tests a **fixed physical trajectory replayed at different sampling rates**. The trajectory is computed once (from the demo physics) and then the operator chain is applied to it at different rates. This is open-loop: v_cmd feeds back into neither the plant simulation nor the trajectory.

**Open-loop replay correctly tests mathematical rate-invariance (point 1).** It correctly verifies that A→Sigma telescopes. It does NOT test closed-loop behavior.

**5. Closed-loop physical response**

In actual operation, v_cmd is applied to the motor, which changes the motor current and torque, which changes θ(t+1) and τ_s(t+1). The physical trajectory at step t+1 depends on v_cmd at step t.

At different sampling rates (different Δt), the same v_cmd produces a different physical response because the plant dynamics evolve continuously. A higher sampling rate with the same v_cmd control policy produces a different physical trajectory from a lower sampling rate.

**A fixed-trajectory open-loop replay cannot establish closed-loop equivalence across sampling rates.** The sampling rate affects:
- How frequently v_cmd is updated
- The physical plant state at each update
- The accumulated error between the control objective and the plant state

**This equivalence is not claimed** — but `continuous_relational_evolution.md` and `sampling_independence_test_plan.md` must state explicitly that their invariance claims apply to open-loop replay only.

**6. Spatial topology versus temporal persistence**

The spatial topology (coupling path v_τ₁ → v_ΔI → v_ΔV) represents present-state physical transformations — it maps admitted τ_s to its electrical equivalents. This topology is evaluated at every step and does not accumulate state across steps.

Temporal persistence (A_p→B_p→R_p in Phase 2) explicitly computes step-to-step relational change. It requires the prior EdgeField and produces a measure of how the relational configuration has evolved.

These are structurally different and must not be conflated:
- Spatial topology: present physical transformation — no cross-step memory
- Temporal persistence: directed relational change — requires prior state

### 5.2 Corrections to Existing Specifications

The following additions are required to `continuous_relational_evolution.md` and `sampling_independence_test_plan.md`:

| Location | Required correction |
|----------|-------------------|
| `continuous_relational_evolution.md` §3.4 | Add: "The telescoping convergence proof applies to open-loop replay of a fixed physical trajectory. It does not establish that closed-loop behavior is sampling-rate-invariant. At different sampling rates, v_cmd updates occur at different intervals, producing different physical plant responses." |
| `sampling_independence_test_plan.md` §1 | Add: "This test plan evaluates open-loop trajectory replay. The physical trajectory is fixed and independent of the sampling schedule. Closed-loop equivalence across sampling rates is not claimed and requires a separate test design." |
| `sampling_independence_test_plan.md` §4 | Add OC-SIT-1: "Event-triggered telescoping accumulation establishes that admitted events telescope correctly, provided all sub-threshold variation in the physical trajectory is declared acceptable to omit. The relationship between ε and the physical accuracy of the accumulated Sigma is OPEN — return to Origin." |

These corrections are applied in a subsequent commit to those files.

---

## 6. Summary of Active State

The minimal active state for one relational evaluation step:

| Variable | Required for | Retained after step? |
|----------|-------------|---------------------|
| `tau_s_prior` | A operator (scalar persistence difference) | YES → becomes prior at next step |
| `sigma_v` | Sigma accumulation, C projection | YES → persistent |
| `theta` | C projection (theta_error) | YES → updated |
| `e_prior` in PersistenceState | Phase 2 (A_p) | YES → replaced by E_current |

**4 variables.** This is the minimal state for continuous relational evolution under the declared M0 architecture.

No other state is required for mathematical correctness. The episodic record (if any) is additional storage outside the active relational computation.

---

*Kernel V8 frozen at `abr-irrs` commit `2a58e46`. This document is historical evidence and must not be modified retroactively. Subsequent Origin declarations that resolve OPEN conditions shall reference this document by commit.*
