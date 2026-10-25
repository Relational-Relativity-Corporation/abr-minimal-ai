# Continuous Relational Evolution

**Generator Command:** Generator Command — Continuous Relational Evolution  
**Project:** `abr-minimal-ai`  
**Source domain:** `abr-robotics-demo` commit `09ea3052`  
**Mathematical authority:** Frozen Kernel V8 through `abr-irrs` commit `2a58e46`  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Mathematical specification, architectural refinement, and test design only. No M1 implementation. No kernel modification.

---

## Governing Objective

Establish a computational architecture in which **observable relational evolution determines mathematical structure**, while sampling intervals, frame rates, clock cycles, and numerical representation remain implementation mechanisms.

> The physical process determines the relationships. The relationships determine the mathematics. The mathematics determines the computational requirements.
>
> The computer must not determine the mathematics merely through its numerical architecture, clock rate, or sampling schedule.

Continuous relational evolution does not require continuous processor execution. The objective is to preserve relational continuity across computational evaluations.

---

## 1. Separation of Physical Evolution from Computational Evaluation

### 1.1 Four Distinct Layers

Every declaration in the relational domain must be assigned to exactly one of four layers. Conflation of layers produces pseudo-mathematics — quantities that appear to describe physical reality but depend on the computational schedule.

| Layer | What belongs here | What does not belong here |
|-------|-------------------|---------------------------|
| **Physical evolution** | τ_s(t) evolving continuously as the arm moves; I and V in the motor circuit; θ increasing under torque | Sampling interval Δt; frame count; reset boundaries |
| **Observable information admitted through M** | τ_s(t_k), θ(t_k), v_cmd(t_k) at declared observation times | τ_s at intermediate times not admitted; derived plant quantities (I, V) not in observable extent |
| **Mathematical operator dependencies** | A depends on prior and current admitted τ_s; B depends on A output and declared topology; R depends on B output, ρ_base, χ₀ | The schedule at which operators are evaluated; whether Δt is 1/24, 1/100, or 1/1000 |
| **Computational schedule** | Observation times t_k = k · Δt; or event-triggered: admitted when ‖τ_s(t) − τ_s(t_last)‖ > ε | The physical relationships themselves; the operator formulas; the declared topology |

### 1.2 Application to the Robotics Domain

The robotics domain D (from `abr-robotics-demo`, m_declaration commit `09ea3052`) has observable extent {τ_s, θ, v_cmd}. The declared operator chain is:

```
O_t → M → A → B → [R: OPEN] → Sigma → C → u_t
```

Layer assignment for this chain:

**Physical evolution layer:**
- τ_s(t): the arm-mounted torque sensor output, evolving continuously under gravity, disturbance, and motor response
- θ(t): joint angle, evolving under the dynamics I_EFF · d²θ/dt² = τ_motor − τ_gravity − DAMPING · dθ/dt
- Motor I and V: evolve in the motor circuit but are NOT in the observable extent

**Observable information layer:**
- τ_s admitted at observation times through M; admission criterion: |τ_s| ≤ TAU_ELEC_MAX = 68.57 N·m
- θ admitted at the same observation times
- These are the ONLY inputs to the relational computation

**Mathematical operator layer:**
- A: A_out = τ_s(t) − τ_s(t−1) (directed temporal difference at persistence edge)
- B: Δ_I = A_out / K_T; B[e_τI] = Δ_I + Δ_V; B[e_IV] = Δ_V = Δ_I × R_W
- R: OPEN — prerequisites ρ_base, χ₀ not declared through M
- Sigma: σ_v(t) = σ_v(t−1) + Δ_V (declared accumulation)
- C: frac = clamp((θ_target − θ)/θ_target, −1, 1); v_cmd = σ_v + frac × V_RAISE

**Computational schedule layer:**
- FPS = 24 Hz (original demo); alternative rates 100, 500, 1000 Hz considered in §4
- Δt = 1/f: the interval between admitted observations
- This is an implementation parameter. The operator formulas do not contain Δt. The declared topology does not depend on Δt. Sampling rate is not a physical primitive.

### 1.3 Why Frame Boundaries Are Not Physical Boundaries

The physical system evolves continuously. When the computational schedule samples at t_k and t_{k+1}, the physical relationships between those times are not observed. They are not zero. They are not interpolated. They are **unresolved**.

The relational state at t_{k+1} is declared from the admitted observations at t_{k+1} and the prior state at t_k. The interval (t_k, t_{k+1}) is computationally invisible but physically real.

Two consequences follow directly:

1. **No state reset at observation boundaries.** Relational state is carried continuously from observation to observation through the declared persistence mechanism. The kernel's `PersistenceState` is not reconstructed from scratch at each step — it is updated from the prior `EdgeField` output.

2. **Different sampling rates produce the same mathematical operators.** The A, B, and R formulas do not change with Δt. What changes is only: (a) the number of evaluations and (b) the magnitude of A_out per evaluation for smooth trajectories. The topology, operator chain, and declared constants are sampling-rate-invariant.

---

## 2. Relational Persistence

### 2.1 Kernel V8 Persistence Mechanism

The frozen Kernel V8 declares the following persistence structure (source: `irrs-kernel/src/operators.rs`, commit `2a58e46`):

**`PersistenceState`** (§12.5 of kernel_reference.md):
```
PersistenceState { e_prior: EdgeField }
```
- Constructed from the kernel output of a completed prior declared observation: `PersistenceState::from_prior(e_prior: EdgeField)`.
- There is NO cold-start constructor. `EdgeField::zero` and `PersistenceState::cold_start` were removed at Kernel V8.
- The first observation uses `operator_e` (Phase 1 only: A→B→R).
- Second and subsequent observations use `operator_e_v5` (Phase 1 + Phase 2: A_p → B_p → R_p applied to the full EdgeField change).

**`operator_a_persistence`**:
```
A_persistence[c][e] = E_current[c][e] − E_prior[c][e]
```
This is the temporal relational difference operating over the complete EdgeField from one observation to the next. It is not a scalar difference — it is a full edge-level change in the kernel output.

**`operator_b_persistence`** and **`operator_r_persistence`**: apply the B and R forms to A_persistence, using the same adj⁺/adj⁻ structure over the declared spatial topology.

### 2.2 Declared Persistence in the Robotics Domain

The robotics demo (`abr_flex_demo.py`) implements a scalar persistence: `self._tau_s_prior` carries the prior admitted torque value. The temporal A operator is computed as `A = tau_s − self._tau_s_prior`.

This is a **domain-layer simplification** consistent with the operator's semantic intent but not using the full kernel EdgeField persistence API. Specifically:

- The demo carries `τ_s(t−1)` as a scalar, not an `EdgeField`.
- The kernel's A_persistence would operate on the complete EdgeField output (all edge values from the prior evaluation), capturing how all coupled relationships changed between observations.
- For a single-component single-coupling system, the scalar simplification produces the same numerical result as the full kernel persistence for the A step, because the EdgeField has only one meaningful spatial value.

This simplification is currently acceptable at M0. At M1, the full kernel PersistenceState API must be used.

**Critical constraint (verbatim from Generator Command §2):**
> Do not reset relational state at observation boundaries unless a declared physical event requires it.
> Do not substitute independently accumulated state for the authorized persistence operators.

The demo's Sigma accumulation (`self._sigma += delta_V`) is declared in M as "declared accumulation" and is not an independent state substitution. It remains authorized.

### 2.3 Unresolved Intervals

Between admitted observations at t_k and t_{k+1}, the relational state is:
- v_τ₁: τ_s(t_k) — last admitted torque
- v_τ₀: τ_s(t_{k−1}) — prior admitted torque
- All derived quantities: as computed at t_k

The physical system continues to evolve. The relational state is **declared as last-admitted**, not as interpolated, not as zero.

Where the interval cannot be resolved from declared information, it must be marked as **UNRESOLVED: interval (t_k, t_{k+1}) not admitted**. Intermediate states must not be invented.

For event-driven observation schedules (§5), this marking is explicit: when no event fires between t_k and t_{k+1_event}, the state carries the last admitted observation and the unresolved interval spans from the last event to the next.

---

## 3. Sampling Independence — Mathematical Analysis

### 3.1 Physical Trajectory (Fixed)

The reference physical trajectory is the `abr-robotics-demo` simulation with declared parameters:

| Parameter | Value | Layer |
|-----------|-------|-------|
| THETA_TARGET | 20° | OBSERVATION-GROUNDED (declared target) |
| THETA_CROSS | 10° | OBSERVATION-GROUNDED (declared disturbance threshold) |
| DIST_MAG | 25.0 N·m | OBSERVATION-GROUNDED (declared disturbance magnitude) |
| TAU_STATIC | 14.709 N·m | ASSUMED (MASS × G × ARM; plant quantities not in D) |
| TAU_ELEC_MAX | 68.57 N·m | ASSUMED (V_BUS / R_W × K_T; from declared constants) |
| Total duration | 6 s | ABSTRACTION (computational duration, not a physical boundary) |
| Original FPS | 24 Hz | ABSTRACTION (computational schedule) |

The physical plant (not in D) uses: MASS=3.0 kg, ARM=0.5 m, G=9.806 m/s², I_EFF=0.75 kg·m², DAMPING=0.5.

### 3.2 Sampling Rate Behavior of A

For a smooth physical trajectory τ_s(t), the A output per step at rate f is:

```
A_out(t_k) = τ_s(t_k) − τ_s(t_{k−1})
           ≈ (dτ_s/dt)|_{t_k} · Δt    (for small Δt)
```

Near θ = 10° with θ̇ ≈ 0.1 rad/s:
```
dτ_s/dt = MASS × G × ARM × cos(θ) × dθ/dt
        ≈ 14.709 × cos(10°) × 0.1
        ≈ 1.449 N·m/s
```

| Rate | Δt | A_out per step (smooth phase) | Steps for 6 s |
|------|----|-------------------------------|---------------|
| 24 Hz | 1/24 s | ≈ 0.060 N·m | 144 |
| 100 Hz | 0.01 s | ≈ 0.014 N·m | 600 |
| 500 Hz | 0.002 s | ≈ 0.003 N·m | 3000 |
| 1000 Hz | 0.001 s | ≈ 0.001 N·m | 6000 |

Individual A_out magnitudes decrease with rate. The number of evaluations increases proportionally.

### 3.3 Step Disturbance — Rate-Independent Detection

DIST_MAG = 25.0 N·m is applied as a step function when θ ≥ THETA_CROSS. Regardless of sampling rate, exactly one evaluation step will observe the full disturbance transition. At that step:

```
A_out_disturbance ≈ DIST_MAG + gravity_change_in_step ≈ 25.0 N·m  (disturbance dominates)
```

The step is detected in one evaluation at any rate. This is a rate-independent property of step events. The disturbance does not spread across multiple steps.

### 3.4 Telescoping Convergence of Sigma

The accumulated Sigma over the complete trajectory:

```
σ_v(T) = Σ_{k=1}^{T·f} Δ_V(t_k)
        = Σ_{k=1}^{T·f} [τ_s(t_k) − τ_s(t_{k−1})] × R_W/K_T
        = [τ_s(T) − τ_s(0)] × R_W/K_T    (telescoping sum, exact)
```

**Critical result:** For any sampling rate f, the total accumulated Sigma is exactly equal to `(τ_s(T) − τ_s(0)) × R_W/K_T`. The telescoping identity holds regardless of f because A takes differences and Sigma sums them. The total accumulation is **sampling-rate-invariant**.

This is the sampling independence property of the A→Sigma path. It holds in exact arithmetic. Numerical differences across rates arise only from floating-point rounding: O(f · machine_epsilon) error for N = T·f additions, which grows as `O(√f)` under random rounding (IEEE 754).

> **§7 Correction (Generator Command — Relational State, B/R, and Memory Reconciliation):**
> The telescoping convergence proof above applies to **open-loop replay of a fixed physical trajectory**. The physical trajectory τ_s(t) is held constant across all sampling schedules; only the observation timestamps differ. This proof does not establish that closed-loop behavior is sampling-rate-invariant. In a closed-loop system, the v_cmd output of each step feeds back into the plant. At different sampling rates, v_cmd updates occur at different intervals, producing different physical plant responses and consequently a different physical trajectory τ_s(t). Open-loop replay and closed-loop physical response are structurally different: rate-invariance of the former does not imply rate-invariance of the latter.

### 3.5 Intermediate State Divergence

While total accumulation converges, the intermediate relational state at any fixed time T' < T may differ across rates. At 24 Hz, σ_v(T') is a coarse approximation; at 1000 Hz it more closely tracks the continuous integral. The difference at any intermediate time T' is bounded by:

```
|σ_v^{(f)}(T') − σ_v^{(∞)}(T')| ≤ (max |d²τ_s/dt²|) × R_W/K_T × T'²/(2f)
```

This vanishes as f → ∞. For the step disturbance (discontinuous dτ_s/dt), the bound is tighter at the disturbance step and the integral converges from above in one step.

### 3.6 Operator Output Comparison by Rate

| Comparison criterion | 24 Hz | 100 Hz | 500 Hz | 1000 Hz | Event-triggered |
|---------------------|-------|--------|--------|---------|-----------------|
| Admitted observations | 144 | 600 | 3000 | 6000 | N_events ≤ N_clocked |
| A_out per step (smooth) | ~0.060 N·m | ~0.014 N·m | ~0.003 N·m | ~0.001 N·m | ≥ ε per event |
| A_out at disturbance | ~25.0 N·m | ~25.0 N·m | ~25.0 N·m | ~25.0 N·m | ~25.0 N·m (if ε < 25.0) |
| B[e_IV] at disturbance | 17.500 V | 17.500 V | 17.500 V | 17.500 V | 17.500 V |
| σ_v(T) total | = telescoping | = telescoping | = telescoping | = telescoping | < telescoping (missed gradual changes) |
| Topology preserved | Yes | Yes | Yes | Yes | Yes |
| R evaluated | OPEN | OPEN | OPEN | OPEN | OPEN |
| Numerical error | O(1/24) | O(1/100) | O(1/500) | O(1/1000) | O(ε/|dτ_s/dt|) |
| Actuator response | Coarse | Finer | Fine | Finest (clocked) | Depends on event rate |
| Computational cost | 144 evals | 600 evals | 3000 evals | 6000 evals | N_events |

### 3.7 Finding: Mathematical Rate-Independence

**The operator chain A→B→Sigma is sampling-rate-invariant in the total accumulation.** The A→B path is a linear map through declared physical constants (K_T, R_W). The telescoping sum through Sigma is exact. The topology, operator formulas, and declared constants do not change with sampling rate.

Different rates produce different intermediate state paths but converge to the same terminal accumulation for the complete trajectory. This confirms the governing principle: the relational mathematics describes continuous evolution; the sampling schedule is an implementation parameter.

---

## 4. Event-Driven Relational Evaluation

### 4.1 Admissible Change Events

An admissible change event is defined by a change in the admitted observable relation that exceeds a declared threshold:

```
event at t_new when |τ_s(t_new) − τ_s(t_last_eval)| > ε
```

where ε must be:
- **Declared through M** with an explicit observational justification
- **Dimensionally consistent** with τ_s (N·m)
- **Not a numerical tolerance** — DETECTION_TOLERANCE = 1e-12 from `derived_invariants.rs` is an implementation-only numerical guard, not a domain event threshold

A threshold ε must have a physical interpretation. Possible declarations:
- Sensor resolution of the torque sensor (if measurable)
- Declared minimum detectable disturbance
- A fraction of TAU_STATIC = 14.709 N·m

**OPEN (OC-CRE-1):** Event threshold ε not declared through M. No observational basis has been established for the event threshold in this domain.

### 4.2 Affected Relationships

When an event fires at t_new:

1. **τ_s(t_new) → v_τ₁**: admitted through M (verification: |τ_s| ≤ TAU_ELEC_MAX)
2. **A operator evaluated**: A_out = τ_s(t_new) − τ_s(t_last_eval)
   - Note: t_new − t_last_eval ≠ Δt_clocked in general; the interval is the actual elapsed time since the last event, not a fixed clock interval
3. **B operator evaluated**: Δ_I, Δ_V, B[e_τI], B[e_IV] computed
4. **R operator**: OPEN (ρ_base, χ₀ undeclared)
5. **Sigma updated**: σ_v += Δ_V
6. **Persistence updated**: τ_s(t_last_eval) := τ_s(t_new)

### 4.3 Unchanged Relationship Retention

Between events, all of the following are retained:
- τ_s(t_last_eval): carried in persistence state
- σ_v: last computed accumulation
- θ(t_last_eval): last admitted angle
- All edge field values from last evaluation

These are not interpolated, not decayed, not zeroed. The relational state persists from the last evaluation. The unresolved interval (t_last_eval, t_new) is explicitly declared as not-admitted.

**Provenance of unchanged state:** EdgeProvenance::Persistence — relational-evolutionary, connecting E_prior[e] to E_current[e] across one declared process step. The kernel's `PersistenceState::from_prior` is the authorized mechanism.

### 4.4 Missing Gradual Change Under Event-Triggered Evaluation

For smooth trajectory phases where |dτ_s/dt| is small, event-triggered evaluation at threshold ε may admit far fewer observations than a clocked schedule. The total accumulated Sigma will be:

```
σ_v^{event}(T) = Σ_{events k} Δτ_s(t_k→t_{k+1}) × R_W/K_T
```

This is a partial telescoping sum — it misses intervals where Δτ_s < ε between consecutive events. The error is:

```
|σ_v^{event}(T) − σ_v^{continuous}(T)| ≤ (number of missed gradual steps) × ε × R_W/K_T
```

For the step disturbance (25.0 N·m >> ε for any reasonable ε), the event is always detected. For slow gravity-driven torque change, the systematic shortfall is proportional to ε.

**Architectural consequence:** Event-driven evaluation is appropriate when changes of interest are large and abrupt (e.g., disturbances). For slow continuous change, a clocked schedule or a small declared threshold is required to preserve telescoping convergence.

---

## 5. Operator Structure — Corrections and Confirmations

### 5.1 Correction to M0 V2 SF-1 and OC-M0R-3

**M0 V2 finding SF-1** stated:

> SF-1: Single-step topology is acyclic — physical feedback cycle does not close within D at step t

**M0 V2 OC-M0R-3** stated:

> OC-M0R-3: χ[e] for acyclic path topology — definition of antisymmetric circulation on a path graph (no cycles) not established

Both findings were **based on an incorrect premise** corrected directly by Generator Command §5:

> R does not require a closed graph cycle merely because it contains an antisymmetric successor–predecessor term.

**Verification from Kernel V8 source** (`operators.rs`, lines 100–109):

The `adj_plus[e]` and `adj_minus[e]` structures are computed from `out[tgt(e)]` and `inc[src(e)]` respectively — pure graph operations that work on any directed graph. No cycle is required.

**Formal confirmation for the robotics topology:**

Spatial edges: e_τI (v_τ₁ → v_ΔI, index 0) and e_IV (v_ΔI → v_ΔV, index 1).

```
adj_plus[e_τI]  = out[v_ΔI]  = {e_IV}  ← non-empty
adj_minus[e_τI] = inc[v_τ₁]  = {}       ← empty

adj_plus[e_IV]  = out[v_ΔV]  = {}       ← empty (terminal)
adj_minus[e_IV] = inc[v_ΔI]  = {e_τI}  ← non-empty
```

R on the acyclic path:
```
R(B(g))[e_τI] = B(g)[e_τI] + ρ[e_τI] × (B(g)[e_IV] − 0)
              = B(g)[e_τI] + ρ × B(g)[e_IV]

R(B(g))[e_IV] = B(g)[e_IV] + ρ[e_IV] × (0 − B(g)[e_τI])
              = B(g)[e_IV] − ρ × B(g)[e_τI]
```

The antisymmetric structure is **structurally present** on the acyclic path: adj+ provides a positive term and adj− provides a negative term. They are on different edges (e_τI sees adj+ from e_IV; e_IV sees adj− from e_τI). This is exactly the intended antisymmetric coupling.

**Corrected status of OC-M0R-3:** WITHDRAWN. The acyclic topology is topologically adequate for R evaluation. The OPEN condition for R is not topological — it is the absence of ρ_base and χ₀ declarations from M.

**Corrected status of SF-1:** PARTIALLY REVISED. The observation that the single-step topology is acyclic remains accurate. The inference that this prevents R evaluation is incorrect and is retracted.

**What remains OPEN from SF-1:** The cross-step observation from SF-1 is carried forward as OC-M0R-5: the physical feedback cycle (v_ΔV(t) → motor → θ(t+1) → τ_s(t+1) → v_τ₁(t+1)) spans multiple computation steps. This is a domain modeling question — not a prerequisite for R evaluation — and remains open for Origin authorization.

### 5.2 Correction: χ Is a Node-Level Quantity, Not a Per-Edge State

M0 V2 referenced "χ[e]" as a per-edge accumulated state variable. This is incorrect.

From `operators.rs` lines 295–309 (compute_rho for ABR):

```
χ[i] = max |A(x)[c][e]| over all components c and edges e incident to node i

ρ[e] = ρ_base × χ[src(e)] / (χ₀ + χ[src(e)])
```

χ is a **node-level** quantity computed **fresh at each evaluation** from the current A field magnitudes. It is not accumulated across evaluations. It is not a per-edge quantity. There is no "χ[e]" — there is χ[node] at the source of each edge.

Consequence: R has no per-evaluation state beyond what A provides. The state between observations is carried entirely by `PersistenceState` (the prior EdgeField), not by χ. This simplifies the persistence requirement and eliminates the "per-edge circulation accumulation" from the M0 V2 description.

### 5.3 Confirmation: B Must Preserve Its Complete Edge-Field Output

The B operator (`operators.rs` lines 485–500) returns an `EdgeField` containing values for all declared edges:

```
B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]
```

For the robotics topology:
```
B(g)[e_τI] = A_out/K_T + A_out × R_W/K_T = A_out × (1/K_T + R_W/K_T) = A_out × (1+R_W)/K_T

  In the disturbance frame (A_out = 25.0 N·m):
  B(g)[e_τI] = 25.0 × (1 + 0.7)/1.0 = 25.0 × 1.7 = 42.500  [mixed units — see §5.5]

B(g)[e_IV] = A_out × R_W/K_T
           = 25.0 × 0.7/1.0 = 17.500 V
```

SF-2 from M0 V2 correctly identified that B(g)[e_τI] = 42.500 is produced by the kernel but is not consumed by the current Sigma accumulation (which uses only B(g)[e_IV]). This finding is **confirmed and carried forward**.

The Generator Command §5 states: "B must preserve its complete edge-field output." This is satisfied structurally — the `EdgeField` returned by `operator_b` contains all edge values. The application layer must not silently discard B(g)[e_τI].

OC-M0R-6 stands: B full tensor usage — what the domain declares to do with B(g)[e_τI] = 42.500 is an open domain question returned to Origin.

### 5.4 Temporal Persistence Must Use the Kernel's Declared Mechanism

The authorized persistence mechanism is `PersistenceState::from_prior(EdgeField)` followed by `operator_e_v5`. The persistence edge (EdgeProvenance::Persistence) connects E_prior[e] to E_current[e] across one declared process step.

The robotics demo's scalar persistence (`self._tau_s_prior`) is an authorized domain-layer simplification for M0, as the domain has a single observable component. At M1, the full kernel persistence API must be used.

The Sigma accumulation is a **declared** accumulation in M, authorized by the m_declaration. It is not a substitution for persistence operators.

### 5.5 Physical Dimensions Must Not Be Combined Without Authorized Mapping

B(g)[e_τI] = 42.500 has no declared physical unit. It combines:
- A_out × (1/K_T): units of [N·m / (N·m/A)] = A (current change)
- A_out × (R_W/K_T): units of [N·m × Ω / (N·m/A)] = V (voltage change)

Current change [A] and voltage change [V] cannot be added without an authorized dimensional mapping. The fact that K_T has units N·m/A and R_W has units Ω produces a sum of incommensurable dimensions at e_τI. This is the physical interpretation of the dimensional conflict in B(g)[e_τI].

**This does not prevent B evaluation** — B is a mathematical operation on the declared field g, which is dimensionless at the operator level. The physical interpretation of the result at e_τI requires a cross-dimensional mapping that is not currently declared.

OC-M0R-6 carries this physical dimension question as well: what is the physical meaning and authorized use of the intra-topology sum B(g)[e_τI] in dimensional terms?

### 5.6 Revised OPEN Conditions

Following the corrections above, the OPEN conditions are revised:

| ID | Description | Status |
|----|-------------|--------|
| OC-FD-1 | ρ_base — antisymmetric coupling weight, not declared through M | **OPEN** — requires M declaration |
| OC-FD-2 | χ₀ — reference scale for ρ saturation, not declared through M | **OPEN** — requires M declaration |
| OC-M0R-1 | RhoSaturatingForm — functional form of ρ(ρ_base, χ, χ₀) | **OPEN** — requires Origin declaration |
| OC-M0R-2 | RhoMaxSelectionRule — max-selection rule for χ over incident edges | **OPEN** — requires Origin declaration |
| OC-M0R-3 | χ[e] for acyclic path topology | **WITHDRAWN** — acyclic topology is topologically adequate for R; OPEN conditions are OC-FD-1 and OC-FD-2 |
| OC-M0R-5 | Cross-step topology — v_ΔV(t) → v_τ₁(t+1) not declared as a relation | **RETAINED** as domain modeling question only; NOT a prerequisite for R evaluation |
| OC-M0R-6 | B full tensor usage — B(g)[e_τI] = 42.500 not consumed; dimensional mapping undeclared | **RETAINED** — requires domain decision and authorized mapping |
| OC-CRE-1 | Event threshold ε — not declared through M for event-driven schedule | **NEW** — from §4, this specification |

---

## 6. Observable-Basis Declaration Matrix

Every quantity in the declared domain is assigned one of:

- **OBSERVATION-GROUNDED:** Established observational relationship with identified provenance.
- **ASSUMED:** Explicitly declared relationship grounded in established physical behavior, but not independently measured in this experiment.
- **ABSTRACTION — OBSERVATIONAL BASIS OPEN:** No established observational mapping.

### 6.1 Physical Evolution Layer

| Quantity | Status | Justification |
|---------|--------|---------------|
| τ_s(t) [continuous] | OBSERVATION-GROUNDED | Physical torque at torque sensor; admitted through M at observation times |
| θ(t) [continuous] | OBSERVATION-GROUNDED | Physical angle at encoder; admitted through M at observation times |
| Motor I(t) [continuous] | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Not in observable extent; not admitted through M |
| Motor V(t) [continuous] | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Not in observable extent; not admitted through M |

### 6.2 Observable Information Layer (admitted through M)

| Quantity | Status | Justification |
|---------|--------|---------------|
| τ_s(t_k) | OBSERVATION-GROUNDED | Admitted through M with temporal provenance; admission criterion enforced |
| τ_s(t_{k−1}) | OBSERVATION-GROUNDED | Prior admitted observation; carried by PersistenceState |
| θ(t_k) | OBSERVATION-GROUNDED | Admitted through M at same observation times |
| v_cmd(t_k) | OBSERVATION-GROUNDED | Actuator electrical input; declared in observable extent; returned to plant |

### 6.3 Declared Physical Constants (through M)

| Quantity | Value | Status | Justification |
|---------|-------|--------|---------------|
| K_T | 1.0 N·m/A | ASSUMED | Declared motor torque constant; EvidenceDeclared provenance; not independently measured in this experiment |
| R_W | 0.7 Ω | ASSUMED | Declared motor winding resistance; EvidenceDeclared; not independently measured |
| V_BUS | 48.0 V | ASSUMED | Declared supply voltage; EvidenceDeclared |
| V_RAISE | 19.0 V | ASSUMED | Declared voltage raise range parameter; EvidenceDeclared |
| TAU_ELEC_MAX | 68.57 N·m | ASSUMED | Derived from V_BUS/(R_W × K_T); depends on ASSUMED constants |

### 6.4 Mathematical Operator Outputs

| Quantity | Status | Justification |
|---------|--------|---------------|
| A_out = τ_s(t) − τ_s(t−1) | OBSERVATION-GROUNDED | Algebraic difference of two admitted observations; no additional assumptions |
| Δ_I = A_out / K_T | ASSUMED | Depends on declared K_T (ASSUMED); not independently measured current change |
| Δ_V = Δ_I × R_W | ASSUMED | Depends on declared R_W (ASSUMED); not independently measured voltage change |
| B(g)[e_IV] = Δ_V | ASSUMED | Terminal coupling edge; inherits ASSUMED status from Δ_V |
| B(g)[e_τI] = Δ_I + Δ_V | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Dimensional sum of incommensurable units (A + V); no authorized physical mapping declared |
| R(B(g))[e] | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Prerequisites ρ_base (OC-FD-1) and χ₀ (OC-FD-2) not declared through M |

### 6.5 Accumulated and Scheduling Quantities

| Quantity | Status | Justification |
|---------|--------|---------------|
| σ_v(t) | ASSUMED → approaches OBSERVATION-GROUNDED over trajectory | Declared accumulation in M; depends on ASSUMED Δ_V; verification possible at each step if v_cmd sensor is in loop |
| ρ_base | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Not declared through M; no observational mapping established for this domain |
| χ₀ | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Not declared through M |
| χ[i] (computed from A) | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Node-level max of A magnitudes; computable but depends on undeclared ρ parameters |
| ρ[e] (computed from χ[i]) | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Depends on undeclared ρ_base and χ₀ |
| FPS / Δt | ABSTRACTION | Computational scheduling parameter; not a physical primitive |
| Event threshold ε | ABSTRACTION — OBSERVATIONAL BASIS OPEN | Not declared through M (OC-CRE-1) |

### 6.6 Excluded Plant Quantities (not in D)

| Quantity | Value | Status |
|---------|-------|--------|
| MASS | 3.0 kg | ABSTRACTION — excluded from D; simulation plant parameter only |
| ARM | 0.5 m | ABSTRACTION — excluded from D; simulation plant parameter only |
| G | 9.806 m/s² | ABSTRACTION — excluded from D; simulation plant parameter only |
| I_EFF | 0.75 kg·m² | ABSTRACTION — excluded from D; simulation plant parameter only |
| DAMPING | 0.5 | ABSTRACTION — excluded from D; simulation plant parameter only |
| TAU_STATIC | 14.709 N·m | ABSTRACTION — derived from excluded plant quantities; not admitted through M |

---

## 7. Minimal Computational Architecture

### 7.1 Design Criterion

The minimal architecture is the smallest structure capable of:

1. Admitting observable relationships through M
2. Preserving their relational topology (4 loci, 3 edges, 1 interior locus)
3. Evaluating authorized operators (A, B; R deferred pending OC-FD-1/2)
4. Retaining relational evolution between observations (PersistenceState)
5. Responding to newly admitted changes (event or clocked)
6. Comparing calculated results against subsequent observations (verification path)

No neural-network layers, embeddings, artificial time steps, learned weights, or additional operators.

### 7.2 Minimal State

| Variable | Type | Contents | Kernel authority |
|---------|------|----------|-----------------|
| `tau_s_prior` | f64 | τ_s at the last admitted observation | PersistenceState (simplified scalar form for M0) |
| `sigma_v` | f64 | Accumulated B[e_IV] output since initialization | Declared accumulation in M |
| `theta` | f64 | θ at the last admitted observation | Observable extent |
| `e_prior` | EdgeField | Full EdgeField from last A→B→R evaluation | PersistenceState (required for full kernel at M1) |

At M0, `e_prior` is not actively used (R is OPEN), but must be maintained as the authorized persistence mechanism for future activation.

### 7.3 Input (through M)

At each observation time t_k:
- `tau_s_new`: f64 — admitted torque sensor reading
- `theta_new`: f64 — admitted angle reading
- Admission check: |tau_s_new| ≤ TAU_ELEC_MAX = 68.57 N·m

### 7.4 Computation (authorized operator chain)

```
Step 1. Admit:
    τ_s(t_k) through M, verify admission criterion

Step 2. A (authorized):
    A_out = τ_s(t_k) − τ_s_prior

Step 3. B (authorized):
    Δ_I        = A_out / K_T                [ASSUMED]
    Δ_V        = Δ_I × R_W                  [ASSUMED]
    B[e_τI]    = Δ_I + Δ_V                  [ABSTRACTION — dimensional sum unresolved — OC-M0R-6]
    B[e_IV]    = Δ_V                         [ASSUMED]

Step 4. R (NOT EVALUATED — OPEN):
    OPEN: OC-FD-1 (ρ_base), OC-FD-2 (χ₀)
    Note: topology is adequate (§5.1); evaluation blocked by missing M declarations only

Step 5. Sigma (declared accumulation):
    σ_v = σ_v + Δ_V                          [ASSUMED]

Step 6. C (projection, not a kernel operator):
    frac    = clamp((θ_target − θ_new) / θ_target, −1, 1)
    v_cmd   = σ_v + frac × V_RAISE           [ASSUMED]

Step 7. Persist:
    τ_s_prior = τ_s(t_k)
    θ         = θ_new
```

### 7.5 Minimal Output

`v_cmd`: declared actuator electrical input. This is the only output of the computation path that is within the declared observable extent.

### 7.6 Verification Path

At observation t_{k+1}:
- Re-admit τ_s(t_{k+1})
- Compare τ_s(t_{k+1}) against the trajectory prediction from v_cmd(t_k) through the plant
  - Note: the plant is not in D; comparison must be through the admitted sensor reading, not through plant simulation
- Declare any discrepancy as a relational change event admitted through M

The verification path is: **observed outcome at t_{k+1} compared against the relational state predicted at t_k**. This comparison must use admitted observables, not plant-model reconstructions.

---

## 8. PASS / FAIL / OPEN Conditions

### 8.1 PASS — Architecture

| ID | Condition | Status | Basis |
|----|-----------|--------|-------|
| ARCH-1 | Observable extent correctly declared: {τ_s, θ, v_cmd} | PASS | m_declaration |
| ARCH-2 | Plant quantities excluded from D: {MASS, ARM, G, I_EFF, DAMPING} | PASS | m_declaration §1 |
| ARCH-3 | Relational topology preserved: 4 loci, 3 edges, 1 interior locus | PASS | M0 V2 D |
| ARCH-4 | A operator correctly maps persistence edge to temporal difference | PASS | operators.rs |
| ARCH-5 | B operator correctly propagates through coupling topology | PASS | operators.rs; M0 V2 §5 |
| ARCH-6 | Sampling rate does not appear in operator formulas | PASS | §3 analysis |
| ARCH-7 | Total Sigma accumulation is rate-invariant (telescoping) | PASS | §3.4 proof |
| ARCH-8 | R topology confirmed adequate — adj+/adj− non-empty on acyclic path | PASS | §5.1 proof |
| ARCH-9 | B full edge-field output preserved in EdgeField return type | PASS | operators.rs lines 485–500 |
| ARCH-10 | χ correctly identified as node-level, computed fresh per evaluation | PASS | operators.rs lines 295–311 |

### 8.2 FAIL — None

No architectural failures have been identified in this specification. OC-M0R-3 (previously identified as a finding requiring cycle topology for R) is WITHDRAWN following verification against kernel source (§5.1).

### 8.3 OPEN

| ID | Condition | Status |
|----|-----------|--------|
| OC-FD-1 | ρ_base — not declared through M | OPEN — returned to Origin |
| OC-FD-2 | χ₀ — not declared through M | OPEN — returned to Origin |
| OC-M0R-1 | RhoSaturatingForm | OPEN — returned to Origin |
| OC-M0R-2 | RhoMaxSelectionRule | OPEN — returned to Origin |
| OC-M0R-5 | Cross-step physical feedback topology | OPEN — domain modeling question |
| OC-M0R-6 | B full tensor B[e_τI] usage and dimensional mapping | OPEN — returned to Origin |
| OC-CRE-1 | Event threshold ε for event-driven schedule | OPEN — returned to Origin |

Total OPEN conditions: 7 (6 carried, 1 new).

---

## 9. Correction Proposal for M0 V2 Inconsistencies

### Proposal CR-1: Retract OC-M0R-3

**Target finding:** OC-M0R-3 from M0 V2 README:
> OC-M0R-3: χ[e] for acyclic path topology — definition of antisymmetric circulation on a path graph (no cycles) not established

**Correction:** OC-M0R-3 is based on two errors:

1. **Incorrect claim:** that R requires topological cycles. The R operator uses adj⁺(e) and adj⁻(e) which are computed from the graph's adjacency structure and are well-defined on any directed graph, including acyclic paths. Confirmed from Kernel V8 source (`operators.rs` lines 521–529).

2. **Incorrect characterization of χ:** χ is a node-level quantity (χ[i]) computed fresh from A magnitudes at each evaluation, not a per-edge accumulated circulation state. There is no "χ[e]" in the kernel — the variable that appears in the ρ formula is χ[src(e)], the source node's max A magnitude.

**Proposed revision:** Remove OC-M0R-3 from the OPEN conditions table. Replace with a clarifying note in the M0 V2 spec:

> R evaluation on the acyclic two-edge coupling path is topologically unobstructed. adj⁺(e_τI) = {e_IV} and adj⁻(e_IV) = {e_τI} are both non-empty. R remains OPEN due to missing M declarations for ρ_base (OC-FD-1) and χ₀ (OC-FD-2) only.

**This proposal requires Origin review before the M0 V2 HTML specification is updated.**

### Proposal CR-2: Revise SF-1 Language in M0 V2

**Target finding:** SF-1 from m0_spec_v2_robotics.html:
> SF-1: Single-step topology is acyclic; physical feedback cycle does not close within D at step t

**Correction:** The first clause (topology is acyclic) is accurate. The implication that this is an obstacle to R evaluation should be removed. The finding should read:

> SF-1: Single-step topology is acyclic (DAG). The R operator is topologically unobstructed on this acyclic path. The physical feedback cycle (motor output → arm → sensor → next input) spans multiple computation steps; the within-step topology is adequate for R. [OC-M0R-5 carries the cross-step domain modeling question.]

**This proposal requires Origin review before the M0 V2 HTML specification is updated.**

---

## 10. Proposed Minimal Implementation Plan

**AUTHORIZATION REQUIRED.** This plan requires explicit Origin approval before any implementation proceeds. No M1 code has been written. This is a specification only.

### 10.1 Phase M1-A: Full Kernel Integration (prerequisite: OC-FD-1, OC-FD-2)

Upon Origin declaration of ρ_base and χ₀:

1. Replace scalar `tau_s_prior` with `PersistenceState::from_prior(EdgeField)` from Kernel V8 API.
2. Replace demo's custom A computation with `operator_a(NodeField, DeclaredRelations, pairs)`.
3. Evaluate `compute_rho(a_field, rel, rho_base, chi_0)`.
4. Evaluate `operator_r(b_field, rel, rho, cc)`.
5. Connect R output to a declared consumer (replacing or supplementing Sigma).

### 10.2 Phase M1-B: Sampling Independence Test Execution

Upon completion of M1-A:

1. Run the `sampling_independence_test_plan.md` test suite at 100, 500, 1000 Hz and event-triggered.
2. Compare admitted observations, operator outputs, topology preservation, persistence, numerical error, actuator response, computational cost.
3. Verify telescoping convergence and rate-invariance claims from §3.4.

### 10.3 Phase M1-C: Event-Driven Schedule

Upon Origin declaration of event threshold ε (OC-CRE-1):

1. Replace clocked schedule with event-triggered admission.
2. Verify that gradual changes below ε are correctly marked as UNRESOLVED.
3. Verify that step disturbances are detected in one event.

### 10.4 Phase M1-D: Cross-Step Topology (prerequisite: OC-M0R-5)

Upon Origin authorization of cross-step declaration:

1. Extend D to include the cross-step relation: v_ΔV(t) → v_τ₁(t+1).
2. Declare the edge with appropriate provenance.
3. Update the topology and verify interior-locus condition.

---

## Stop Condition

This document completes the specification and test design phase.

No implementation has been produced. The six original OPEN conditions plus one new OPEN condition (OC-CRE-1) are returned to Origin. No Kernel V8 modifications have been made. No M1 implementation has been initiated.

The M0 V2 specification is preserved as historical evidence. The correction proposals in §9 require Origin review before the M0 V2 HTML is updated.

---

## Source Pinning

| Repository | Commit | Purpose |
|-----------|--------|---------|
| `abr-irrs` | `2a58e46` | Kernel V8 operators, DeclaredRelations, PersistenceState |
| `abr-robotics-demo` | `09ea3052` | Source domain: m_declaration, abr_flex_demo.py |
| `abr-minimal-ai` | `7d119e5` | M0 V2 specification (under correction proposal) |

---

*Generator: Claude Sonnet 4.6 — Session https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6*
