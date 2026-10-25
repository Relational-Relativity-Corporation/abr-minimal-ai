# Minimal Reciprocal Relational Calculation

**Generator Command:** Generator Correction — Return to the Minimal Relational Primitive  
**Project:** `abr-minimal-ai`  
**Reference:** `abr-minimal-ai` commit `8e9d910`  
**Kernel authority:** Frozen Kernel V8 · `abr-irrs` commit `2a58e46`  
**Preceding specification:** `br_dimensional_resolution.md` (commit 8e9d910)  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Mathematical specification only. No kernel modification. No implementation.

---

## 0. Entry Condition and Governing Principle

This document is an observational entry into an already operating reciprocal relational process.

The ABR operators do not construct the relational environment. They declare the minimum sufficient mathematical structure for participating in that environment — receiving observations of changing relationships, calculating within the admitted situational boundary, and acting back into the same ongoing process.

The sensor-actuator interaction is already running before any declaration is made. The declaration provides an interface to it. This document identifies the minimum structure that interface requires.

---

## 1. What Is Directly Observed

The admitted observable extent (M0) contains three quantities:

| Symbol | Quantity | Unit | Role |
|--------|----------|------|------|
| τ_s | Joint torque (sensor) | N·m | Load-side torque observation |
| θ | Joint angle | rad | Position for C projection |
| v_cmd | Motor voltage command | V | Actuator electrical output |

**Key observation about v_cmd:** v_cmd is simultaneously (a) the output the ABR system produces at each step and (b) an admitted observable — it is in the declared observable extent. The actuator output is always known to the system that produced it.

**Declared domain constants:**
- K_T = 1.0 N·m/A, R_W = 0.7 Ω
- V_BUS = 48.0 V, V_RAISE = 19.0 V
- TAU_ELEC_MAX = V_BUS × K_T / R_W = 48.0 / 0.7 ≈ 68.57 N·m

The motor circuit carries winding resistance drop V = τ_s × R_W/K_T [V] at every moment the loop is running. This is the physical electrical potential the motor circuit holds as a consequence of the admitted torque.

---

## 2. Identifying the Reciprocal Relational Unit

The actuator-load interaction has two poles present in the admitted observations:

- **Load pole:** τ_s — what the load imposes on the actuator, expressed as the electrical potential V = τ_s × R_W/K_T [V] the motor circuit carries as a result
- **Actuator pole:** v_cmd(t−1) — what the actuator commanded at the prior step [V]

Both poles are in the same unit: Volts. Both are admitted through M. The relationship between them is the operational relational primitive: **the gap between what the load shows and what was last commanded**.

```
gap(t) = τ_s(t) × R_W/K_T − v_cmd(t−1)    [V]
```

This gap is directly observable at every step. It requires no intermediate computation, no new operator, and no conversion beyond the physical circuit constant R_W/K_T already used in the NodeField declaration.

The gap has clear relational meaning:
- gap > 0: the load's electrical demand exceeds the prior command — the interaction requires more drive
- gap = 0: the interaction is in relational balance — command and load are matched
- gap < 0: the prior command exceeds the load's current demand — the interaction has been over-driven

This is a reciprocal relationship: the load influences the gap through τ_s; the actuator influences the gap through v_cmd. The gap is the quantity that connects both sides of the interaction. It is the minimum sufficient observable primitive for a complete reciprocal relational calculation.

---

## 3. Why the Prior Declaration (v_τ → v_ref = 0) Was Not Reciprocal

The prior corrected topology (`br_dimensional_resolution.md §2.3`) declared:

```
v_τ : τ_s × R_W/K_T [V]     — sensor side
v_ref : 0.0 [V]              — circuit ground (fixed, non-observable)
Edge e_τV : v_τ → v_ref
```

v_ref = 0 is not in the admitted observable extent. It is a fixed declared constant, not an actuator output. The prior A computation:

```
A[e_τV] = τ_s × R_W/K_T − 0 = τ_s × R_W/K_T    [V]
```

This captures only the sensor side of the interaction. The actuator's prior output (v_cmd) does not appear in the spatial topology. The declared domain enters the already-running process with only one pole of the reciprocal relationship.

**Consequence:** the prior declaration resolves dimensional coherence but does not constitute a reciprocal relational calculation. The zero antisymmetric R contribution follows from a topology that is missing the actuator pole, not from a sufficiency declaration of the full interaction.

**This finding does not require adding a new edge or increasing the structural count.** It requires replacing the fixed reference locus (v_ref = 0) with the admitted actuator observable (v_cmd). The structural count remains identical: 2 loci, 1 edge.

---

## 4. Corrected Minimal Declaration

### 4.1 Spatial Topology

**Loci (2):**

| Locus | Symbol | NodeField value at step t | Unit | Relational role |
|-------|--------|--------------------------|------|-----------------|
| v_τ | Torque locus | τ_s(t) × R_W/K_T | V | Load pole — motor circuit potential at admitted torque |
| v_cmd | Command locus | v_cmd(t−1) | V | Actuator pole — prior electrical output (admitted observable) |

**Edges (1):**

| Edge | Direction | Provenance |
|------|-----------|------------|
| e_τC | v_τ → v_cmd | Spatial |

**Component count:** k = 1  
**Unit of component 0:** Volts [V]  
**comp_pairs:** empty  
**cc:** empty

### 4.2 Topology Declaration

```
nodes = [v_τ, v_cmd]
edges = [(v_τ, v_cmd)]     → e_τC
EdgeProvenance: [Spatial]
comp_pairs = []
cc = []
```

### 4.3 NodeField at Step t

```
f.data[0][v_τ]   = τ_s(t) × R_W / K_T    [V]
f.data[0][v_cmd] = v_cmd(t−1)             [V]
```

**Initialization:** At t = 0 (cold start), no prior actuator output has been produced. The declared initial condition is v_cmd(−1) = 0 [V] — the null command state, representing no prior actuation. This is a domain declaration, not a parameter selection.

---

## 5. Complete Operator Sequence (Phase 1)

### 5.1 Adjacency Structure

```
adj⁺[e_τC] = out[v_cmd] = {}    (v_cmd has no outgoing spatial edges)
adj⁻[e_τC] = inc[v_τ]  = {}    (v_τ has no incoming spatial edges)
```

**relational_isolation = true**: all edges have empty adj⁺ and adj⁻. This follows from the sufficient declaration of a 2-observable, single-directed-edge topology. The two admitted observables stand as the two poles of the interaction; no intermediate chain is declared because none is required by the observed relationship.

### 5.2 A Operator (Phase 1)

From `operators.rs` line 455–478:

```
A.spatial[0][e_τC] = f.data[0][v_τ] − f.data[0][v_cmd]
                   = τ_s(t) × R_W/K_T − v_cmd(t−1)
                   = gap(t)                              [V]
```

A directly computes the relational gap between the two poles of the admitted interaction. Both operands are in Volts. All A outputs: **V** ✓

### 5.3 B Operator (Phase 1)

From `operators.rs` line 480–500:

```
B.spatial[0][e_τC] = A.spatial[0][e_τC] + Σ_{f ∈ adj⁺(e_τC)} A.spatial[0][f]
                   = gap(t) + 0            (adj⁺ empty)
                   = gap(t)                              [V]
```

B accumulates one component, one terminal edge. Homogeneous single unit. All B outputs: **V** ✓

### 5.4 compute_rho (Phase 1)

From `operators.rs` lines 267–348:

```
chi[v_τ]   = max_{e incident to v_τ}   |A.spatial[0][e]| = |gap(t)|    [V]
chi[v_cmd] = max_{e incident to v_cmd} |A.spatial[0][e]| = |gap(t)|    [V]
```

(Single incident edge for each locus; chi is the same at both loci for this topology.)

```
ρ[e_τC] = ρ_base × chi[src(e_τC)] / (χ₀ + chi[src(e_τC)])
         = ρ_base × |gap(t)| / (χ₀ + |gap(t)|)          [dimensionless]
```

Both ρ_base and χ₀ require declaration (see §12). ρ[e_τC] is computed at every step — it is a live non-zero value whenever gap ≠ 0.

### 5.5 R Operator (Phase 1)

From `operators.rs` line 502–546:

```
R.spatial[0][e_τC] = B.spatial[0][e_τC]
                   + ρ[e_τC] × (Σ_{f ∈ adj⁺(e_τC)} B.spatial[0][f]
                               − Σ_{f ∈ adj⁻(e_τC)} B.spatial[0][f])
                   = gap(t)
                   + ρ[e_τC] × (0 − 0)
                   = gap(t)                              [V]
```

The antisymmetric R term is structurally zero because adj⁺ and adj⁻ are both empty — a consequence of the sufficient 2-observable declaration, not of missing structure. ρ[e_τC] is computed and non-zero; it multiplies a sum that is zero by the topology's structure.

All R outputs: **V** ✓

**Dimensional status: A, B, R — all PASS. Complete A → B → R sequence is dimensionally coherent in V.**

---

## 6. Phase 2 (Temporal Persistence)

### 6.1 Operator A Persistence

From `operators.rs` line 568–629:

```
A_p.spatial[0][e_τC] = E_current.spatial[0][e_τC] − E_prior.spatial[0][e_τC]
                      = gap(t) − gap(t−1)
                      = Δ_gap(t)                        [V]
```

where E_current = R output from Phase 1, E_prior = PersistenceState carrying gap(t−1).

Δ_gap decomposes as:

```
Δ_gap(t) = [τ_s(t) × R_W/K_T − v_cmd(t−1)] − [τ_s(t−1) × R_W/K_T − v_cmd(t−2)]
          = Δ_V_τ(t) − Δ_v_cmd(t−1)
```

where Δ_V_τ = change in sensor-side electrical potential and Δ_v_cmd = change in actuator command. Δ_gap captures the evolution of the relational balance — whether the gap is growing (imbalance increasing) or shrinking (system correcting).

### 6.2 B_p, ρ_p, R_p (Phase 2)

With the same isolated topology, Phase 2 follows the same structural path:

```
B_p.spatial[0][e_τC] = A_p.spatial[0][e_τC] = Δ_gap(t)    [V]
R_p.spatial[0][e_τC] = B_p.spatial[0][e_τC] = Δ_gap(t)    [V]
```

### 6.3 PersistenceState Update

```
E_prior(t+1) = PersistenceState::from_prior(E_current)
             = PersistenceState storing gap(t)
```

This makes gap(t) available as E_prior at step t+1, enabling Phase 2 at step t+1 to compute gap(t+1) − gap(t).

### 6.4 Sigma Accumulation — Architectural Decision Required

Phase 1 R output = gap(t). Phase 2 R_p output = Δ_gap(t).

These are structurally different accumulands:
- **Phase 1 accumulation:** sigma(t) = sigma(t−1) + gap(t) — integral of the gap. Sigma grows with every step where gap ≠ 0 and converges when gap → 0.
- **Phase 2 accumulation:** sigma(t) = sigma(t−1) + Δ_gap(t) — integral of the gap derivative, equivalent to sigma(t) = sigma(0) + gap(t) − gap(0). Sigma tracks the current gap offset from its initial value.

**At the disturbance step with prior gap = 0 (steady-state relational balance):** both formulations produce the same sigma increment (gap(t) = Δ_gap(t) when prior gap = 0). The numerical results at the worked disturbance step are therefore identical for both accumulation choices.

**In subsequent steps:** the two approaches diverge. Phase 1 accumulation continues integrating the gap at each step, providing integral control action. Phase 2 accumulation tracks the total gap change, providing proportional-to-gap action.

The original m_declaration declared sigma += delta_V from Phase 1 B output, which, under the prior topology, was equivalent to Phase 1 R = B = A = V_τ (the full torque voltage, not the gap). Under the corrected topology, Phase 1 R = gap and Phase 2 R_p = Δ_gap. Which of these corresponds to the intended relational accumulation is an **architectural decision returned to Origin**.

The worked calculation below uses Phase 1 R = gap for sigma, consistent with the interpretation that sigma should accumulate the current relational imbalance at each step. This produces the same sigma increment at the disturbance step as the prior design.

---

## 7. Relational State and Its Persistence

**Active relational state (minimum required):**

| Variable | Value | Source | Operator using it |
|----------|-------|--------|-------------------|
| E_prior [EdgeField] | gap(t−1) [V] | PersistenceState from prior step | operator_a_persistence at step t+1 |
| sigma_v [V] | Accumulated R outputs | Sigma update rule | C projection at each step |
| theta [rad] | Joint angle | Sensor observation | C projection at each step |

**v_cmd(t−1):** This is the system's own prior actuator output. It is produced by the system and is always available without separate storage. It enters the NodeField at step t as the v_cmd locus value.

**Total active state: 3 variables** — same count as the prior declaration. One locus is replaced (v_ref → v_cmd); no state is added.

**Persistence mechanism:** At the end of each step, PersistenceState is updated:
```
E_prior(t+1) ← PersistenceState::from_prior(gap(t))
```

This carries gap(t) forward so that Phase 2 at step t+1 can compute Δ_gap(t+1) = gap(t+1) − gap(t).

The relational state is not a historical model of the physical system. It is the minimum information required to continue the relational calculation at the next observation.

---

## 8. Where Reciprocity Is Expressed Mathematically

The reciprocal relational structure appears explicitly in three places:

**1. NodeField declaration (the declaration of both poles):**
```
f.data[0][v_τ]   = τ_s(t) × R_W/K_T    [V]    ← load acting on actuator
f.data[0][v_cmd] = v_cmd(t−1)           [V]    ← actuator acting on load
```

Both sides of the interaction are declared as loci. Neither is fixed to zero. Neither is absent from the declared domain.

**2. A operator (the direct measurement of the relational gap):**
```
A[e_τC] = v_τ − v_cmd = τ_s(t) × R_W/K_T − v_cmd(t−1) = gap(t)    [V]
```

A directly computes the relational difference between what the load presents and what the actuator last commanded. This is not a conversion or a modeling step — it is the observation of the relational gap between the two admitted poles.

**3. The closed feedback path (the actuator output entering the next step's locus):**
```
v_cmd(t)   ← C(sigma_v(t), theta(t))          [V]
               ↓
f.data[0][v_cmd] at step t+1 ← v_cmd(t)       [V]
               ↓
A[e_τC] at step t+1 = τ_s(t+1) × R_W/K_T − v_cmd(t)
```

The actuator output enters the running process as the next step's v_cmd locus. The gap computed at step t+1 includes the actuator's t-step response. The loop is formally closed: actuator output feeds sensor observation feeds gap computation feeds sigma feeds actuator output.

**This is the mathematical expression of reciprocity:** the gap at each step is computed from the joint contributions of sensor (τ_s) and actuator (v_cmd). Each influences the gap; each is influenced by the gap's accumulated history (through sigma).

---

## 9. Worked Calculation — Disturbance Step

### 9.1 Prior State (Steady State Before Disturbance)

The system was holding the arm against gravity torque. In relational balance (gap = 0), v_cmd matched the load's electrical demand:

```
tau_s_prior    = 2.553 N·m
v_tau_prior    = 2.553 × 0.7 = 1.787 V
v_cmd_prior    = 1.787 V          (relational balance: gap_prior = 0)
sigma_v_prior  = 1.787 V
theta          = 10° (= π/18 rad)
E_prior        = PersistenceState encoding gap_prior = 0 V
```

*Note: v_cmd_prior = 1.787 V requires frac = 0 in the C projection (arm at reference angle with no active raise component). The exact v_cmd_prior depends on the C projection declaration for the prior step; the present example uses frac = 0 for this step. The structural argument is independent of this specific frac choice.*

### 9.2 Disturbance Event

A 25 N·m load disturbance is applied. The sensor observes:

```
tau_s(t) = 2.553 + 25.000 = 27.553 N·m
theta(t) = 10° (= π/18 rad, arm position unchanged)
```

### 9.3 NodeField Construction

```
f.data[0][v_τ]   = 27.553 × (0.7 / 1.0) = 19.287 V    ← motor circuit potential at admitted torque
f.data[0][v_cmd] = v_cmd(t−1)            =  1.787 V    ← prior actuator output (admitted observable)
```

### 9.4 Phase 1 Operators

**A:**
```
A.spatial[0][e_τC] = 19.287 − 1.787 = 17.500 V = gap(t)
```

The gap is 17.500 V: the load now imposes an electrical demand 17.500 V greater than what was last commanded.

**B:**
```
adj⁺[e_τC] = {}    (no successors)
B.spatial[0][e_τC] = 17.500 V    (terminal)
```

**compute_rho:**
```
chi[v_τ]   = |A.spatial[0][e_τC]| = 17.500 V
chi[v_cmd] = |A.spatial[0][e_τC]| = 17.500 V    (same single incident edge)

ρ[e_τC] = ρ_base × 17.500 / (χ₀ + 17.500)    [dimensionless; requires ρ_base and χ₀ declaration]
```

ρ[e_τC] is a live non-zero quantity. Its numerical value depends on the declaration of ρ_base and χ₀ (see §12).

**R:**
```
adj⁺[e_τC] = {},  adj⁻[e_τC] = {}
R.spatial[0][e_τC] = 17.500 + ρ[e_τC] × (0 − 0) = 17.500 V = gap(t)
```

Phase 1 R = gap = 17.500 V.

### 9.5 Phase 2 (Persistence)

```
E_prior.spatial[0][e_τC] = gap_prior = 0 V    (in relational balance before disturbance)

A_p.spatial[0][e_τC] = gap(t) − gap_prior = 17.500 − 0 = 17.500 V = Δ_gap(t)
B_p.spatial[0][e_τC] = A_p = 17.500 V
R_p.spatial[0][e_τC] = B_p = 17.500 V
```

*At this specific step (prior gap = 0): Δ_gap = gap = 17.500 V. Phase 1 R and Phase 2 R_p are identical.*

### 9.6 Sigma Update and C Projection

Using Phase 1 R = gap for sigma accumulation:

```
sigma_v(t) = sigma_v_prior + R.spatial[0][e_τC]
           = 1.787 + 17.500
           = 19.287 V
```

C projection (frac = 0.500 for θ = 10° = π/18, arm position in raise segment):

```
v_cmd(t) = sigma_v(t) + frac × V_RAISE
         = 19.287 + 0.500 × 19.0
         = 19.287 + 9.500
         = 28.787 V
```

### 9.7 PersistenceState Update

```
E_prior(t+1) ← PersistenceState::from_prior(gap(t) = 17.500 V)
```

This will be used at step t+1 to compute Δ_gap(t+1) = gap(t+1) − 17.500.

### 9.8 Resulting Relational State

```
sigma_v  = 19.287 V    (accumulated relational gap history)
E_prior  = 17.500 V    (gap at step t, for Phase 2 at t+1)
v_cmd(t) = 28.787 V    (actuator output entering the physical loop)
theta    = 10°          (unchanged at this step)
```

---

## 10. Comparison with Subsequent Physical Observation

The actuator output v_cmd(t) = 28.787 V enters the motor drive. The motor receives this command and the physical plant responds. At step t+1, the sensor returns a new τ_s(t+1).

**If the disturbance load persists (τ_s(t+1) = 27.553 N·m):**

```
f.data[0][v_τ]   = 19.287 V    (same load)
f.data[0][v_cmd] = 28.787 V    (prior actuator output from step t)

gap(t+1) = 19.287 − 28.787 = −9.500 V
```

The gap has changed sign. The prior command (28.787 V) now exceeds the load's electrical demand (19.287 V). The relational imbalance has been over-corrected by 9.500 V.

Phase 2:
```
Δ_gap(t+1) = gap(t+1) − gap(t) = −9.500 − 17.500 = −27.000 V
```

With Phase 1 R = gap accumulation:
```
sigma_v(t+1) = sigma_v(t) + gap(t+1) = 19.287 + (−9.500) = 9.787 V
v_cmd(t+1) = 9.787 + frac × V_RAISE
```

The sigma trajectory is converging. The gap has moved from +17.500 V (step t) to −9.500 V (step t+1), crossing zero. The relational process is oscillating around balance. This is consistent with the single-pole integral control characteristic: the system will oscillate with decreasing amplitude before settling.

**Reciprocity in the subsequent observation:** The new gap at t+1 depends jointly on τ_s(t+1) (load side, determined by physics) and v_cmd(t) (actuator side, determined by the ABR calculation). The two poles of the interaction have both contributed to the new relational state. The loop is formally reciprocal at every step.

---

## 11. Parameter Provenance — χ, χ₀, ρ_base

### 11.1 χ (computed, no declaration required)

χ[node] is computed from A-field values by the kernel:

```
chi[i] = max_{c, e incident to i} |A.spatial[c][e]|
```

For the corrected topology at the disturbance step:
```
chi[v_τ]   = |gap(t)| = 17.500 V    [V]
chi[v_cmd] = |gap(t)| = 17.500 V    [V]
```

χ is a function of the observed relational gap. It requires no separate declaration — its value follows directly from the A computation. Unit: Volts (same as A outputs). ✓

### 11.2 χ₀ — CANDIDATE: V_BUS = 48.0 V (returned to Origin for confirmation)

χ₀ is the saturation scale for the ρ formula: as χ → χ₀, ρ → ρ_base/2.

The prior declaration of χ₀ = V_BUS (OC-FD-2, `br_dimensional_resolution.md §4.3`) was based on units matching and position as the only declared voltage constant. The Generator Command states: "Do not declare a parameter resolved merely because its units match an existing domain constant." OC-FD-2 is **reopened**.

**Candidate relational argument for χ₀ = V_BUS:**

χ measures |gap| = |τ_s × R_W/K_T − v_cmd|. The maximum physically admitted value of |gap| is bounded by the domain constants:

- Maximum gap: τ_s = TAU_ELEC_MAX, v_cmd = 0 → gap_max = TAU_ELEC_MAX × R_W/K_T = 68.57 × 0.7 = 48.0 V = V_BUS
- Minimum gap: τ_s = 0, v_cmd = V_BUS → gap_min = 0 − V_BUS = −48.0 V → |gap| = 48.0 V

The maximum magnitude of the admitted relational gap equals V_BUS. A candidate interpretation: χ₀ = V_BUS because it is the saturation reference for ρ at the boundary of the admitted domain — ρ reaches half-maximum when the relational gap equals the maximum the declared domain can admit.

This argument is relational (derived from the admitted domain bounds) rather than merely units-matching. However, the characterization "natural saturation reference at the domain boundary" requires explicit Origin confirmation. Does ρ reaching half-maximum at maximum admitted gap reflect the intended meaning of the saturation parameter?

**Status: OC-FD-2 OPEN — candidate presented, returned to Origin for confirmation.**

### 11.3 ρ_base — OPEN: OC-FD-1

ρ_base is a dimensionless scaling coefficient. No declared domain constant determines it. It controls the absolute magnitude of the antisymmetric R coupling. In the M0 topology (relational_isolation = true), ρ[e_τC] is computed and non-zero but multiplies a sum of zero — the antisymmetric R contribution remains structurally zero regardless of ρ_base's value.

ρ_base is a live variable in the kernel's require_rho_declaration assert. It must be declared finite before evaluation. Its value is an independent Origin decision.

**Status: OC-FD-1 OPEN — no relational derivation possible from M0 constants. Requires Origin declaration.**

---

## 12. Minimality Audit

### 12.1 Changes from Prior Declaration (v_τ → v_ref = 0)

| Element | Prior declaration | Corrected declaration | Observable/mathematical reason |
|---------|-----------------|----------------------|-------------------------------|
| Locus v_ref | 0.0 V (fixed, non-observable) | Removed | v_ref = 0 is not in the admitted observable extent; it is a fixed reference that admits only one pole of the interaction |
| Locus v_cmd | Absent from topology | Added: v_cmd(t−1) [V] (admitted observable) | v_cmd is in M0's declared observable extent; its presence as the actuator pole is required by the reciprocal relationship |
| Edge e_τV / e_τC | v_τ → v_ref | v_τ → v_cmd | The edge now spans both poles of the admitted interaction |
| A output | τ_s × R_W/K_T [V] | gap = τ_s × R_W/K_T − v_cmd(t−1) [V] | A now captures the relational gap between both admitted observables |
| Phase 2 A_p | Δ_V_τ = Δ(τ_s × R_W/K_T) [V] | Δ_gap = Δ_V_τ − Δ_v_cmd [V] | Persistence now tracks changes in the full relational gap, not only the sensor side |
| Active state count | 3 variables | 3 variables | No increase; v_cmd is the system's own prior output, always available |
| Structural count | 2 loci, 1 edge | 2 loci, 1 edge | No increase |
| relational_isolation | true | true | Same isolated topology structure |
| R antisymmetric term | 0 (structural) | 0 (structural) | Same adjacency; zero follows from sufficient 2-observable declaration, not from missing structure |
| OC-M0R-6 status | RESOLVED | Preserved ✓ | Dimensional coherence maintained |
| OC-FD-2 (χ₀) | Declared RESOLVED (challenged) | OPEN — candidate presented | Units-match argument insufficient; relational bound argument returns to Origin |
| OC-FD-1 (ρ_base) | OPEN | OPEN | No change; unchanged |

### 12.2 Minimality Test for Each Admitted Element

| Element | Observable relationship requiring it | Frozen operator using it | Information lost if removed | Obtainable another way? |
|---------|-------------------------------------|-------------------------|---------------------------|------------------------|
| v_τ locus | τ_s × R_W/K_T — the load pole of the interaction | operator_a: src(e_τC) | The load-side electrical demand disappears from the topology | No — τ_s is directly observed; the locus is its formal declaration |
| v_cmd locus | v_cmd(t−1) — the actuator pole of the interaction | operator_a: tgt(e_τC) | The actuator's contribution to the gap disappears; the calculation loses reciprocity | No — v_cmd is an admitted observable; replacing it with a fixed reference (v_ref = 0) removes the actuator pole |
| e_τC edge | The directed relationship from load pole to actuator pole | operator_a, operator_b, operator_r | The relational gap between the two admitted poles cannot be computed | No — the gap is the operational primitive; it requires the edge |
| E_prior (PersistenceState) | The prior gap value | operator_a_persistence | Temporal continuity lost; Phase 2 cannot compute Δ_gap | No — prior gap is not directly observable; it is carried forward by the persistence mechanism |
| sigma_v | Accumulated relational gap history | C projection (via domain declaration) | Integral action lost; v_cmd would reflect only the current step's gap | No — the integration of past gaps is not a current observation |
| theta | Joint angle | C projection frac term | Position-dependent raise component cannot be computed | No — theta is a direct observation not derivable from other admitted quantities |

No element in the corrected declaration fails the minimality test.

### 12.3 Elements Explicitly Not Present

| Absent element | Reason for absence |
|---------------|-------------------|
| v_ref = 0 locus | Not an admitted observable; replaced by v_cmd |
| Intermediate locus (e.g., v_I = 0) | Not required by the observed relationship; would be introduced only to create a non-isolated topology (force R ≠ 0), which the Generator Command prohibits |
| Reverse edge (v_cmd → v_τ) | Not required; reciprocity is expressed through the directed gap, not through bidirectional edges |
| Closed spatial cycle | Not required; the feedback loop is expressed through the actuator output entering the next step's NodeField, not through a topology cycle |
| Additional components (k > 1) | Not required; the entire M0 interaction is in a single voltage domain |
| Persistence edge in spatial topology | Not required and structurally prohibited; temporal difference is handled by the authorized persistence mechanism |

---

## 13. Open Conditions

| ID | Condition | Status | Notes |
|----|-----------|--------|-------|
| OC-FD-1 | ρ_base declaration | OPEN | Independent Origin declaration required; no M0 constant determines it |
| OC-FD-2 | χ₀ declaration | OPEN | Candidate χ₀ = V_BUS with relational bound argument; units-match resolution rejected by Generator Command; returned to Origin |
| OC-ARCH-1 | Sigma accumulation source | OPEN | Phase 1 R (= gap) vs Phase 2 R_p (= Δ_gap) produce identical result at disturbance step (prior gap = 0) but diverge at subsequent steps; architectural decision required from Origin |
| OC-INIT-1 | Cold-start v_cmd initialization | CANDIDATE: v_cmd(−1) = 0 | Declared as null command state; requires Origin confirmation or override |
| OC-M0R-1 | Saturating form derivation for ρ | NOT ADDRESSED | Outside scope of this document |
| OC-M0R-6 | B dimensional coherence | RESOLVED ✓ | Corrected topology preserves resolution from br_dimensional_resolution.md |
| OC-M0R-2 | Chi max-selection over components | RESOLVED ✓ | k = 1; trivial |

---

## 14. Return Statement

The minimal declaration, worked calculation, and minimality audit are complete. The following are returned to Origin and Verifier for decision before further specification or implementation:

1. **OC-ARCH-1:** Confirm which R output sigma should accumulate — Phase 1 (gap integral) or Phase 2 (gap-change integral). The worked calculation uses Phase 1 R = gap.

2. **OC-FD-2:** Confirm or reject χ₀ = V_BUS on the relational bound argument (max admitted |gap| = V_BUS = 48.0 V). Units-match resolution was insufficient; this is the alternative relational argument.

3. **OC-FD-1:** Declare ρ_base. No M0 domain constant determines it. The corrected topology preserves the requirement but cannot resolve it.

**The calculation stops here. No implementation follows this document.**

---

## 15. Source Pinning

| Authority | Repository | Commit |
|-----------|-----------|--------|
| Frozen Kernel V8 | `relational-relativity-corporation/abr-irrs` | `2a58e46` |
| M0 prior corrected declaration | `abr-minimal-ai` | `8e9d910` |
| B/R dimensional resolution | `docs/br_dimensional_resolution.md` | `8e9d910` |
| M Declaration V1.0 | `abr-robotics-demo/docs/m_declaration.md` | (prior session) |
| State retention audit | `docs/state_retention_audit.md` | (prior session) |
