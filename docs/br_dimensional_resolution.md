# B/R Dimensional Resolution

**Generator Command:** Generator Command — Complete B/R Dimensional Resolution  
**Project:** `abr-minimal-ai`  
**Reference:** `abr-minimal-ai` commit `ad20fd2`  
**Kernel authority:** Frozen Kernel V8 · `abr-irrs` commit `2a58e46`  
**Preceding specification:** `br_operator_reconciliation.md`, `relational_state_architecture.md`  
**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6  
**Date:** 2026-10-25  
**Authorization:** Mathematical specification only. No kernel modification. No implementation.

---

## 0. Governing Principle: The Sensor-Actuator Loop Is Already in Motion

The ABR operators do not model a physical system from outside. They declare and formalize the relational structure of information that is already in motion in the sensor-actuator loop.

The physical loop for M0:

```
τ_s(t) [torque sensor] → [ABR operator chain] → v_cmd(t) [motor voltage command]
         ↑                                                        |
         └────────────────── [motor physics] ────────────────────┘
                          τ = K_T × I = K_T × v_cmd / R_W
```

The motor carries winding current I = τ/K_T. The resistance drop across the winding is V = I × R_W = τ × R_W/K_T. This voltage exists in the circuit as a physical potential at every moment the motor is running. M admits τ_s; the NodeField declares the electrical potential the motor circuit already holds as a consequence of the admitted torque. The operators then read, accumulate, and formalize the relational state of a loop that is already executing.

v_cmd is the voltage the ABR system outputs into the motor drive — a physical electrical quantity that immediately enters the running circuit and produces a new torque. The sensor reads the torque consequence of v_cmd. The loop was in motion before the operators were declared; the operators give its relational structure a formal home.

**Consequence for this specification:** any process the operators compute is a process that is physically occurring in the running system. ρ[e_τV] is computed at every step because the antisymmetric relational tension between the admitted torque state and the reference is physically present at every step — not because a future topology extension will make it relevant. Δ_V is the actual voltage change that occurred in the circuit between steps. sigma_v is the accumulated electrical record of real circuit events.

---

## 1. Root Cause: OC-M0R-6

### 1.1 What B Actually Does

From `operators.rs` lines 480–500:

```rust
B(g)[c][e] = g.spatial[c][e] + Σ_{f ∈ adj⁺(e)} g.spatial[c][f]
```

B accumulates the A-output of an edge with the A-outputs of all edges leaving its target. It is a homogeneous summation within each component `c`. It performs no scaling, no division, no domain transformation. It does not read the declared `EdgeProvenance` (Spatial, Coupling, Persistence) at runtime.

**The consequence:** every term B adds within component `c` must carry the same physical dimension. B has no mechanism for ÷K_T or ×R_W. Those operations cannot occur inside B by any declaration.

### 1.2 What the M0 V2 Declaration Asserted

M0 V2 (`m_declaration.md`, topology edges):

```
Edge e_persist : v_τ₀ → v_τ₁   (Persistence)
Edge e_τI      : v_τ₁ → v_ΔI   (Coupling / ÷K_T)
Edge e_IV      : v_ΔI → v_ΔV   (Coupling / ×R_W)
```

NodeField at the disturbance step (τ_new = 27.553 N·m, τ_prior = 2.553 N·m):

| Locus | Value | Unit |
|-------|-------|------|
| v_τ₀ | 27.553 | N·m |
| v_τ₁ | 2.553 | N·m |
| v_ΔI | 25.000 | A |
| v_ΔV | 17.500 | V |

A computes differences across edges. The NodeField carries four different physical dimensions across four loci. B accumulates A-outputs in each component. The results (`br_operator_reconciliation.md §3`):

| Edge | A output | adj⁺ | B output | Dimensional status |
|------|----------|------|----------|--------------------|
| e_persist | 25.000 N·m | {e_τI} | 25.0 N·m + 25.0 A = 50.0 [N·m + A] | **FAIL** |
| e_τI | 25.000 A | {e_IV} | 25.0 A + 17.5 V = 42.5 [A + V] | **FAIL** |
| e_IV | 17.500 V | {} | 17.500 V | PASS (terminal only) |

**Root cause of OC-M0R-6:** The M0 V2 topology conflates two structurally distinct aspects of the running loop:

1. **Temporal persistence** — the change in circuit state between steps. The motor circuit carries a different voltage potential at t than at t−1. This temporal difference belongs in the kernel's authorized persistence mechanism (PersistenceState, operator_a_persistence), not in a spatial topology edge.

2. **Physical circuit state** — the voltage the motor circuit carries at the present moment as a consequence of the admitted torque. This is V = τ_s × R_W/K_T, a physical potential in the running circuit. It belongs in the NodeField declaration, where M declares the admitted electrical state before the operators run. It is not a domain transformation the operators perform; it is the circuit state they receive.

The M0 V2 declaration places both aspects in the spatial topology as edges, treating the physical circuit state as a computation B should perform via ÷K_T and ×R_W. The kernel's B operator performs no such operations. B accumulates and the incoherent combinations of N·m, A, and V values follow.

---

## 2. Corrected Topology

### 2.1 Remove Temporal Persistence from the Spatial Topology

Temporal persistence — the comparison of τ_s(t) to τ_s(t−1) — belongs in the kernel's authorized persistence mechanism:

```
PersistenceState::from_prior(e_current)  →  operator_a_persistence
```

`operator_a_persistence` computes `E_current[e] − E_prior[e]` on the Phase 1 EdgeField output. This is the correct structural location for temporal differences.

**e_persist is removed from the spatial topology.** No temporal edge is introduced to replace it.

### 2.2 Physical Circuit State in the NodeField

The motor circuit carries winding current I = τ_s/K_T and the resistance drop V = I × R_W = τ_s × R_W/K_T as physical electrical quantities at every moment the loop is running. With K_T = 1.0 N·m/A and R_W = 0.7 Ω:

```
V = τ_s [N·m] × R_W [Ω] / K_T [N·m/A] = τ_s × 0.7    [V]
```

M admits τ_s. The NodeField declares the voltage the motor circuit already carries as a consequence of the admitted torque. This is not a conversion the operators perform — it is the physical electrical state of the running loop, admitted through M at each step.

The factor R_W/K_T uses both declared motor constants (K_T the torque-to-current ratio, R_W the winding resistance). The intermediate current I = τ_s/K_T is present in the circuit but is not represented as a separate graph node because:

1. I is not in the observable extent (m_declaration §2).
2. A separate intermediate locus carrying I would require a node value that either duplicates the voltage value (making it redundant) or introduces temporal information (prohibited for the spatial topology).

The single NodeField entry f.data[0][v_τ] = τ_s × R_W/K_T declares the complete electrical state of the torque locus. The operators receive this electrical state and operate on it.

### 2.3 Corrected Spatial Topology

**Loci (2):**

| Locus | Symbol | NodeField value | Unit | Role |
|-------|--------|-----------------|------|------|
| v_τ | Torque locus | τ_s × R_W/K_T | V | Electrical potential the motor circuit carries at the admitted torque |
| v_ref | Electrical reference | 0.0 | V | Circuit ground — declared zero-voltage reference |

**Edges (1):**

| Edge | Direction | Provenance |
|------|-----------|------------|
| e_τV | v_τ → v_ref | Spatial |

**Component count:** k = 1  
**Unit of component 0:** Volts [V]  
**comp_pairs:** empty (no cross-topology coupling declared)  
**cc:** empty

**Topology declaration:**

```
nodes = [v_τ, v_ref]
edges = [(v_τ, v_ref)]     → e_τV
EdgeProvenance: [Spatial]
comp_pairs = []
cc = []
```

**NodeField at step t:**

```
f.data[0][v_τ]   = τ_s(t) × R_W / K_T     [V]
f.data[0][v_ref] = 0.0                     [V]
```

The reference locus carries the circuit ground — the actual zero-voltage reference of the motor drive circuit. It is a fixed declared value, not an observation. v_cmd is measured with respect to this reference.

---

## 3. Dimensional Coherence of the Corrected Topology

### 3.1 A Operator (Phase 1)

```
A.spatial[0][e_τV] = f.data[0][v_τ] − f.data[0][v_ref]
                   = τ_s × R_W/K_T − 0
                   = τ_s × R_W/K_T       [V]
```

Single component, single edge. All A outputs: **V** ✓

### 3.2 B Operator (Phase 1)

`adj⁺[e_τV]` = out[v_ref] = {} (empty — v_ref has no outgoing edges)

```
B.spatial[0][e_τV] = A.spatial[0][e_τV] + 0
                   = τ_s × R_W/K_T       [V]
```

B accumulates one component, one edge. No incoherent sum is possible. All B outputs: **V** ✓

**Dimensional status of B:** PASS (terminal edge; homogeneous single component in V)

### 3.3 Adjacency Structure

```
adj⁺[e_τV] = out[v_ref] = {}    (empty)
adj⁻[e_τV] = inc[v_τ]  = {}    (empty)
```

`relational_isolation(rel)` = **true**: every edge has empty adj⁺ and adj⁻. This is a declared structural property of the 1-edge topology. The R antisymmetric term is structurally zero; not a missing declaration.

### 3.4 R Operator (Phase 1)

From `operators.rs` line 529:

```
R.spatial[0][e_τV] = B.spatial[0][e_τV]
                   + ρ[e_τV] × (Σ_succ − Σ_pred)
                   + cross_topology_term
```

With empty adj⁺/adj⁻ and empty comp_pairs:

```
Σ_succ = 0
Σ_pred = 0
cross_topology_term = 0

R.spatial[0][e_τV] = B.spatial[0][e_τV]
                   = τ_s × R_W/K_T       [V]
```

All R outputs: **V** ✓

`ρ[e_τV]` is computed and non-zero (see §5), but its coefficient (Σ_succ − Σ_pred) is structurally zero for the 1-edge topology. The computation of ρ still requires a declared ρ_base (see §6.1).

**Dimensional status of R:** PASS. R is evaluated; its antisymmetric contribution is zero by topology structure.

### 3.5 E_current (Phase 1 Output Stored in PersistenceState)

```
E_current.spatial[0][e_τV] = R.spatial[0][e_τV] = τ_s(t) × R_W/K_T    [V]
```

This is the Phase 1 relational state — the electrical potential the motor circuit carries at the present admitted torque. It is stored in PersistenceState so Phase 2 at the next step can read the change in circuit state between steps.

---

## 4. compute_rho: Dimensional Analysis

### 4.1 Kernel Implementation

From `operators.rs` lines 323–349 (`compute_rho`):

```rust
for e in 0..n_edges {
    let src = rel.src(e);
    let tgt = rel.tgt(e);
    for c in 0..a_field.k {
        let v = a_field.spatial[c][e].abs();
        if v > chi[src] { chi[src] = v; }
        if v > chi[tgt] { chi[tgt] = v; }
    }
}
rho[e] = rho_base * chi[src] / (chi_0 + chi[src]);
```

`chi[i]` takes the maximum of `|A.spatial[c][e]|` over **all components c** and all edges incident to node i.

**Prior problem (M0 V2):** With k > 1 and components in N·m, A, and V, this maximum compared incommensurable physical magnitudes. The max-selection across incompatible units produced a dimensionally incoherent chi.

### 4.2 Normalization via NodeField Declaration

**Corrected topology:** k = 1, component 0 in V.

With k = 1, the max-over-components loop iterates exactly once (c = 0). There is no multi-component comparison. Chi is computed from a single-component A-field, all values in V:

```
chi[v_τ]   = |A.spatial[0][e_τV]| = τ_s × R_W/K_T    [V]
chi[v_ref] = |A.spatial[0][e_τV]| = τ_s × R_W/K_T    [V]
```

**Chi is dimensionally homogeneous [V].** The coherence is achieved by declaring the NodeField in the physical dimension the motor circuit already carries (V), using the actual motor characteristic R_W/K_T, before the kernel receives it. No kernel modification is required.

### 4.3 χ₀ Candidate: V_BUS = 48.0 V

Chi is in volts. The declared saturation parameter χ₀ must carry the same unit as chi (since ρ = ρ_base × chi/(χ₀ + chi) requires chi and χ₀ to be commensurate):

```
χ₀ must be declared in [V]
```

The declared domain has one voltage ceiling: **V_BUS = 48.0 V** (declared drive specification, m_declaration §3).

Physical justification: V_BUS is the maximum electrical potential the drive circuit can supply. As chi → V_BUS, the motor is approaching the declared electrical ceiling — the point where the circuit can no longer increase the voltage it carries. V_BUS provides the natural saturation reference: at chi = V_BUS, ρ = ρ_base/2 (half-maximum coupling weight). The saturation is a property of the running circuit, not a mathematical convenience.

Cross-check: TAU_ELEC_MAX × R_W/K_T = 68.57 × 0.7 = 48.0 V = V_BUS ✓

The declared electrical maximum torque, expressed in the motor circuit's voltage unit, equals V_BUS exactly. The same circuit ceiling that bounds admissible torques also bounds chi. This is not a coincidence — chi measures the same electrical state the admission rule guards.

**OC-FD-2 resolution:** χ₀ = V_BUS = 48.0 V [V] — follows from declared domain constants.

### 4.4 ρ_base Units

```
ρ[e] = ρ_base × chi[V] / (χ₀[V] + chi[V])
```

The ratio chi/(χ₀ + chi) is dimensionless. For ρ to multiply the antisymmetric term (which is in V) and add to B (also in V), ρ must be dimensionless. Therefore **ρ_base is dimensionless**. This follows from the unit analysis and requires no Origin declaration for its units.

The numerical value of ρ_base remains a genuinely independent domain choice (§6).

### 4.5 OC-M0R-2 Resolution

OC-M0R-2 was: "the max-selection rule over multiple components requires Origin declaration." With k = 1, the max-over-components loop is trivial (one iteration). The multi-component max-selection problem does not arise.

**OC-M0R-2:** RESOLVED by the k = 1 topology declaration.

---

## 5. Phase 2: Temporal Change via Persistence Operators

### 5.1 operator_a_persistence

From `operators.rs` lines 573–587:

```rust
a_p[c][e] = e_current.spatial[c][e] − e_prior.spatial[c][e]
```

At step t:

```
A_p.spatial[0][e_τV] = E_current.spatial[0][e_τV] − E_prior.spatial[0][e_τV]
                     = τ_s(t) × R_W/K_T − τ_s(t−1) × R_W/K_T
                     = (τ_s(t) − τ_s(t−1)) × R_W/K_T
                     = Δ_τ × R_W/K_T
                     = Δ_V                             [V]
```

**Δ_V is the actual voltage change that occurred in the motor circuit between steps t−1 and t.** The motor circuit carried τ_s(t−1) × R_W/K_T at the prior step and carries τ_s(t) × R_W/K_T at the present step. The persistence operator reads this change directly from the stored Phase 1 EdgeField states — no spatial coupling edges required. The circuit event is already in the PersistenceState; operator_a_persistence reads it.

### 5.2 B_p and R_p (Phase 2)

`adj⁺[e_τV]` = {} (relational isolation holds for Phase 2 as well — same topology).

```
B_p.spatial[0][e_τV] = A_p.spatial[0][e_τV] + 0 = Δ_V    [V]

chi_p[v_τ]  = |A_p.spatial[0][e_τV]| = Δ_V               [V]
chi_p[v_ref]= |A_p.spatial[0][e_τV]| = Δ_V               [V]

ρ_p[e_τV] = ρ_base × Δ_V / (χ₀ + Δ_V)                    [dimensionless]

R_p.spatial[0][e_τV] = B_p.spatial[0][e_τV]
                     + ρ_p[e_τV] × (0 − 0)
                     = Δ_V                                  [V]
```

Phase 2 output R_p[e_τV] = Δ_V ✓. The antisymmetric contribution is structurally zero for the same reason as Phase 1.

### 5.3 Sigma Accumulation

```
σ_v(t) = σ_v(t−1) + R_p.spatial[0][e_τV]
        = σ_v(t−1) + Δ_V                                   [V]
```

Sigma accumulates the Phase 2 R operator output — the actual voltage changes that occurred in the motor circuit at each step. σ_v is the electrical record of the circuit's history of torque events. It is not an approximation or a model of the circuit; it is the accumulated relational state of the running loop. R is the formal path (not substituted by B or A_p directly).

---

## 6. Worked Example: 25 N·m Disturbance Step

**Input:**

```
τ_s(t)      = 27.553 N·m    (admitted: 27.553 ≤ 68.57 ✓)
τ_s(t−1)    = 2.553  N·m    (from PersistenceState)
σ_v(t−1)    = 1.787  V
θ           = 0.17453 rad   (10°)
R_W/K_T     = 0.7/1.0 = 0.7 V/(N·m)
```

**NodeField construction (k=1, [V]):**

```
f.data[0][v_τ]   = 27.553 × 0.7 = 19.287 V
f.data[0][v_ref] = 0.0 V
```

**Phase 1:**

```
A.spatial[0][e_τV]  = 19.287 − 0.000 = 19.287 V    [PASS: V]
B.spatial[0][e_τV]  = 19.287 + 0     = 19.287 V    [PASS: V; terminal, adj⁺ empty]

chi[v_τ]  = 19.287 V
chi[v_ref]= 19.287 V

χ₀ = V_BUS = 48.0 V
ρ[e_τV] = ρ_base × 19.287 / (48.0 + 19.287)
         = ρ_base × 19.287 / 67.287
         = ρ_base × 0.2866                          [dimensionless]

R.spatial[0][e_τV] = 19.287 + ρ_base × 0.2866 × 0  = 19.287 V    [PASS: V]

E_current.spatial[0][e_τV] = 19.287 V    → stored in PersistenceState
```

**Phase 2 (persistence):**

```
E_prior.spatial[0][e_τV] = 2.553 × 0.7 = 1.787 V    (from PersistenceState)

A_p.spatial[0][e_τV] = 19.287 − 1.787 = 17.500 V    [PASS: V] = Δ_V ✓

B_p.spatial[0][e_τV] = 17.500 V                      [PASS: V; terminal]

chi_p[v_τ]  = 17.500 V
chi_p[v_ref]= 17.500 V
ρ_p[e_τV]  = ρ_base × 17.500 / (48.0 + 17.500)
            = ρ_base × 17.500 / 65.500
            = ρ_base × 0.2672                        [dimensionless]

R_p.spatial[0][e_τV] = 17.500 + ρ_base × 0.2672 × 0 = 17.500 V    [PASS: V]
```

**Sigma and C:**

```
σ_v(t) = 1.787 + 17.500 = 19.287 V                  [PASS: V]

frac    = clamp((0.34907 − 0.17453) / 0.34907, −1, 1) = 0.500
v_cmd   = 19.287 + 0.500 × 19.0 = 28.787 V          [PASS: V]
```

**All outputs pass dimensional check:**

| Quantity | Value | Unit | Status |
|----------|-------|------|--------|
| A[e_τV] Phase 1 | 19.287 | V | PASS |
| B[e_τV] Phase 1 | 19.287 | V | PASS |
| R[e_τV] Phase 1 | 19.287 | V | PASS |
| chi Phase 1 | 19.287 | V | PASS |
| ρ Phase 1 | ρ_base × 0.2866 | — | PASS (dimensionless) |
| A_p[e_τV] | 17.500 | V | PASS |
| B_p[e_τV] | 17.500 | V | PASS |
| R_p[e_τV] | 17.500 | V | PASS |
| sigma_v | 19.287 | V | PASS |
| v_cmd | 28.787 | V | PASS |

**OC-M0R-6 RESOLVED.** No abstraction label required. All outputs are dimensionally coherent.

---

## 7. Structural R Expression

The R expression for the corrected topology, written symbolically to separate what the kernel computes from what requires a numerical ρ_base declaration:

### 7.1 General Form (from `operators.rs` lines 522–537)

```
R[c][e_τV] = B[c][e_τV]
           + ρ[e_τV] × (Σ_{f ∈ adj⁺[e_τV]} B[c][f]  −  Σ_{f ∈ adj⁻[e_τV]} B[c][f])
           + Σ_{p} (B.comp[p][src(e_τV)] × cc[p]) × direction_sign[c,p]
```

### 7.2 Applied to the Corrected Topology

**adj⁺[e_τV] = {}, adj⁻[e_τV] = {}, comp_pairs = {}, cc = {}:**

```
R[0][e_τV] = B[0][e_τV]
           + ρ[e_τV] × (0 − 0)
           + 0
           = B[0][e_τV]
           = τ_s × R_W/K_T                           [V]
```

**Mathematical computation (does not require ρ_base):**

```
R[0][e_τV] = B[0][e_τV] = τ_s × R_W/K_T             [V]
```

The circuit voltage is known exactly from the admitted torque and declared motor constants. The antisymmetric ρ contribution propagates to zero in R's output for the 1-edge topology — but ρ is evaluated at every step as a live quantity.

**ρ at each step (live evaluation):**

```
ρ[e_τV] = ρ_base × chi[v_τ] / (χ₀ + chi[v_τ])
         = ρ_base × (τ_s × R_W/K_T) / (V_BUS + τ_s × R_W/K_T)  [dimensionless]
```

ρ[e_τV] is non-zero at every step the loop is running (ρ_base × 0.2866 for the disturbance example). It is the antisymmetric coupling weight of the relational tension the circuit carries — the ratio of the present circuit voltage to the total electrical range (present + ceiling). ρ multiplies Σ_succ − Σ_pred; that sum is zero by topology structure because e_τV has no adj⁺ or adj⁻. The zero propagated into R[0][e_τV] is the product of a live ρ value and a structurally absent adjacency. ρ itself is not absent. ρ_base governs a quantity that is computed in the running system at every step.

**Same analysis for Phase 2 R_p:**

```
R_p[0][e_τV] = B_p[0][e_τV] = Δ_V = (τ_s(t) − τ_s(t−1)) × R_W/K_T   [V]

ρ_p[e_τV] = ρ_base × Δ_V / (V_BUS + Δ_V)            [dimensionless]

ρ_p × (Σ_succ − Σ_pred) = 0                          [structural zero]
```

Sigma accumulates R_p (not B_p directly — the formal operator sequence A_p → B_p → R_p is preserved; numerical equality arises from topology structure).

---

## 8. R Parameter Independence Analysis

### 8.1 Parameters Required by the Kernel

`require_rho_declaration(rho_base, chi_0)` (lines 231–240) asserts:
- `rho_base.is_finite()` — must be a finite f64
- `chi_0.is_finite() && chi_0 > 0.0` — must be a finite positive f64

Both must be declared through M before evaluation. Neither has a default.

### 8.2 Parameters That Follow from the Established Mapping

**χ₀ = V_BUS = 48.0 V:** Follows from:
1. Chi is computed from A-field values; A-field is in V (corrected topology)
2. χ₀ must be in the same unit as chi (commensurateness requirement for the saturating form)
3. V_BUS is the only declared electrical ceiling in V
4. Cross-check: TAU_ELEC_MAX × R_W/K_T = 48.0 V = V_BUS ✓

χ₀ is **determined by the corrected topology and declared domain constants**. It is not an independent Origin decision.

**cc = [] (empty):** Follows from k = 1 (no multiple components to couple) and no declared cross-topology structure. The `require_cc_declaration` assertion is satisfied by an empty array matching empty comp_pairs.

**OC-M0R-2 (max-selection rule over components):** Resolved by k = 1. With a single component, "max over c" is trivial. No Origin declaration needed for the selection rule.

**ρ_base units (dimensionless):** Follows from unit analysis (§4.4). No Origin declaration needed for units.

**Antisymmetric contribution = 0:** Follows from topology structure (relational_isolation = true). This is a property of the M0 1-edge topology, not a parameter choice.

### 8.3 Parameters That Are Genuinely Independent Domain Choices

**ρ_base (dimensionless):** The antisymmetric coupling coefficient — the ceiling on how strongly R can modify B when the relational tension in the circuit is resolved through connected edges. No declared domain constant determines its value.

ρ[e_τV] = ρ_base × (circuit voltage) / (circuit voltage + electrical ceiling) is evaluated at every step of the running loop. It expresses the antisymmetric relational weight of the present circuit state relative to the total electrical range. Its product with the adjacency sum is zero by topology structure (e_τV has no adj⁺ or adj⁻), so ρ does not propagate into R's output for the 1-edge M0 topology. But ρ is computed — the relational tension it measures is present in the circuit at every step.

ρ_base is required as a finite declaration by `require_rho_declaration` before evaluation. It is an active open condition, not a deferred one: the system is running with ρ at every step.

**OPEN: OC-FD-1.** ρ_base must be declared by Origin before evaluation.

ρ[e_τV] is computed at every step of the running system:

```
ρ[e_τV] = ρ_base × chi[v_τ] / (V_BUS + chi[v_τ])
```

For the disturbance step example: ρ[e_τV] = ρ_base × 0.2866. This is a live, non-zero quantity in each evaluation. Its product with the antisymmetric coefficient (Σ_succ − Σ_pred = 0 for the 1-edge topology) is zero by topology structure, so ρ does not propagate into the R output for M0. But ρ is evaluated — it is not absent or inert. ρ_base governs an active computed quantity in the running system. The structural zero of the antisymmetric term is a topology property, not a declaration that ρ_base is irrelevant.

**OC-M0R-1 (saturating form of ρ):** The functional form `ρ_base × χ/(χ₀ + χ)` is stated in the kernel implementation but its derivation is not closed. Whether a different functional form is correct requires Origin confirmation. This is independent of ρ_base's value.

**OPEN: OC-M0R-1.** The saturating form is as-implemented in Kernel V8; its derivation requires Origin confirmation.

### 8.4 Summary of R Parameter Status

| Parameter | Value | Determination | Status |
|-----------|-------|---------------|--------|
| χ₀ | 48.0 V | V_BUS × (K_T/K_T) = V_BUS; TAU_ELEC_MAX × R_W/K_T = V_BUS ✓ | **RESOLVED** |
| cc | [] | k=1, no comp_pairs; empty cc satisfies require_cc_declaration | **RESOLVED** |
| ρ_base (units) | dimensionless | Unit analysis of ρ × (V) + B[V] = R[V] | **RESOLVED** |
| max-sel. rule | trivial (k=1) | Single component, no multi-dim comparison | **RESOLVED** |
| ρ_base (value) | (undeclared) | No declared domain constant determines it; ρ is computed live each step | **OPEN: OC-FD-1** |
| Saturating form | as-implemented | Derivation of χ/(χ₀+χ) not closed | **OPEN: OC-M0R-1** |

---

## 9. Minimal Active-State Inventory (Corrected)

### 9.1 Variables Required for Evaluation

| Variable | Type | Unit | Carrier | Required by | Lifetime |
|----------|------|------|---------|-------------|----------|
| `theta` | observed | rad | direct | C projection | single step; refreshed each admitted observation |
| `sigma_v` | accumulated | V | Sigma | C projection | trajectory-persistent; += R_p each step |
| `e_prior` (PersistenceState) | EdgeField | V | PersistenceState | Phase 2 A_p | single step; replaced by E_current each step |

**Minimum active state: 3 variables** (reduced from 4 in prior design).

### 9.2 Why tau_s_prior Is Not a Separate Variable

In M0 V2, `tau_s_prior` (N·m) was stored as a distinct active-state variable. In the corrected topology, `tau_s_prior` is encoded in `e_prior`:

```
e_prior.spatial[0][e_τV] = τ_s(t−1) × R_W/K_T    [V]
```

Recovery: `τ_s(t−1) = e_prior.spatial[0][e_τV] × K_T/R_W`

`tau_s_prior` as a separate N·m variable is **eliminated**. It is derivable from `e_prior`. Do not store it independently.

### 9.3 What E_prior Encodes

```
E_prior.spatial[0][e_τV] = R[0][e_τV]_prior = τ_s(t−1) × R_W/K_T    [V]
```

This is the Phase 1 R output from the previous evaluation — the "prior relational potential." A_p takes E_current − E_prior to produce Δ_V. The PersistenceState mechanism is the correct carrier of this information.

### 9.4 Variables NOT in Active State

| Variable | Status | Reason |
|----------|--------|--------|
| `tau_s_new` | present observation only | Admitted through M; enters NodeField; released after step |
| `A[e_τV]` | intermediate | = τ_s × R_W/K_T = E_current[e_τV]; only E_current persists |
| `B[e_τV]` | intermediate | = A[e_τV] for terminal edge; not retained |
| `R[e_τV]` | intermediate → stored as e_prior | The Phase 1 R output becomes E_current; this is e_prior at the next step |
| `A_p`, `B_p` | intermediate | Released after R_p is computed |
| `R_p[e_τV]` | intermediate | Value Δ_V added to sigma_v; then released |
| K_T, R_W, V_BUS, TAU_ELEC_MAX, V_RAISE | reference | Declared constants; immutable; not accumulated |
| Topology, comp_pairs | reference | Immutable; not accumulated |

---

## 10. Recommended Corrected Domain Declaration

### 10.1 Complete Specification

**Project:** `abr-minimal-ai` — M0 Corrected  
**Revision:** Corrected from M0 V2; OC-M0R-6 resolved  
**Kernel:** V8 (`abr-irrs` commit `2a58e46`)

---

**Region:** Single-axis mechanical joint with electrical drive — load-bearing arm.

**Observable extent (unchanged from m_declaration):**

| Observable | Symbol | Unit |
|-----------|--------|------|
| Load-side joint torque | τ_s | N·m |
| Joint angle | θ | rad |
| Motor voltage command | v_cmd | V |

**Declared constants (unchanged from m_declaration):**

| Symbol | Value | Unit | Provenance |
|--------|-------|------|------------|
| K_T | 1.0 | N·m/A | Declared motor constant |
| R_W | 0.7 | Ω | Declared motor constant |
| V_BUS | 48.0 | V | Declared drive specification |
| TAU_ELEC_MAX | 68.57 | N·m | V_BUS × K_T / R_W |
| V_RAISE | 19.0 | V | Declared raise parameter |

---

**Corrected spatial topology:**

```
Loci:      [v_τ,   v_ref]
Edges:     [(v_τ → v_ref)]  →  e_τV
Provenance:[Spatial]
k:          1  (single component, unit: V)
comp_pairs: []
cc:         []
```

**NodeField construction (applied before kernel evaluation):**

```
f.data[0][v_τ]   ← τ_s(t) × R_W / K_T        [V]
f.data[0][v_ref] ← 0.0                         [V]
```

The motor circuit carries V = τ_s × R_W/K_T as a physical electrical potential at the admitted torque. M admits τ_s; the NodeField declares this circuit voltage. v_ref is the circuit ground. The kernel receives an electrical state and operates entirely in volts.

---

**Admission rule (unchanged):**

```
|τ_s(t)| ≤ TAU_ELEC_MAX = 68.57 N·m
```

---

**Operator chain:**

```
τ_s(t) → M (admission) → NodeField construction → operator_e_v5 → Sigma → C → v_cmd
```

**Phase 1 (present circuit state, A→B→R):**

```
A[0][e_τV]  = τ_s(t) × R_W/K_T                          [V]  — relational potential of present circuit state
B[0][e_τV]  = A[0][e_τV]                                 [V]  (terminal — no adjacency to accumulate)
chi[v_τ]    = A[0][e_τV] = τ_s(t) × R_W/K_T             [V]  — circuit voltage at v_τ
ρ[e_τV]     = ρ_base × chi[v_τ] / (V_BUS + chi[v_τ])    [—]  — antisymmetric coupling weight (live, non-zero)
R[0][e_τV]  = B[0][e_τV]  (antisymmetric term = 0 by topology structure)  [V]
E_current   = R output → stored in PersistenceState; records circuit voltage at this step
```

**Phase 2 (circuit voltage change between steps, A_p→B_p→R_p):**

```
A_p[0][e_τV]  = E_current[0][e_τV] − E_prior[0][e_τV]
              = (τ_s(t) − τ_s(t−1)) × R_W/K_T
              = Δ_V                                        [V]  — actual voltage change that occurred
B_p[0][e_τV]  = Δ_V                                       [V]  (terminal)
chi_p[v_τ]    = Δ_V                                        [V]
ρ_p[e_τV]     = ρ_base × Δ_V / (V_BUS + Δ_V)             [—]  (live, non-zero)
R_p[0][e_τV]  = Δ_V  (antisymmetric term = 0 by topology structure)  [V]
```

**Sigma and C (unchanged structure):**

```
σ_v(t)  = σ_v(t−1) + R_p[0][e_τV]
        = σ_v(t−1) + Δ_V                                  [V]  — accumulated electrical record of circuit events

frac    = clamp((θ_target − θ) / θ_target, −1, 1)
v_cmd   = σ_v(t) + frac × V_RAISE                         [V]  — voltage output into motor drive
```

---

**ρ parameters:**

| Parameter | Value | Status |
|-----------|-------|--------|
| χ₀ | V_BUS = 48.0 V | **Declared: follows from NodeField unit and electrical ceiling** |
| ρ_base | (undeclared) | **OPEN: OC-FD-1 — genuinely independent domain choice** |
| cc | [] | Declared: empty (k=1, no cross-topology structure) |

---

**Relational isolation:**

The corrected M0 topology has relational_isolation = true. ρ[e_τV] is computed at every evaluation as a live non-zero quantity (= ρ_base × chi/(V_BUS + chi)). Its product with Σ_succ − Σ_pred is zero by topology structure (adj⁺ and adj⁻ empty). The R antisymmetric term propagates to zero in the output; ρ itself does not. ρ_base governs an active computed quantity in the running system. OC-FD-1 is an active open condition, not a deferred one.

---

### 10.2 State Transition

```
Prior active state:    {σ_v(t−1), θ(t−1), e_prior}
Admitted observation:  {τ_s(t), θ(t)}

Phase 1:   A → B → R → E_current
Phase 2:   A_p → B_p → R_p → Δ_V = R_p[0][e_τV]

Updated active state:  {σ_v(t) = σ_v(t−1) + Δ_V,
                        θ(t),
                        PersistenceState::from_prior(E_current)}
```

Released after step: `τ_s(t), τ_s(t−1), A, B, R (intermediate), A_p, B_p, R_p (intermediate), Δ_V`

---

## 11. Open Conditions Summary

| Condition | Subject | Prior status | Current status |
|-----------|---------|--------------|----------------|
| OC-M0R-6 | B dimensional coherence | OPEN | **RESOLVED** — topology redesign (§2–§3) |
| OC-FD-2 | χ₀ declaration | OPEN | **RESOLVED** — χ₀ = V_BUS = 48.0 V (§4.3, §8) |
| OC-M0R-2 | max-sel. rule (multi-component chi) | OPEN | **RESOLVED** — k=1 topology (§4.5, §8) |
| OC-FD-1 | ρ_base declaration | OPEN | **OPEN** — genuinely independent; zero contribution for M0 topology (§6, §8) |
| OC-M0R-1 | Saturating form derivation | OPEN | **OPEN** — as-implemented; derivation not closed (§8) |
| OC-EM-1 | ε_episode admission criterion | OPEN | unchanged |
| OC-EM-2 | Recall thresholds ε_A, ε_τ, ε_θ | OPEN | unchanged |
| OC-EM-3 | Episodic adjustment for C | OPEN | unchanged |
| OC-SIT-1 | Event-triggered Sigma accuracy | OPEN | unchanged |

**Conditions resolved in this document: OC-M0R-6, OC-FD-2, OC-M0R-2**  
**Conditions returned to Origin: OC-FD-1, OC-M0R-1**

---

## 12. Source Pinning

| Repository | Commit | Purpose |
|-----------|--------|---------|
| `abr-irrs` | `2a58e46` | Kernel V8: operators, DeclaredRelations, adj⁺/adj⁻, PersistenceState, compute_rho |
| `abr-robotics-demo` | `09ea3052` | m_declaration V1.0 (basis for corrected topology) |
| `abr-minimal-ai` | `ad20fd2` | Preceding specifications (OC-M0R-6 analysis, state architecture) |

---

*Kernel V8 frozen at `abr-irrs` commit `2a58e46`. This document is historical evidence and must not be modified retroactively. Future declarations that resolve OPEN conditions shall reference this document by commit.*

*Generator: Claude Sonnet 4.6 — Session https://claude.ai/code/session_01CFPKeATovNRJR4oHjYyLM6*
