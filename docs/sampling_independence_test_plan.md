# Sampling Independence Test Plan

**Generator Command:** Generator Command — Continuous Relational Evolution §3  
**Project:** `abr-minimal-ai`  
**Source domain:** `abr-robotics-demo` commit `09ea3052`  
**Mathematical authority:** Frozen Kernel V8 through `abr-irrs` commit `2a58e46`  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Test design only. No implementation. No kernel modification.

---

## 1. Objective

Determine whether the relational operator chain A→B→Sigma produces consistent outputs when the same physical trajectory is observed at different sampling rates. Identify which differences arise from observation frequency, numerical approximation, or the mathematics itself.

The test holds the underlying physical process and declared parameters constant. The computational schedule is the variable.

**Governing principle from `continuous_relational_evolution.md` §3.4:**

The A→Sigma path exhibits **exact telescoping convergence**: the total accumulated Sigma over a trajectory is sampling-rate-invariant in exact arithmetic. This is the primary mathematical prediction to verify. Deviations from this prediction indicate numerical error only, not a sampling-rate dependence of the mathematics.

---

## 2. Physical Trajectory (Fixed)

The test trajectory is the canonical robotics-demo arm scenario. All physical parameters are held constant across all observation schedules.

### 2.1 Declared Simulation Parameters

| Parameter | Value | Observable-basis status |
|-----------|-------|------------------------|
| THETA_TARGET | 20° = 0.34907 rad | OBSERVATION-GROUNDED |
| THETA_CROSS | 10° = 0.17453 rad | OBSERVATION-GROUNDED |
| DIST_MAG | 25.0 N·m | OBSERVATION-GROUNDED |
| TAU_ELEC_MAX | 68.57 N·m | ASSUMED |
| K_T | 1.0 N·m/A | ASSUMED |
| R_W | 0.7 Ω | ASSUMED |
| V_BUS | 48.0 V | ASSUMED |
| V_RAISE | 19.0 V | ASSUMED |

### 2.2 Plant Parameters (NOT in D — simulation only)

| Parameter | Value |
|-----------|-------|
| MASS | 3.0 kg |
| ARM | 0.5 m |
| G | 9.806 m/s² |
| I_EFF | 0.75 kg·m² |
| DAMPING | 0.5 |
| TAU_STATIC | 14.709 N·m (derived) |

### 2.3 Trajectory Phases

The trajectory has three distinct phases relevant to the relational analysis:

| Phase | Time range | Physical event | A_out character |
|-------|-----------|----------------|-----------------|
| 1 — Pre-disturbance | t ∈ [0, t_cross) | Arm moves from 0° toward 10°; gravity torque increases | Small, smooth, increasing |
| 2 — Disturbance step | t ≈ t_cross | θ crosses 10°; τ_s steps up by 25.0 N·m | Single large step |
| 3 — Post-disturbance | t ∈ (t_cross, 6 s] | Arm continues toward 20° under combined gravity + disturbance | Modified trajectory |

The disturbance is a **step function** — it applies its full magnitude at the single observation step where θ ≥ THETA_CROSS. This property is critical for the step-detection test.

### 2.4 Initial Conditions

| Quantity | Value |
|---------|-------|
| θ(0) | 0.0 rad |
| τ_s(0) | MASS × G × ARM × sin(0) = 0.0 N·m |
| σ_v(0) | 0.0 V |
| τ_s_prior(0) | 0.0 N·m |

Initial conditions are identical across all observation schedules.

---

## 3. Observation Schedules

Four observation schedules are tested against the same physical trajectory. The underlying simulation must be run at a sufficiently fine resolution (≥ 10,000 Hz or with continuous-time integration) to provide τ_s(t) and θ(t) at arbitrary observation times. The test then samples from this high-resolution reference.

### 3.1 Clocked Schedules

| Schedule | Rate | Δt | Total observations |
|----------|------|----|--------------------|
| S-24 | 24 Hz | 1/24 s ≈ 0.04167 s | 144 |
| S-100 | 100 Hz | 0.01 s | 600 |
| S-500 | 500 Hz | 0.002 s | 3000 |
| S-1000 | 1000 Hz | 0.001 s | 6000 |

All clocked schedules observe at t_k = k · Δt for k = 0, 1, 2, … , T/Δt.

The operator evaluation at each step follows the identical declared operator chain:
```
Admit τ_s(t_k), θ(t_k) through M
A_out = τ_s(t_k) − τ_s(t_{k−1})
Δ_I = A_out / K_T
Δ_V = Δ_I × R_W
σ_v = σ_v + Δ_V
```

### 3.2 Event-Triggered Schedule

**S-EVT**: Admit an observation when |τ_s(t) − τ_s(t_last)| > ε.

Event threshold ε is a **test parameter** (not yet declared through M — OC-CRE-1). Two test values are used:

| Schedule | ε | Notes |
|----------|---|-------|
| S-EVT-0.1 | 0.1 N·m | Fine threshold; admits most gradual changes |
| S-EVT-1.0 | 1.0 N·m | Coarse threshold; misses small gradual changes |

For S-EVT, the time between events is variable. The operator formulas remain identical. Between events, the relational state is the last admitted state (not interpolated, not zeroed).

**Note:** ε must be declared through M with physical justification before S-EVT can be authorized as a production schedule. These test runs establish the mathematical behavior; the threshold declaration is returned to Origin as OC-CRE-1.

---

## 4. Comparison Criteria

The following seven comparison criteria are drawn directly from Generator Command §3.

### 4.1 Criterion C1: Admitted Relational Observations

**What to measure:** Number of admitted observations; distribution across trajectory phases.

**Expected behavior:**
- S-24: 144 total; ~1 disturbance-step observation
- S-100: 600 total; ~1 disturbance-step observation  
- S-500: 3000 total; ~1 disturbance-step observation
- S-1000: 6000 total; ~1 disturbance-step observation
- S-EVT-0.1: N_EVT < N_clocked; all disturbance-step observations captured (25.0 N·m >> 0.1 N·m)
- S-EVT-1.0: N_EVT << N_clocked; disturbance-step captured; some gradual steps missed

**Test assertion:** For all clocked schedules, exactly one observation corresponds to the disturbance step (single τ_s step change ≥ 25.0 N·m). For S-EVT, the disturbance step is captured as a single event at both thresholds.

**PASS condition:** Disturbance detected in exactly one step at all clocked rates and both event thresholds.  
**FAIL condition:** Disturbance missed, or detected in more than one step.

### 4.2 Criterion C2: Operator Outputs

**What to measure:** Per-step operator outputs at the disturbance observation; total accumulated Sigma; B full tensor preservation.

**At the disturbance step (disturbance frame):**

| Operator output | Expected value (all schedules) |
|----------------|-------------------------------|
| A_out | ≈ 25.0 N·m (disturbance dominates gravity change in step) |
| Δ_I | ≈ 25.0 N·m / 1.0 N·m/A = 25.0 A |
| Δ_V | ≈ 25.0 A × 0.7 Ω = 17.500 V |
| B[e_IV] | ≈ 17.500 V |
| B[e_τI] | ≈ 42.500 (mixed units — OC-M0R-6) |

The approximation "≈ 25.0 N·m" holds because the gravity-driven change within a single step at any rate is O(Δt) ≪ 25.0 N·m.

More precisely: gravity change = dτ_s/dt|_{θ=10°} · Δt ≈ 1.449 × Δt N·m, which is 0.060 N·m at 24 Hz and 0.001 N·m at 1000 Hz.

**Corrected disturbance-step A_out by schedule:**

| Schedule | Gravity change in step | A_out at disturbance | Δ_V |
|----------|----------------------|---------------------|-----|
| S-24 | 0.060 N·m | 25.060 N·m | 17.542 V |
| S-100 | 0.014 N·m | 25.014 N·m | 17.510 V |
| S-500 | 0.003 N·m | 25.003 N·m | 17.502 V |
| S-1000 | 0.001 N·m | 25.001 N·m | 17.501 V |
| S-EVT-0.1 | variable | ~25.0 N·m | ~17.500 V |
| S-EVT-1.0 | variable | ~25.0 N·m | ~17.500 V |

**PASS condition:** |A_out_disturbance − 25.0| < gravity_change_in_step for each schedule. Disturbance contributes 99.9% or more of A_out at all schedules ≥ 100 Hz.  
**FAIL condition:** A_out at disturbance ≠ DIST_MAG ± gravity_change_in_step.

**Total accumulated Sigma:**

By the telescoping convergence theorem (§3.4 of `continuous_relational_evolution.md`):

```
σ_v^{(f)}(T) = [τ_s(T) − τ_s(0)] × R_W/K_T   (exact, any rate)
```

τ_s(0) = 0.0 N·m; τ_s(T) = τ_s at θ=20° + disturbance = MASS × G × ARM × sin(20°) + 25.0 N·m ≈ 5.027 + 25.0 = 30.027 N·m.

Expected total: σ_v(T) ≈ 30.027 × 0.7/1.0 ≈ 21.019 V (approximately, depending on final θ).

**PASS condition:** |σ_v^{(f1)}(T) − σ_v^{(f2)}(T)| < 10 × machine_epsilon × T × max_rate for any two clocked schedules f1, f2.  
**FAIL condition:** More than floating-point rounding differences between total accumulated Sigma across clocked schedules.

### 4.3 Criterion C3: Preservation of Relational Topology

**What to measure:** That the declared topology (4 loci, 3 edges, 1 interior locus) is unchanged across all observation schedules and steps.

**Expected behavior:** The topology is invariant. It is a declaration, not a computed quantity.

| Topology property | All schedules |
|------------------|--------------|
| Loci count | 4 |
| Edge count | 3 |
| Interior loci | 1 (v_ΔI) |
| Edge provenances | Persistence, Coupling, Coupling |
| adj⁺[e_τI] | {e_IV} |
| adj⁻[e_IV] | {e_τI} |

**PASS condition:** Topology unchanged at all observation steps, all schedules.  
**FAIL condition:** Any topology modification triggered by observation frequency or step count.

### 4.4 Criterion C4: Persistence Across Observations

**What to measure:** That relational state is carried from observation to observation without reset; no artificial discontinuities at observation boundaries.

**Test cases:**

1. **State continuity at the disturbance step:** σ_v immediately before and after the disturbance step differs by exactly Δ_V at that step. No reset to zero.

2. **State continuity across quiet phases (S-EVT):** When no event fires for N consecutive physical time steps, σ_v and τ_s_prior remain at their last admitted values. Test by inserting a forced quiet interval (no τ_s change > ε) and verifying no state drift.

3. **Cross-rate state equivalence at fixed observation times:** At t = 3.0 s (mid-trajectory), compare σ_v across all clocked schedules. Differences should be within numerical error (O(machine_epsilon × T × f)).

**PASS condition:**
- No state reset unless declared physical event (not present in this trajectory)
- σ_v at disturbance step = σ_v before + Δ_V at that step
- State persists without change during S-EVT quiet intervals
- Cross-rate σ_v at fixed observation times within numerical tolerance

**FAIL condition:**
- Any unexplained state discontinuity
- Zero-initialization at observation boundaries
- State change between events in S-EVT

### 4.5 Criterion C5: Numerical Error

**What to measure:** Floating-point accumulation error in σ_v as a function of observation rate.

**Expected error model:**

For N = T · f additions of values of order ε_step ≈ (τ_s_max − τ_s_min) · R_W/K_T / N:

```
σ_v accumulated error ≈ O(√N) × machine_epsilon × |σ_v|
                       = O(√(T·f)) × machine_epsilon × |σ_v|
```

Under random rounding (IEEE 754 double precision, machine_epsilon ≈ 2.2e-16):

| Schedule | N | Expected σ_v error |
|----------|---|--------------------|
| S-24 | 144 | ~2.6e-15 |
| S-100 | 600 | ~5.4e-15 |
| S-500 | 3000 | ~1.2e-14 |
| S-1000 | 6000 | ~1.7e-14 |

All errors are well below DETECTION_TOLERANCE = 1e-12 (from `derived_invariants.rs`).

**PASS condition:** |σ_v^{(S-24)}(T) − σ_v^{(S-1000)}(T)| < DETECTION_TOLERANCE.  
**FAIL condition:** Difference exceeds DETECTION_TOLERANCE, indicating a non-numerical source of divergence.

### 4.6 Criterion C6: Actuator Response

**What to measure:** v_cmd at the disturbance step and at the trajectory endpoint across all schedules.

**v_cmd formula (C projection, not a kernel operator):**
```
frac    = clamp((θ_target − θ) / θ_target, −1, 1)
v_cmd   = σ_v + frac × V_RAISE
```

The v_cmd value depends on:
- σ_v: sampling-rate-invariant at the total level (§3.4)
- θ: admitted observation, sampling-rate-invariant (same physical value at common observation times)
- frac: determined by θ; identical for schedules that observe at the same physical time

**Disturbance-step v_cmd comparison:**

At the step where the disturbance fires (t ≈ t_cross), different schedules may observe slightly different θ values because the physical arm position at t_k depends on the exact time t_k. For clocked schedules, t_cross occurs between clock ticks and the detection is at the first tick after θ ≥ THETA_CROSS.

Expected: σ_v at disturbance step will be nearly equal across rates (dominated by DIST_MAG × R_W/K_T = 17.500 V). Pre-disturbance σ_v will be rate-dependent (faster rates accumulate more gravity steps but telescope to the same total).

**PASS condition:** v_cmd at trajectory endpoint (t = T) within numerical tolerance across all clocked schedules.  
**OPEN condition:** Quantitative comparison requires trajectory simulation. Reference result (S-24) is established from `abr_flex_demo.py` (24 Hz). Test compares S-100, S-500, S-1000 against S-24 endpoint v_cmd.

### 4.7 Criterion C7: Computational Cost

**What to measure:** Number of operator evaluations and total arithmetic operations per trajectory.

| Schedule | Operator evaluations | A evaluations | B evaluations | Sigma updates |
|----------|---------------------|---------------|---------------|---------------|
| S-24 | 144 | 144 | 144 | 144 |
| S-100 | 600 | 600 | 600 | 600 |
| S-500 | 3000 | 3000 | 3000 | 3000 |
| S-1000 | 6000 | 6000 | 6000 | 6000 |
| S-EVT-0.1 | N_events ≤ N_ref | N_events | N_events | N_events |
| S-EVT-1.0 | N_events << N_ref | N_events | N_events | N_events |

**Cost per evaluation (current authorized chain, R OPEN):**
- A: 1 subtraction
- B: 2 multiplications + 2 additions
- Sigma: 1 addition

Total: ~6 arithmetic operations per evaluation.

At 1000 Hz for 6 s: 6000 × 6 = 36,000 arithmetic operations.

**Event-driven cost reduction:** For S-EVT-1.0 with ε = 1.0 N·m and smooth gravity-driven torque change of ≈ 1.449 N·m/s, an event fires approximately every 1.0/1.449 ≈ 0.69 s during the smooth phase. Over 6 s: ~9 gravity events + 1 disturbance event = ~10 evaluations (plus additional events during active control response). Compare to 144 at 24 Hz.

**Trade-off:** Event-driven evaluation reduces computational cost but introduces systematic Sigma shortfall proportional to ε (§4.4 of `continuous_relational_evolution.md`).

**PASS condition for computational cost analysis:** Document N_events for both S-EVT schedules; confirm N_events < N_{S-24} at both thresholds; confirm disturbance event captured (N_events ≥ 1 at disturbance phase).

---

## 5. Test Protocol

### 5.1 Test Environment

**Source repository:** `abr-robotics-demo` commit `09ea3052`.  
**Reference implementation:** `abr_flex_demo.py` (24 Hz clocked, existing).  
**Required new implementations:** Modified versions of `abr_flex_demo.py` with configurable `FPS` parameter and an event-triggered admission scheduler.

**Note:** The implementations described here require Origin authorization under the M1 implementation plan (`continuous_relational_evolution.md` §10). This test plan specifies the design; execution requires authorization.

### 5.2 Reference Trajectory Generation

1. Run the plant simulation at ≥ 10,000 Hz (or with continuous-time Runge-Kutta integration) to generate a high-resolution reference trajectory τ_s(t), θ(t).
2. Store the reference trajectory as a fixed data file (committed to the repository) so all test schedules draw from identical physics.
3. This eliminates any sampling-rate-induced differences in the physical trajectory itself.

### 5.3 Clocked Schedule Test Procedure

For each rate f ∈ {24, 100, 500, 1000} Hz:

```
Initialize: tau_s_prior = 0.0, sigma_v = 0.0, theta = 0.0
For k = 0, 1, ..., floor(T × f):
    t_k = k / f
    tau_s_k = sample_reference(t_k)       # from high-resolution reference
    theta_k = sample_reference_theta(t_k)
    
    # Admission check (M)
    assert |tau_s_k| <= TAU_ELEC_MAX
    
    # A operator
    A_out = tau_s_k - tau_s_prior
    
    # B operator
    delta_I = A_out / K_T
    delta_V = delta_I * R_W
    B_eIV   = delta_V
    B_etauI = delta_I + delta_V
    
    # Sigma (declared accumulation)
    sigma_v += delta_V
    
    # C (not a kernel operator)
    frac  = clamp((THETA_TARGET - theta_k) / THETA_TARGET, -1.0, 1.0)
    v_cmd = sigma_v + frac * V_RAISE
    
    # Persist
    tau_s_prior = tau_s_k
    
    # Record
    record(k, t_k, tau_s_k, A_out, B_eIV, B_etauI, sigma_v, v_cmd)

Record final sigma_v as sigma_v_total(f)
```

### 5.4 Event-Triggered Schedule Test Procedure

For each threshold ε ∈ {0.1, 1.0} N·m:

```
Initialize: tau_s_prior = 0.0, sigma_v = 0.0, theta = 0.0
t_last_eval = 0.0

For t in high-resolution timeline (≥ 10,000 Hz):
    tau_s_t = sample_reference(t)
    
    # Event detection
    if |tau_s_t - tau_s_prior| > epsilon:
        
        # Admission check (M)
        assert |tau_s_t| <= TAU_ELEC_MAX
        
        # A operator
        A_out = tau_s_t - tau_s_prior
        
        # B, Sigma, C (identical to clocked)
        delta_I = A_out / K_T
        delta_V = delta_I * R_W
        B_eIV   = delta_V
        B_etauI = delta_I + delta_V
        sigma_v += delta_V
        theta_t  = sample_reference_theta(t)
        frac     = clamp((THETA_TARGET - theta_t) / THETA_TARGET, -1.0, 1.0)
        v_cmd    = sigma_v + frac * V_RAISE
        
        # Mark interval as resolved
        mark_resolved(t_last_eval, t)
        
        # Persist
        tau_s_prior = tau_s_t
        t_last_eval = t
        
        record(event_count, t, tau_s_t, A_out, B_eIV, B_etauI, sigma_v, v_cmd)
    
    else:
        # Mark interval as unresolved (not admitted)
        mark_unresolved(t_last_eval, t)

Record final sigma_v as sigma_v_total_evt(epsilon)
```

### 5.5 Comparison Metrics to Compute

For each pair of schedules S1, S2:

1. `delta_sigma_total = |sigma_v_total(S1) - sigma_v_total(S2)|` — telescoping convergence test
2. `A_out_disturbance(S)` — disturbance detection test
3. `N_observations(S)` — admitted observation count
4. `N_topology_changes(S)` — expected: 0 for all S
5. `sigma_v(t=3.0, S)` — intermediate state comparison at common time t=3.0 s
6. `v_cmd(t=T, S)` — terminal actuator response
7. `N_evals(S)` — computational cost

---

## 6. Expected Results Summary

| Criterion | S-24 | S-100 | S-500 | S-1000 | S-EVT-0.1 | S-EVT-1.0 |
|-----------|------|-------|-------|--------|-----------|-----------|
| C1: Observations | 144 | 600 | 3000 | 6000 | ~80 | ~15 |
| C2: Disturbance A_out | 25.060 | 25.014 | 25.003 | 25.001 | ~25.0 | ~25.0 |
| C2: Total σ_v | = baseline | = baseline | = baseline | = baseline | < baseline | << baseline |
| C3: Topology preserved | Yes | Yes | Yes | Yes | Yes | Yes |
| C4: No state reset | Yes | Yes | Yes | Yes | Yes | Yes |
| C5: Numerical error | ~3e-15 | ~5e-15 | ~1e-14 | ~2e-14 | N/A | N/A |
| C6: v_cmd(T) | baseline | ≈ baseline | ≈ baseline | ≈ baseline | < baseline | << baseline |
| C7: Evaluations | 144 | 600 | 3000 | 6000 | ~80 | ~15 |

---

## 7. PASS / FAIL / OPEN Conditions

### 7.1 PASS Conditions (expected)

| ID | Assertion | Criterion |
|----|-----------|-----------|
| ST-PASS-1 | Disturbance detected in 1 step at all clocked rates | C1 |
| ST-PASS-2 | |A_out_disturbance − 25.0| < gravity_step for each rate | C2 |
| ST-PASS-3 | |σ_v_total(S-24) − σ_v_total(S-1000)| < DETECTION_TOLERANCE | C2, C5 |
| ST-PASS-4 | Topology unchanged (4 loci, 3 edges) at all steps, all schedules | C3 |
| ST-PASS-5 | No relational state reset at any observation boundary | C4 |
| ST-PASS-6 | Floating-point error < DETECTION_TOLERANCE across all clocked schedules | C5 |
| ST-PASS-7 | N_evals(S-EVT) < N_evals(S-24) for both event thresholds | C7 |
| ST-PASS-8 | Disturbance event captured at both ε thresholds | C1, C2 |

### 7.2 OPEN Conditions

| ID | Condition | Notes |
|----|-----------|-------|
| ST-OPEN-1 | Quantitative v_cmd trajectory comparison requires simulation execution | §6 expected results are analytical only; numerical values require running M1-A |
| ST-OPEN-2 | Event threshold ε — OC-CRE-1 — not declared through M | S-EVT results are exploratory; ε declaration required for authorization |
| ST-OPEN-3 | R operator outputs across schedules — requires OC-FD-1, OC-FD-2 resolution | R comparison cannot be completed until ρ_base and χ₀ declared |
| ST-OPEN-4 | B[e_τI] comparison across schedules — OC-M0R-6 — dimensional mapping undeclared | Physical interpretation of 42.500-unit value blocked |

### 7.3 FAIL Conditions (would invalidate the architecture)

| ID | Assertion | Implication if fails |
|----|-----------|---------------------|
| ST-FAIL-1 | σ_v_total differs by more than numerical error across clocked schedules | Telescoping property violated — operator chain is not sampling-rate-invariant |
| ST-FAIL-2 | Topology modified by any schedule | Declared topology is not invariant — architectural error |
| ST-FAIL-3 | Relational state reset at observation boundaries | PersistenceState mechanism not functioning as declared |
| ST-FAIL-4 | Disturbance not detected at any clocked schedule | Admission criterion or A operator not functioning |

---

## 8. Authorization Requirements

This test plan is a **design document only**. Before any test can be executed:

1. **M1-A authorization** from Origin: permission to create modified versions of `abr_flex_demo.py` with configurable rate.
2. **OC-FD-1 and OC-FD-2 resolution** (optional, for R comparison): ρ_base and χ₀ declaration.
3. **OC-CRE-1 resolution** (required for S-EVT authorization): event threshold ε declaration through M.

---

## Source Pinning

| Repository | Commit | Purpose |
|-----------|--------|---------|
| `abr-irrs` | `2a58e46` | Kernel V8: operators, DeclaredRelations, adj+/adj−, PersistenceState |
| `abr-robotics-demo` | `09ea3052` | Source domain, `abr_flex_demo.py`, m_declaration |
| `abr-minimal-ai` | `7d119e5` | M0 V2 specification (under correction proposal) |

---

*Generator: Claude Sonnet 4.6 — Session https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6*
