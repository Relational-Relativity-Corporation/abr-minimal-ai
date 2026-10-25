# Motor–Sensor–Actuator Relational Declaration

**Document:** `motor_sensor_actuator_relational_declaration.md`  
**Physical reference:** `abr-robotics-demo` V0.5.2 — `abr_flex_demo.py`  
**Mathematical authority:** Frozen Kernel V8, `abr-irrs` commit `2a58e46`  
**Depends on:** `motor_sensor_actuator_observables.md`  
**Status:** Generator deliverable — declaration only; no implementation authorized

---

## 1. Governing principle

The physical process is already connected. The declaration represents the observable
portion of that connection. It does not create the connection.

The declaration determines what enters the operators. The operators act on what the
declaration supplies. Neither the operators nor the desired operator outputs determine
the declaration.

---

## 2. Domain D — bounded observable region

The domain of observation is the motor–sensor–actuator closed loop during one simulation
step. The relevant observable extent is bounded by:

- **Lower bound:** The load-side torque sensor's minimum admitted reading, defined by
  the M admission boundary: `|τ_s| ≤ τ_elec_max = 68.571 N·m`
- **Upper bound:** Same (symmetric admission boundary)
- **Temporal extent:** One simulation step Δt = 1/24 s, plus the prior completed
  relational state retained by PersistenceState

The declared electrical parameters (R_W = 0.7 Ω, K_T = 1.0 N·m/A, V_BUS = 48 V)
define the relational coupling within this domain.

**What is inside D:**
- The load-side torque sensor observation τ_s [N·m]
- The joint angle observation θ [rad]
- The prior actuator command v_cmd(t−1) [V]
- The accumulated controller state sigma_v [V]
- The declared coupling constants K_T and R_W

**What is outside D:**
- Plant parameters (MASS, ARM, G, I_EFF, DAMPING) — inside plant, not observable
  to controller
- The net joint torque τ_net — internal to plant
- The disturbance τ_d — not observable through a sensor channel
- Angular velocity ω — available simulated state, not currently admitted

---

## 3. Admitted loci

The admitted loci are the physical locations whose observable quantities participate
in the relational calculation.

### 3.1 Current admitted loci in V0.5.2

| Locus | Physical referent | Admitted quantity | Units | Classification |
|---|---|---|---|---|
| v_τ | Load-side torque sensor | τ_s | N·m | Simulated sensor observation |
| v_cmd | Actuator electrical interface | v_cmd | V | Prior controller output |

**v_τ** represents the load-side torque sensor. Its observable is τ_s —
the gravitational and disturbance loading observed by the sensor. From
`integrate_plant`:
```python
tau_sensed = MASS * G * ARM * math.sin(theta) + tau_disturbance
```
This is not motor torque.

**v_cmd** represents the actuator command locus. Its observable is the prior
voltage command, v_cmd(t−1). This is the actuator's last known command state —
the relational counterpart of the sensor observation in the closed loop.

### 3.2 Why these two loci

The closed loop connects: sensor observation → relational calculation → actuator
command → plant response → next sensor observation.

The two poles of this relationship are the sensor (v_τ, what the physical process
reported) and the actuator command (v_cmd, what the controller last issued). Their
relational difference is the gap the controller must act on.

### 3.3 Loci that were examined and not admitted

**Motor torque τ_m** — derived from v_cmd: `τ_m = K_T × (v_cmd / R_W)`. It is not
independently measured. Admitting it as a separate locus would add a derived quantity
that carries no information not already in v_cmd. See minimality audit (Document 4).

**Winding current I** — not independently sensed. Inferred from v_cmd. Same reason.

**Angular velocity ω** — available simulated state, not currently a separate sensor.
Its admission would require declaring an angular velocity locus with its own mapping.
This is an OPEN question (OC-LOCI-1, below).

**Interior junction locus** — the handoff (§6.4) prohibits introducing an interior
locus merely to force a nonzero B or R contribution. A junction between v_τ and v_cmd
is not supported by a distinct physical observable. The physical coupling (K_T, R_W)
enters through declared edge constants, not through a third locus.

---

## 4. Connected relationships

The declared relationship between v_τ and v_cmd is the closed-loop coupling. The sensor
observes the mechanical response. The actuator responds with a voltage command. The
physical process connects them.

The direction of the edge is: `v_τ → v_cmd` — the sensor locus is the source, the
actuator command locus is the target. This reflects the direction of relational flow:
observed load state → actuator adjustment.

**Single edge:** e_1: v_τ → v_cmd

**Edge constants:** The declared motor constants K_T and R_W establish the dimensional
relationship between torque and voltage units. Their role in the topology is to support
the dimensional mapping projection defined in §5, not to appear as additional loci.

**Temporal relationship:** The temporal continuity of the relational state is handled
by the kernel's PersistenceState mechanism. The prior observation τ_s(t−1) is retained
by the A operator's state (`_tau_s_prior` in the Python demo; `PersistenceState` in
Kernel V8). This is not represented as a spatial edge.

---

## 5. Dimensional mapping

### 5.1 The incoherence problem

The physical coupling K_T relates N·m to A, and R_W relates A to V. If v_τ carries
τ_s in N·m and v_cmd carries v_cmd in V, a NodeField with a single component would
require those values to be dimensionally homogeneous (the same units), because:

- Kernel V8 A: `A(x)[e] = x[src(e)] − x[tgt(e)]` — subtracts node values
- Subtraction of N·m from V is dimensionally incoherent

### 5.2 Required projection

To support a dimensionally coherent A calculation, τ_s must be projected into the
same units as v_cmd (Volts) using the declared motor constants:

```
projection: τ_s [N·m] × R_W [Ω] / K_T [N·m/A] → τ_s × R_W / K_T [V]
```

Dimensional verification:
```
N·m × Ω / (N·m/A) = N·m × (V/A) / (N·m/A) = N·m × (V·A) / (N·m) = V ✓
```

Physical interpretation: The projected value `τ_s × R_W / K_T` represents the voltage
equivalent of the load-side torque observation — the voltage that would be required
to produce a motor torque balancing that load. This is a declared dimensional mapping,
not a motor model.

### 5.3 NodeField declaration

With this projection:

| Component | Index | Locus v_τ value | Locus v_cmd value | Units |
|---|---|---|---|---|
| 0 | k=0 | τ_s(t) × R_W / K_T | v_cmd(t−1) | V |

A single-component NodeField in Volts:
- `n_nodes = 2` (v_τ, v_cmd)
- `k = 1` (one component, in V)
- At v_τ: `x[v_τ] = τ_s × R_W / K_T` [V]
- At v_cmd: `x[v_cmd] = v_cmd(t−1)` [V]

### 5.4 A operator with this NodeField

```
A(x)[e_1] = x[v_τ] − x[v_cmd]
           = (τ_s × R_W / K_T) − v_cmd(t−1)
           [V]
```

This is the relational gap: the voltage equivalent of the observed load minus the
prior actuator command. Both sides are in Volts. The subtraction is coherent.

At the pre-disturbance steady state, A ≈ 0 (the command approximates the load voltage
equivalent). At the disturbance step, A becomes non-zero as τ_s increases.

---

## 6. Kernel data structures

### 6.1 DeclaredRelations

```
DeclaredRelations::from_edges_with_provenance(
    n_nodes = 2,
    edges   = [(src=0, tgt=1)],   // e_1: v_τ → v_cmd
    provenance = [declared coupling: K_T = 1.0 N·m/A, R_W = 0.7 Ω]
)
```

Node indices: v_τ = 0, v_cmd = 1.

### 6.2 NodeField

```
NodeField::new(data)
  data[component=0][node=v_τ]   = τ_s(t) × R_W / K_T  [V]
  data[component=0][node=v_cmd] = v_cmd(t−1)           [V]
```

### 6.3 EdgeField (after A)

After A operates on NodeField x:
```
g[e_1] = A(x)[e_1] = x[v_τ] − x[v_cmd]  [V]
```

### 6.4 B operator result (2-locus, 1 edge, terminal)

With 2 loci and 1 edge (v_τ → v_cmd):
- `adj⁺(e_1)` = out-edges of tgt(e_1) = out-edges of v_cmd = ∅  (terminal edge)
- `B(g)[e_1] = g[e_1] + Σ_{f ∈ ∅} g[f] = g[e_1]`

B reduces to identity for a terminal edge. No accumulation occurs.

**This is a structural consequence of the 2-locus, 1-edge topology.**
It is not an error, but it means B carries no additional relational structure
beyond A's output. Whether this 2-locus topology is sufficient for the intended
relational calculation is an OPEN condition (OC-ARCH-1 below).

### 6.5 R operator result (relational isolation)

With `adj⁺(e_1) = ∅` and `adj⁻(e_1) = ∅`:
- The antisymmetric accumulation term in R is structurally zero for this step
- R carries A=B output unchanged within this step
- `R(g)[e_1] = g[e_1] + ρ[v_τ] × (Σ_succ − Σ_pred) = g[e_1] + ρ[v_τ] × 0`

**Relational isolation** (as defined in Kernel V8) holds for this topology within
each step. The R operator would still be evaluated (unlike in the Python demo
where R is marked OPEN due to ρ_base and χ₀ not being declared through M).

ρ_base: OPEN (OC-FD-1)  
χ₀: OPEN (OC-FD-2)

### 6.6 PersistenceState

The kernel's temporal mechanism retains the prior completed relational state.
In this topology, persistence carries:
- Prior τ_s value (for A's directed difference calculation)
- sigma_v (the accumulated Sigma state)
- Prior v_cmd (the actuator command locus value for the next NodeField)

These are temporal, not spatial, relationships.

---

## 7. Note on the Python demo's topology vs Kernel V8 B

The `abr_flex_demo.py` V0.5.2 implements a 3-node topology (`Delta_tau`, `Delta_I`,
`Delta_V`) with 2 coupling edges (`tau_to_I`, `I_to_V`). Its "B" operator is a
sequential function application:
```python
delta_I = A_out / K_T    # edge tau_to_I
delta_V = delta_I * R_W  # edge I_to_V
```

This is not Kernel V8 B. The frozen Kernel V8 B operator is:
```
B(g)[e] = g[e] + Σ_{f ∈ adj⁺(e)} g[f]
```

For a path `Delta_tau → Delta_I → Delta_V` with `adj⁺(tau_to_I) = {I_to_V}`:
```
B(g)[tau_to_I] = g[tau_to_I] + g[I_to_V]
               = (26.93 N·m) + (18.85 V)  [dimensionally incoherent]
```

The Python demo's topology would produce an incoherent B sum if the frozen operator
were applied. The handoff (§6.3) states: "The original Python demonstration does this
and therefore must not be treated as proof of Kernel V8 B conformance."

The 2-locus, k=1 in V declaration (this document) avoids this incoherence by projecting
τ_s into Volts before the topology, so A operates on a dimensionally homogeneous NodeField.

---

## 8. Sigma and C projection

**Sigma** accumulates the B (= A) output each step:
```
sigma_v(t) = sigma_v(t−1) + B(g)[e_1]
           = sigma_v(t−1) + (τ_s × R_W / K_T − v_cmd(t−1))  [V]
```

The accumulated quantity is a voltage. The accumulation tracks the total relational
gap between the observed load voltage equivalent and the prior commands.

**C projection** (declared positional authority, not a kernel operator):
```
frac  = clamp((θ_target − θ) / θ_target, −1, 1)
v_cmd = clamp(sigma_v + frac × V_RAISE, −V_BUS, V_BUS)
```

The C projection uses both sigma_v (load compensation history) and θ (current
position relative to target). It is not A, B, or R.

The resulting v_cmd enters the plant and produces the next τ_s observation,
closing the relational loop.

---

## 9. OPEN conditions

| ID | Condition | Nature |
|---|---|---|
| OC-FD-1 | ρ_base value | Origin declaration required; no candidate from observation alone |
| OC-FD-2 | χ₀ value | Candidate: V_BUS = 48 V (relational bound); awaits Origin confirmation |
| OC-ACC-1 | Sigma accumulation source | Phase 1 (A=B output, V) vs Phase 2 (temporal change, V/step); both dimensionally coherent in this declaration; Origin must evaluate behavioral difference |
| OC-ARCH-1 | 2-locus sufficiency | B=identity and R antisymmetric=0 are structural consequences of terminal edge; whether this is sufficient for the intended calculation or requires connected topology requires Origin authorization |
| OC-LOCI-1 | ω admission | Angular velocity is available simulated state not currently admitted; no physical adjacency to v_τ or v_cmd is declared; Origin must determine if it belongs |
| OC-PROJ-1 | Projection physical interpretation | τ_s × R_W / K_T is a dimensional projection, not a motor model; whether it correctly represents the relevant observable relationship at v_τ requires Origin review |

---

## 10. What is not authorized

- No implementation of these structures is authorized by this document
- No kernel modification is authorized
- The OPEN conditions above must be returned to Origin before implementation
- The Python demo's topology (`Delta_tau → Delta_I → Delta_V`) is acknowledged as
  not Kernel V8 B conformant; no alternative implementation is proposed here

---

*This declaration establishes the declared relational structure for the operator trace
(Document 3) and minimality audit (Document 4).*
