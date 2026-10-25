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

**Root cause of OC-M0R-6:** The M0 V2 topology conflates two structurally distinct mechanisms:

1. **Temporal persistence** — the A-operator difference between present and prior torque observations. This is not a spatial relationship; it is the kernel's authorized persistence mechanism (PersistenceState, operator_a_persistence).

2. **Domain transformation** — the algebraic mapping τ → I → V via declared constants K_T and R_W. This is not a kernel operator; it is a declared physical law applied at the NodeField level before the operators run.

The M0 V2 declaration places both mechanisms in the spatial topology as edges. The kernel's B operator then accumulates incoherent combinations of N·m, A, and V values.

---

## 2. Corrected Topology

### 2.1 Remove Temporal Persistence from the Spatial Topology

Temporal persistence — the comparison of τ_s(t) to τ_s(t−1) — belongs in the kernel's authorized persistence mechanism:

```
PersistenceState::from_prior(e_current)  →  operator_a_persistence
```

`operator_a_persistence` computes `E_current[e] − E_prior[e]` on the Phase 1 EdgeField output. This is the correct structural location for temporal differences.

**e_persist is removed from the spatial topology.** No temporal edge is introduced to replace it.

### 2.2 Domain Transformation in the NodeField

The algebraic conversion τ [N·m] → V [V]:

```
V_equiv = τ_s × R_W / K_T
```

This uses both declared motor constants. With K_T = 1.0 N·m/A and R_W = 0.7 Ω:

```
V_equiv [V] = τ_s [N·m] × R_W [Ω] / K_T [N·m/A]
            = τ_s × 0.7  (numerical)
```

The torque observation is expressed as a voltage-equivalent potential in the NodeField. The conversion chain τ → I → V is embedded in this single factor R_W/K_T, which uses both declared constants. The intermediate current quantity I = τ/K_T is not represented as a separate graph node, because:

1. I is not in the observable extent (m_declaration §2).
2. I is a derived quantity, not an admitted observation.
3. A separate intermediate locus carrying I would require an independent node value that either duplicates the V_equiv value (making it redundant) or introduces temporal information (prohibited for the spatial topology).

### 2.3 Corrected Spatial Topology

**Loci (2):**

| Locus | Symbol | NodeField value | Unit | Role |
|-------|--------|-----------------|------|------|
| v_τ | Torque locus | τ_s × R_W/K_T | V | Present torque as voltage potential |
| v_ref | Electrical reference | 0.0 | V | Declared zero-voltage reference |

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

The reference locus carries the declared electrical ground. This is a fixed declared value, not an observation.

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

This is the Phase 1 relational potential — the present torque expressed as a voltage-equivalent. It is stored in PersistenceState for Phase 2 at the next step.

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

**Chi is dimensionally homogeneous [V].** The normalization is achieved by declaring the NodeField in a single physical dimension (V) using the declared domain mapping R_W/K_T, before the kernel receives it. No kernel modification is required.

### 4.3 χ₀ Candidate: V_BUS = 48.0 V

Chi is in volts. The declared saturation parameter χ₀ must carry the same unit as chi (since ρ = ρ_base × chi/(χ₀ + chi) requires chi and χ₀ to be commensurate):

```
χ₀ must be declared in [V]
```

The declared domain has one voltage ceiling: **V_BUS = 48.0 V** (declared drive specification, m_declaration §3).

Physical justification: V_BUS is the maximum electrical potential available to the drive circuit. As chi → V_BUS, the torque observation is approaching the declared electrical maximum. V_BUS provides the natural saturation reference: at chi = V_BUS, ρ = ρ_base/2 (half-maximum coupling weight).

Cross-check: TAU_ELEC_MAX × R_W/K_T = 68.57 × 0.7 = 48.0 V = V_BUS ✓

The electrical maximum torque, expressed in the NodeField's voltage unit, equals V_BUS exactly. This confirms V_BUS as the natural saturation scale.

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

**Δ_V = Δ_τ × R_W/K_T is produced exactly by the authorized persistence mechanism, without any spatial coupling edges.** The physical conversion τ → V is expressed via the NodeField normalization R_W/K_T; the temporal difference Δ_τ is expressed via the persistence operator's EdgeField subtraction.

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

Sigma accumulates the Phase 2 R operator output. This preserves the formal role of R in the Sigma path (not substituted by B or A_p directly).

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

The B value is known exactly from admitted observables and declared constants. The antisymmetric ρ contribution is structurally zero.

**Numerical evaluation requiring ρ_base:**

```
ρ[e_τV] = ρ_base × chi[v_τ] / (χ₀ + chi[v_τ])
         = ρ_base × (τ_s × R_W/K_T) / (V_BUS + τ_s × R_W/K_T)  [dimensionless]
```

This is computable once ρ_base is declared, but its product with zero leaves R[0][e_τV] = B[0][e_τV] regardless. The structural zero is a property of the topology (relational_isolation = true), not of the ρ value.

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

**ρ_base (dimensionless):** Represents the maximum coupling weight of the antisymmetric relational term as the declared system approaches its electrical ceiling. No declared domain constant determines its value.

- For M0's 1-edge topology: ρ_base's contribution is structurally zero in both Phase 1 and Phase 2 (relational_isolation = true; antisymmetric term has no coefficients)
- ρ_base is still required as a finite declaration by `require_rho_declaration`
- ρ_base becomes numerically significant only when the topology is extended to include interior nodes (multi-edge path where adj⁺ or adj⁻ is non-empty)
- Physical interpretation: the dimensionless ceiling on how strongly R modifies B in the presence of antisymmetric torque-voltage tension

**OPEN: OC-FD-1.** ρ_base must be declared by Origin before evaluation. Its contribution to M0 is structurally zero; its value matters for topology extensions.

**OC-M0R-1 (saturating form of ρ):** The functional form `ρ_base × χ/(χ₀ + χ)` is stated in the kernel implementation but its derivation is not closed. Whether a different functional form is correct requires Origin confirmation. This is independent of ρ_base's value.

**OPEN: OC-M0R-1.** The saturating form is as-implemented in Kernel V8; its derivation requires Origin confirmation.

### 8.4 Summary of R Parameter Status

| Parameter | Value | Determination | Status |
|-----------|-------|---------------|--------|
| χ₀ | 48.0 V | V_BUS × (K_T/K_T) = V_BUS; TAU_ELEC_MAX × R_W/K_T = V_BUS ✓ | **RESOLVED** |
| cc | [] | k=1, no comp_pairs; empty cc satisfies require_cc_declaration | **RESOLVED** |
| ρ_base (units) | dimensionless | Unit analysis of ρ × (V) + B[V] = R[V] | **RESOLVED** |
| max-sel. rule | trivial (k=1) | Single component, no multi-dim comparison | **RESOLVED** |
| ρ_base (value) | (undeclared) | No declared domain constant determines it | **OPEN: OC-FD-1** |
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

The conversion τ [N·m] → V [V] via R_W/K_T is applied once, at the NodeField. The kernel operates entirely in volts.

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

**Phase 1 (spatial state, A→B→R):**

```
A[0][e_τV]  = τ_s(t) × R_W/K_T                          [V]
B[0][e_τV]  = A[0][e_τV]                                 [V]  (terminal)
chi[v_τ]    = A[0][e_τV] = τ_s(t) × R_W/K_T             [V]
ρ[e_τV]     = ρ_base × chi[v_τ] / (V_BUS + chi[v_τ])    [—]
R[0][e_τV]  = B[0][e_τV]  (antisymmetric term = 0)       [V]
E_current   = R output → stored as PersistenceState for next step
```

**Phase 2 (temporal change, A_p→B_p→R_p):**

```
A_p[0][e_τV]  = E_current[0][e_τV] − E_prior[0][e_τV]
              = (τ_s(t) − τ_s(t−1)) × R_W/K_T
              = Δ_V                                        [V]
B_p[0][e_τV]  = Δ_V                                       [V]  (terminal)
chi_p[v_τ]    = Δ_V                                        [V]
ρ_p[e_τV]     = ρ_base × Δ_V / (V_BUS + Δ_V)             [—]
R_p[0][e_τV]  = Δ_V  (antisymmetric term = 0)             [V]
```

**Sigma and C (unchanged structure):**

```
σ_v(t)  = σ_v(t−1) + R_p[0][e_τV]
        = σ_v(t−1) + Δ_V                                  [V]

frac    = clamp((θ_target − θ) / θ_target, −1, 1)
v_cmd   = σ_v(t) + frac × V_RAISE                         [V]
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

The corrected M0 topology has relational_isolation = true. The R antisymmetric term is structurally zero in both Phase 1 and Phase 2. ρ_base's numerical contribution to M0 outputs is zero by topology structure. ρ_base becomes non-trivially significant when the topology is extended to a multi-edge path with interior nodes.

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
