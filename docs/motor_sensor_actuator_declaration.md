# Observation-First Declaration — Connected Motor–Sensor–Actuator Process

**Generator:** Claude Sonnet 4.6  
**Session:** https://claude.ai/code/session_01PAsYhqsHWn44mgVCKDkDbk  
**Kernel authority:** Frozen Kernel V8 · `abr-irrs` commit `2a58e46`  
**Preceding specification:** `minimal_reciprocal_relational_calculation.md` (commit `b55f809`)  
**Destination:** Origin (human) and Verifier (ChatGPT)  
**Date:** 2026-10-25  
**Authorization scope:** Mathematical declaration only. No implementation. No kernel modification.

---

## 0. Entry Condition

This document is a return to Origin and Verifier before implementation proceeds. It supersedes the topology in `minimal_reciprocal_relational_calculation.md` as the structural basis for the connected motor–sensor–actuator calculation. It does not supersede that document's dimensional resolution or worked calculation, which remain valid for the isolated topology they describe.

The task: identify the minimum relational structure for the connected motor–sensor–actuator process — not the minimum structure that produces a dimensionally coherent number.

---

## 1. The Physical Participants

The motor–sensor–actuator process has five physical participants. They are all present and interacting before any declaration is made.

| Participant | Symbol | Physical role | Observable? |
|-------------|--------|--------------|-------------|
| Applied motor voltage | v_applied | Voltage the drive delivers to the winding | Indirectly — the system issues v_cmd; v_applied follows |
| Motor activity | I, τ_motor | Winding current (I = v_applied/R_W at steady state); resulting torque (τ_motor = K_T × I) | Not directly; inferred from v_cmd and R_W, K_T constants |
| Mechanical load and response | τ_load, θ | Load torque at the joint; joint angle | θ is admitted through M; τ_load is not directly observed |
| Torque sensor observation | τ_s | Net torque at the joint, admitted through M | **Yes — directly admitted** |
| Subsequent actuator command | v_cmd | Voltage command issued as system output | **Yes — the system's own output; always known** |

**Declared domain constants (from prior work):** K_T = 1.0 N·m/A, R_W = 0.7 Ω, V_BUS = 48.0 V, V_RAISE = 19.0 V.

---

## 2. Relationships That Must Remain Connected

The physical process connects the five participants through three relationships that are always present simultaneously:

**R1 — Electrical coupling: command → motor circuit**  
v_cmd determines the current in the winding: I = v_cmd/R_W (at steady state, ignoring back-EMF). The motor torque follows: τ_motor = K_T × I = v_cmd × K_T/R_W. The command and the motor's electrical state are inseparable within a step.

**R2 — Electromechanical coupling: motor → load**  
τ_motor acts against the mechanical load at the joint. The load's reaction torque τ_load and the motor torque together determine the net torque at the joint. This is the physical coupling between the electrical and mechanical domains.

**R3 — Mechanical-to-sensor: joint → observation**  
The torque sensor observes the net torque at the joint. τ_s is the admitted observable through M — the point where the physical relational process enters the declared domain.

**R4 — Temporal: step t output → step t+1 input**  
v_cmd issued at step t becomes the applied voltage at step t+1. This closes the loop in time. The closure is through PersistenceState, not through a spatial edge — a spatial ring is a DRIFT SIGNAL and is refused by Kernel V8.

All four relationships are always present in the running system. The declaration does not create them. It identifies the minimum structure that admits them.

---

## 3. Quantities Actually Observed (Admitted Through M)

Three quantities are admitted through M:

| Symbol | Quantity | Unit | Role in declared domain |
|--------|----------|------|------------------------|
| τ_s | Joint torque (sensor) | N·m | Load-side observation |
| θ | Joint angle | rad | Position; enters C projection (frac term) |
| v_cmd | Motor voltage command | V | Actuator pole; the system's own prior output |

**Not admitted through M:**  
- Winding current I (unless a current sensor is declared)  
- Back-EMF or shaft speed ω  
- Motor output torque τ_motor independently of load reaction  
- Applied voltage v_applied independently of v_cmd (unless drive feedback is declared)

The three admitted quantities are the observable basis for the entire declared domain. No further observation is available without additional sensor declarations.

---

## 4. The Structural Requirement: Interior Locus

The connected motor–sensor–actuator process requires an **interior locus** in its declared topology.

An interior locus is a declared locus that is both the target of one declared edge and the source of another. Without an interior locus:

- B accumulation reduces to identity: B[e] = A[e] for every terminal edge.
- R's antisymmetric term is structurally zero: adj⁺ and adj⁻ are both empty for every edge.
- The third relational step — "carry that relationship forward through recursive algebra" — is not expressed within a step.

The isolated 2-locus, 1-edge topology (from commit `b55f809`, and from `minimal_reciprocal_relational_calculation.md`) is relational_isolation = true for this structural reason. It captures the directed gap between the two admitted poles (τ_s × R_W/K_T and v_cmd) correctly and dimensionally coherently. What it does not express is the intermediate physical step: the motor is not a direct connection between v_cmd and τ_s — it is a physical process that mediates between them.

The interior locus is the mediation point. It is the locus where the actuator's command arrives and where the sensor measures. In the motor–sensor–actuator system, that locus is the **joint** — the mechanical interface where:
- The actuator drive applies a voltage (electrical input)
- The motor produces torque (mechanical output)
- The load applies a reaction torque (mechanical coupling)
- The torque sensor measures the net torque (the admitted observation)

This is a physically existing, observable locus. It is not introduced for mathematical convenience.

**Structural pattern for the connected topology:**

```
v_act → v_joint → v_sensor
  (command)  (interior)  (observation)
```

This gives:
- e₁: v_act → v_joint (what changes between command and motor output)
- e₂: v_joint → v_sensor (what changes between motor output and torque observation)
- adj⁺[e₁] = out[v_joint] = {e₂} — non-empty
- adj⁻[e₂] = inc[v_joint] = {e₁} — non-empty
- B[e₁] = A[e₁] + A[e₂] — non-trivial accumulation at the interior
- R antisymmetric term at e₁ and e₂ — structurally non-zero

---

## 5. Candidate Minimum Connected Topology

Three loci, two edges, one interior. All provenance candidates — Origin must declare observable evidence for each.

### Loci

| Locus | Symbol | Physical identity | Candidate NodeField value | Unit |
|-------|--------|------------------|--------------------------|------|
| Actuator command | v_act | Command issued by the system | v_cmd(t−1) | V |
| Joint (interior) | v_joint | Motor output / mechanical interface / torque sensor location | **OPEN — see §6** | V or N·m or rad |
| Torque sensor | v_sensor | Torque sensor reading | τ_s(t) × R_W/K_T | V (or τ_s in N·m) |

### Edges

| Edge | Direction | Candidate provenance | Relational meaning |
|------|-----------|---------------------|-------------------|
| e₁ | v_act → v_joint | Continuation (actuator drives the joint — declared from device geometry) | What changes between the actuator command and the motor output locus |
| e₂ | v_joint → v_sensor | Coupling (torque sensor measures at the joint — declared from sensor placement) | What changes between the motor output locus and the torque observation |

**k (component count):** OPEN — depends on NodeField unit declaration (see §6).  
**comp_pairs:** OPEN — depends on whether cross-dimensional coupling is required.  
**Ring inadmissibility:** No spatial edge closes back to v_act. The feedback loop closes through PersistenceState across steps.

---

## 6. The Central OPEN Condition: NodeField at v_joint

The interior locus v_joint is physically observable. What Observable quantity, in what unit, represents v_joint in the declared domain D is the central declaration required from Origin.

The NodeField must be dimensionally coherent: all NodeField components at all loci must carry the same physical unit within each declared component (B accumulates homogeneously).

**Three candidate projections:**

**Option A — All loci in Volts (electrical projection):**

```
f.data[0][v_act]   = v_cmd(t−1)          [V]   ← actuator pole
f.data[0][v_joint] = ?                    [V]   ← motor output in electrical units
f.data[0][v_sensor]= τ_s(t) × R_W/K_T   [V]   ← load-side electrical equivalent
```

What enters v_joint in volts? Candidates:
- Back-EMF: K_B × ω — requires shaft speed ω, which is not admitted through M as declared.
- Torque-equivalent voltage at v_joint derived from τ_s alone: τ_s × R_W/K_T (same as v_sensor) — but this collapses the interior to the sensor, removing the structural distinction between joint and sensor.
- The average of command and torque-equivalent: (v_cmd + τ_s × R_W/K_T)/2 — not a declared observable; inadmissible.

No electrical projection for v_joint is available from the current admitted observables without adding a new observable (ω or a direct mid-circuit measurement).

**Option B — All loci in N·m (torque projection):**

```
f.data[0][v_act]   = v_cmd(t−1) × K_T/R_W   [N·m]   ← command as equivalent torque demand
f.data[0][v_joint] = τ_equivalent_at_joint   [N·m]   ← OPEN — see below
f.data[0][v_sensor]= τ_s(t)                  [N·m]   ← direct sensor reading
```

What enters v_joint in N·m? The joint's mechanical state is characterized by θ (angle, rad) and τ_s (torque, N·m). The torque at the joint is what the sensor reports, but placing τ_s at both v_joint and v_sensor collapses the two loci. If v_joint carries a distinct observable value in N·m, it must come from a separate torque measurement not yet declared.

**Option C — k=2 (electrical V + mechanical N·m), cross-topology coupling:**

```
f.data[0][v_act]    = v_cmd(t−1) [V]      (electrical component)
f.data[0][v_joint]  = τ_s(t) × R_W/K_T [V] or 0.0 [V]  (electrical component at joint)
f.data[0][v_sensor] = 0.0 [V]             (electrical component, terminal)

f.data[1][v_act]    = 0.0 [N·m]           (torque component)
f.data[1][v_joint]  = τ_s(t) [N·m]        (torque component — sensor measurement declared at joint)
f.data[1][v_sensor] = τ_s(t) [N·m]        (torque component at sensor)
```

With comp_pairs = [(0, 1)] and cc declared by Origin. R's cross-topology term then handles the electromechanical coupling between the electrical command and the torque observation, while B accumulates within each component separately. This is the authorized cross-dimensional mechanism in Kernel V8.

**Option D — All loci use θ (position projection, k=1 in rad):**

```
f.data[0][v_act]    = θ_cmd(t−1)   [rad]   ← command as equivalent position demand (requires declared conversion)
f.data[0][v_joint]  = θ(t)         [rad]   ← joint angle — directly admitted through M
f.data[0][v_sensor] = θ_s(t)       [rad]   ← torque as position equivalent (requires declared conversion)
```

θ is admitted through M. If the joint's natural observable state is its angle, placing θ at the interior is consistent with observability. The command must then be projected to position units (requires a declared conversion from V to rad), and the torque sensor reading must be projected to position units (requires a declared torque-to-angle conversion via load stiffness — not currently declared).

**Origin must select one option and declare the observable evidence and conversion for each locus.** Neither the Generator nor the operators choose the projection.

---

## 7. Minimum Calculations Required

Once the NodeField declaration (§6) is resolved:

**Phase 1 (within each step):**

1. M admits τ_s, θ, v_cmd through their declared regions.
2. NodeField f constructed: f.data[c][v_act], f.data[c][v_joint], f.data[c][v_sensor] for each component c.
3. A: directed differences over e₁ and e₂ for each component.
4. B: accumulation at v_joint interior.
   - B[e₁] = A[e₁] + A[e₂] (non-trivial: carries both directed differences through the interior)
   - B[e₂] = A[e₂] (terminal)
5. ρ computation: requires M-declared ρ_base and χ₀ (OPEN: OC-FD-1, OC-FD-2).
6. R: antisymmetric adjustment (non-trivial at e₁ and e₂ since adj⁺ and adj⁻ are both non-empty).
7. Phase 1 result E_k = R(B(A(f_k))).

**Phase 2 (across steps via PersistenceState):**

8. A_persistence[e] = E_k[e] − E_{k−1}[e] for each declared edge.
9. B_persistence and R_persistence follow the same structural path.
10. Phase 2 result: change in relational state across the declared step.

**Output:**

11. Sigma accumulation over Phase 1 R or Phase 2 R_p output (OPEN: OC-ACC-1).
12. v_cmd = sigma + frac(θ) × V_RAISE.
13. v_cmd enters PersistenceState and NodeField at v_act for the next step.

---

## 8. What the Prior Isolated Topology Expresses and What It Does Not

The 2-locus, 1-edge topology from commit `b55f809` and `minimal_reciprocal_relational_calculation.md`:

```
v_τ (torque pole, V) → v_cmd (command pole, V)
```

**What it expresses correctly:**
- The directed gap between the two admitted observable poles: gap = τ_s × R_W/K_T − v_cmd.
- Dimensional coherence: A, B, R all in V.
- Reciprocity: both the load side (τ_s) and the actuator side (v_cmd) participate.
- Temporal persistence through PersistenceState: Δ_gap across steps.

**What it does not express:**
- The interior physical step (the motor and joint) is absent from the topology. The process v_cmd → [motor] → τ_s is represented as a direct 2-node edge with no intermediate.
- B accumulation at an interior is not expressed: adj⁺[e_τC] = {} means B = identity.
- R's antisymmetric adjustment is structurally zero within a step. ρ is computed and non-zero, but multiplies an empty sum.
- The "carry forward within a step" — the second half of the third relational step — has no declared structure to accumulate over.

The prior topology is the minimum sufficient structure for a dimensionally coherent reciprocal gap calculation. It is not the minimum sufficient structure for a connected motor–sensor–actuator relational calculation.

**This finding does not invalidate the prior topology's numerical results.** It identifies what the connected topology adds in structural terms. Whether the additional structure is required by Origin's goals is Origin's judgment.

---

## 9. OPEN Conditions Returned to Origin

| ID | Description | Status | What is needed |
|----|-------------|--------|---------------|
| OC-CONN-1 | NodeField at v_joint (interior) | **OPEN — blocking** | Origin declares: what observable quantity, in what unit, represents the motor output locus? Chooses from Options A–D (§6) or declares an alternative. |
| OC-CONN-2 | NodeField dimensional coherence (k, comp_pairs, cc) | **OPEN — blocking** | Follows from OC-CONN-1 resolution. |
| OC-CONN-3 | Edge provenance for e₁ (v_act → v_joint) | **OPEN — blocking** | Observable evidence for this direction from device geometry/specifications. |
| OC-CONN-4 | Edge provenance for e₂ (v_joint → v_sensor) | **OPEN — blocking** | Observable evidence for this direction from sensor placement/specifications. |
| OC-CONN-5 | Observable extent for v_joint | **OPEN — blocking** | Floor and ceiling of the NodeField observable at v_joint from physical specifications. |
| OC-FD-1 | ρ_base | **OPEN** | Carried from prior work. Dimensionless; no M0 constant determines it. |
| OC-FD-2 | χ₀ | **OPEN** | Carried from prior work. Candidate χ₀ = V_BUS with relational bound argument (prior work §11.2); requires Origin confirmation. |
| OC-ACC-1 | Sigma accumulation: Phase 1 R or Phase 2 R_p | **OPEN** | Carried from `minimal_reciprocal_relational_calculation.md` §6.4. |
| OC-ARCH-1 | Is the connected 3-locus topology required for the project objective? | **OPEN — returned to Origin** | The isolated topology is simpler. Origin declares whether the interior locus (and the non-trivial B and R terms it enables) is required for the intended relational calculation, or whether the isolated gap is sufficient. |

**OC-ARCH-1 is the primary architectural question.** If Origin determines that the isolated gap (2-locus, 1-edge) is the intended calculation, the prior topology is the authorized one. If Origin determines that the connected interior process must be expressed, OC-CONN-1 through OC-CONN-5 must be resolved before the connected topology is declared.

---

## 10. Summary — Structural Findings

The motor–sensor–actuator process connects five physical participants through four relationships. Three are observable through M (τ_s, θ, v_cmd). The temporal relationship closes the loop through PersistenceState.

The minimum topology for the connected process requires an interior locus at the motor output / joint — the physical point where the actuator's command arrives and the torque sensor measures. This interior locus enables non-trivial B accumulation and R antisymmetric adjustment within a step.

The prior 2-locus topology is the minimum topology for the reciprocal gap. The 3-locus topology is the minimum topology for the connected process. Which is the right topology for this project is Origin's declaration.

The NodeField value at the interior locus is the central unresolved question. It requires a declared projection from admitted observables to a common dimensional space. No such projection can be made without Origin authorization from observable evidence.

**No implementation follows this document. Returning to Origin and Verifier.**

---

*Generator output. Origin authorization required before topology selection, NodeField declaration, or implementation proceeds.*  
*Kernel V8 frozen at `abr-irrs` commit `2a58e46`. No kernel modification.*  
*Metatron Dynamics, Inc.*
