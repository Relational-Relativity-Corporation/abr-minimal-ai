# Motor–Sensor–Actuator Minimality Audit

**Document:** `motor_sensor_actuator_minimality_audit.md`  
**Physical reference:** `abr-robotics-demo` V0.5.2 — `abr_flex_demo.py`  
**Mathematical authority:** Frozen Kernel V8, `abr-irrs` commit `2a58e46`  
**Depends on:** `motor_sensor_actuator_observables.md`, `motor_sensor_actuator_relational_declaration.md`, `motor_sensor_actuator_operator_trace.md`  
**Status:** Generator deliverable — audit only; no implementation authorized

---

## 1. Audit criterion

For every proposed computational element, the audit establishes:

1. Which observed relationship requires it?
2. Which admitted quantity supplies its value?
3. Which operator uses it?
4. What information would be lost if it were removed?
5. Whether an existing admitted quantity already provides the same information.

The objective is minimum sufficient relational computation — not minimum graph size
and not maximum operator activity.

A computational element is retained if and only if an observed relationship requires
it and no existing admitted quantity already provides the same information.

---

## 2. Elements retained in the declared topology (Document 2)

### 2.1 Locus v_τ — load-side torque sensor

**Which observed relationship requires it?**  
The physical process couples mechanical load to controller observation through the
torque sensor. The sensor is the only channel through which disturbance and gravitational
loading are visible to the controller. Without a locus representing the sensor, the
controller has no observation of the mechanical state.

**Which admitted quantity supplies its value?**  
τ_s [N·m] — returned by `integrate_plant` as `tau_sensed = MASS * G * ARM * sin(theta) + tau_disturbance`.
This is the simulated sensor observation. Its value at each step is an observable.

**Which operator uses it?**  
The A operator uses it as one pole of the relational difference. In the Kernel V8
2-locus declaration, `x[v_τ] = τ_s × R_W / K_T` enters `A(x)[e_1] = x[v_τ] − x[v_cmd]`.

**What information would be lost if removed?**  
All knowledge of mechanical load and disturbance at the controller. Without v_τ, A has
no observation to compare to the command. The relational difference is undefined.

**Does any existing admitted quantity already provide the same information?**  
No. v_cmd does not carry load-side torque information. No other observable in the
admitted set represents the sensor output. Motor torque τ_m is derived from the
command (not independently observed) and does not carry disturbance information.

**Verdict: REQUIRED.**

---

### 2.2 Locus v_cmd — actuator command locus

**Which observed relationship requires it?**  
The closed loop connects the controller's prior command back to the sensor through the
plant. The controller must carry its own prior output to establish a relational reference
against the incoming sensor observation. Without v_cmd, the comparison has no actuator pole.

**Which admitted quantity supplies its value?**  
v_cmd(t−1) [V] — the prior actuator command, retained by the persistence mechanism
between steps. Source: `c_projection(sigma_v, theta)` from the previous step.

**Which operator uses it?**  
The A operator uses it as the other pole: `x[v_cmd] = v_cmd(t−1)` and
`A(x)[e_1] = x[v_τ] − x[v_cmd]`. (In the Python demo, A uses the temporal difference
of τ_s directly; v_cmd is used by C projection but not explicitly in A's computation.
This is a structural difference between Trace P and Trace K — see Document 3.)

**What information would be lost if removed?**  
In Trace K: the relational gap between what was observed and what was commanded.
Without v_cmd, A has only one pole and cannot compute a relational difference.

In Trace P: v_cmd is the C projection target. Without its prior value, sigma_v cannot
be combined with a position term to produce the command.

**Does any existing admitted quantity already provide the same information?**  
No. τ_s does not carry command history. The prior command is distinct from the
current observation.

**Verdict: REQUIRED.**

---

### 2.3 Edge e_1: v_τ → v_cmd

**Which observed relationship requires it?**  
The sensor and actuator are connected through the closed loop: the sensor observes the
plant response to prior commands, and the controller issues new commands based on sensor
readings. This directional relationship (sensor → command calculation) is the relevant
observable adjacency.

**Which admitted quantity supplies its value?**  
After A: `g[e_1] = A(x)[e_1]` — the relational gap in Volts. The edge carries the
operator result, not an independently sensed quantity.

**Which operator uses it?**  
B operates on the edge field produced by A. For this terminal edge, B = identity.
R operates on the B output (OPEN). Sigma accumulates the final operator output.

**What information would be lost if removed?**  
There would be no declared relational structure and A cannot operate. The edge is the
minimal structural requirement for a connected 2-locus topology.

**Does any existing admitted quantity already provide the same information?**  
No. The edge is the structural declaration of the relationship, not a redundant quantity.

**Verdict: REQUIRED.**

---

### 2.4 Dimensional projection τ_s × R_W / K_T

**Which observed relationship requires it?**  
The physical relationship between load-side torque and electrical command voltage is
mediated by the declared motor constants K_T and R_W. τ_s is in N·m; v_cmd is in V.
For A to compute `x[v_τ] − x[v_cmd]` coherently, both node values must be in the same
units within the NodeField component. The projection converts τ_s into Volts using the
declared coupling constants.

**Which admitted quantity supplies its value?**  
τ_s [N·m] from v_τ, combined with declared constants R_W [Ω] and K_T [N·m/A]:
`τ_s × R_W / K_T [V]`

**Which operator uses it?**  
The A operator uses the projected value as `x[v_τ]`. Without the projection, A's
subtraction is dimensionally incoherent.

**What information would be lost if removed?**  
Dimensional coherence of A. Without the projection, the two node values are in
different units and their difference is physically undefined. Alternatively, removing
the projection forces a different topology choice (e.g., a separate intermediate locus
carrying the dimensional conversion) or a different component structure.

**Does any existing admitted quantity already provide the same information?**  
No. The projection uses τ_s (admitted) and the declared motor constants. Neither v_cmd
nor any other admitted quantity carries the load-torque-as-voltage representation.

**Verdict: REQUIRED for dimensional coherence of A. The physical interpretation of
τ_s × R_W / K_T as "voltage equivalent of observed load" is declared, not implied.**

---

### 2.5 PersistenceState — temporal continuity

**Which observed relationship requires it?**  
The physical process continues between observations. The A operator requires the prior
τ_s value (in Trace P) or the prior v_cmd value (in Trace K) to compute a relational
difference. These are temporally displaced quantities from the same continuing process.

**Which admitted quantity supplies its value?**  
- Trace P: prior τ_s — the previous step's sensor reading, retained by `_tau_s_prior`
- Trace K: prior v_cmd — the previous step's actuator command, retained from the prior
  step's C projection output
- sigma_v — the accumulated Sigma state

**Which operator uses it?**  
A requires the prior observation. Sigma requires the accumulated state. C requires
the accumulated sigma_v.

**What information would be lost if removed?**  
Temporal continuity. Without persistence, every step starts from zero with no
relational context. A cannot compute a difference without a prior value. Sigma cannot
accumulate without a carried state.

**Does any existing admitted quantity already provide the same information?**  
No. The current observation τ_s does not contain the prior observation's value.

**Verdict: REQUIRED. Must use kernel's PersistenceState, not a spatial edge.**

---

### 2.6 Sigma accumulation

**Which observed relationship requires it?**  
The physical system requires persistent load compensation: a single observation of τ_s
change may not produce sufficient voltage response. Sigma carries the accumulated
relational history across steps, enabling the actuator command to build over time.

**Which admitted quantity supplies its value?**  
The B (= A for terminal edge) output at each step, accumulated into sigma_v [V].
Source: operator result, not a separately sensed quantity.

**Which operator uses it?**  
C projection uses sigma_v: `v_cmd = sigma_v + frac × V_RAISE`.

**What information would be lost if removed?**  
The accumulated load compensation history. Without Sigma, C projection would have
no memory of prior load conditions, and the command would not track sustained
disturbances.

**Does any existing admitted quantity already provide the same information?**  
No. Neither τ_s nor v_cmd_prior carry the accumulated history. sigma_v is the
dedicated accumulation state.

**Verdict: REQUIRED. Source (Phase 1 gap or Phase 2 Δ_gap) is OPEN (OC-ACC-1).**

---

### 2.7 C projection (v_cmd = sigma_v + frac × V_RAISE)

**Which observed relationship requires it?**  
The actuator command must close the loop: it must translate the accumulated relational
state into a physical voltage command. C combines the load compensation (sigma_v) with
positional authority (frac × V_RAISE) to produce v_cmd.

**Which admitted quantity supplies its value?**  
sigma_v [V] (from Sigma) and θ [rad] (from plant encoder-equivalent observation).

**Which operator uses it?**  
C is not a kernel operator. It is the declared actuator projection. Its output v_cmd
enters the plant as the actuator command u_t.

**What information would be lost if removed?**  
The actuator command. Without C, the operator chain produces no output that enters
the plant. The closed loop is broken.

**Does any existing admitted quantity already provide the same information?**  
No. sigma_v alone does not incorporate positional authority. v_cmd requires both.

**Verdict: REQUIRED. C is a declared projection, not a kernel operator, per handoff §6.6.**

---

## 3. Elements examined and not retained

### 3.1 Motor torque τ_m as separate locus

**Required?** No. τ_m = K_T × (v_cmd / R_W) is a derived quantity from v_cmd using
declared constants. Admitting τ_m as a separate locus would add a node whose value is
entirely determined by the existing admitted quantity v_cmd.

**Redundancy check:** v_cmd already provides the same information. τ_m is a declared
conversion of v_cmd, not an independent observation.

**Verdict: NOT REQUIRED. Redundant with v_cmd + declared constants K_T, R_W.**

---

### 3.2 Winding current I as separate locus

**Required?** No. I = v_cmd / R_W is not independently sensed in this simulation.
It is inferred from v_cmd. Admitting I as a separate locus adds a derived quantity
with no independent sensor.

**Redundancy check:** v_cmd already provides the same information via the declared R_W.
Adding I as a locus would introduce a node whose value is I = v_cmd / R_W — entirely
derivable from v_cmd.

**Verdict: NOT REQUIRED. Not independently sensed; derivable from v_cmd.**

---

### 3.3 Angular velocity ω as separate locus

**Required?** Unknown — OPEN (OC-LOCI-1). ω is available as a simulated state but is
not currently admitted into the ABR operator chain. Its physical role in the relational
calculation (if any) has not been declared by Origin.

**What adjacency would it require?** An admitted ω locus would require a declared
relational edge to v_τ, v_cmd, or a new locus. No such adjacency has been declared.

**Verdict: OPEN (OC-LOCI-1). Not currently admitted; no declared adjacency.**

---

### 3.4 Applied voltage v_applied as separate locus

**Required?** No, in the current simulation. v_applied is not independently represented
in V0.5.2. The simulation uses v_cmd directly. Without an independent v_applied
measurement or model, adding a separate v_applied locus would either duplicate v_cmd
or introduce an unobservable.

**Verdict: NOT REQUIRED in current simulation. Would require plant simulation extension.**

---

### 3.5 Interior junction locus (Delta_tau, Delta_I, Delta_V nodes from Python demo)

**Required?** No. The Python demo's 3-node topology introduces intermediate quantity
nodes (Delta_tau, Delta_I, Delta_V) that do not correspond to distinct physical loci
with independent observables. They represent stages in a chain of dimensional conversions.

Under Kernel V8 B, this topology produces dimensionally incoherent sums
(N·m + V). The projection τ_s × R_W / K_T in the 2-locus declaration
handles the dimensional relationship without requiring intermediate loci.

**Verdict: NOT REQUIRED. Dimensional conversion handled by declared projection;
intermediate nodes do not represent distinct physical observables.**

---

### 3.6 Disturbance τ_d as separate locus

**Required?** No. τ_d is a known experimental input (scenario parameter), not a
controller observation. The controller observes τ_d's effect through τ_s. Admitting τ_d
as a separate locus would require the controller to know the disturbance directly —
which contradicts the experiment's stated condition of "unannounced environmental load."

**Verdict: NOT REQUIRED. τ_d is not a controller observation; its effect is carried by τ_s.**

---

### 3.7 Net torque τ_net as separate locus

**Required?** No. τ_net is computed inside `integrate_plant` and not returned.
It is not observable to the controller. Its value is:
`τ_net = τ_cmd − MASS × G × ARM × sin(θ) − DAMPING × ω − τ_d`
This requires plant parameters (MASS, ARM, G) which are not admitted into the ABR
operator path.

**Verdict: NOT REQUIRED. Not observable; requires plant parameters not in ABR path.**

---

## 4. Computational elements audit summary

| Element | Required? | Reason |
|---|---|---|
| Locus v_τ (torque sensor) | ✓ Required | Only observation channel for mechanical load |
| Locus v_cmd (actuator) | ✓ Required | Relational reference for A comparison |
| Edge e_1: v_τ → v_cmd | ✓ Required | Minimal declared adjacency for 2-locus topology |
| Projection τ_s × R_W / K_T | ✓ Required | Dimensional coherence of A |
| PersistenceState | ✓ Required | Temporal continuity; kernel mechanism |
| Sigma accumulation | ✓ Required | Load compensation memory |
| C projection | ✓ Required | Closed-loop actuator command generation |
| τ_m as locus | ✗ Redundant | Derivable from v_cmd + declared constants |
| I as locus | ✗ Redundant | Not independently sensed; derivable from v_cmd |
| ω as locus | ? OPEN | OC-LOCI-1; no declared adjacency |
| v_applied as locus | ✗ Not in simulation | Not independently represented |
| Interior junction loci | ✗ Not required | B incoherent; projection handles dimensionality |
| τ_d as locus | ✗ Not required | Not a controller observation |
| τ_net as locus | ✗ Not required | Not observable; requires plant parameters |

---

## 5. Minimality statement

The declared 2-locus topology (v_τ, v_cmd, edge e_1) with a single Volt-component
NodeField and the projection τ_s × R_W / K_T is the minimum sufficient declared
structure for representing the motor–sensor–actuator closed-loop relationship under
the Kernel V8 operators, subject to the OPEN conditions listed below.

No admitted quantity can be removed without losing information required for the
relational calculation. No additional quantity is required beyond what the physical
observation provides.

---

## 6. OPEN conditions affecting minimality

| ID | Condition | Effect on minimality |
|---|---|---|
| OC-FD-1 | ρ_base undeclared | R cannot be evaluated; no ρ-based element can be retained or excluded |
| OC-FD-2 | χ₀ undeclared | Same |
| OC-ACC-1 | Sigma source (gap vs Δ_gap) | Determines what the Sigma accumulator carries; affects which A output is the correct input |
| OC-ARCH-1 | 2-locus sufficiency | B=identity and R antisymmetric=0 are structural; whether this is the correct declared topology requires Origin authorization |
| OC-LOCI-1 | ω admission | Would require new locus, new adjacency declaration |
| OC-PROJ-1 | Projection physical interpretation | Whether τ_s × R_W / K_T correctly represents the v_τ observable for the intended calculation |

These conditions are returned to Origin. No additional computational elements are
proposed to resolve them.

---

*Minimality audit complete. All retained elements are required by observed physical
relationships. All examined elements that were not retained have been shown redundant
or not observable. OPEN conditions require Origin declaration before any implementation.*
