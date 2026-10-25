# Motor–Sensor–Actuator Observables

**Document:** `motor_sensor_actuator_observables.md`  
**Physical reference:** `abr-robotics-demo` V0.5.2 — `abr_flex_demo.py`  
**Mathematical authority:** Frozen Kernel V8, `abr-irrs` commit `2a58e46`  
**Status:** Generator deliverable — observation record only; no implementation authorized

---

## 1. Physical system

The reference system is a single-axis robotic arm driven by an electric motor.
An electrical drive supplies the winding. Motor torque interacts with the arm's inertia,
gravitational loading, joint damping, and any applied disturbance.
The arm responds through angular motion. A load-side torque sensor and a joint angle
encoder observe aspects of that response. The controller receives those observations,
performs relational calculations, and issues an actuator command.

The physical process is continuous and connected. Controller output at step t enters the
plant, and the plant produces the sensor observation that enters step t+1.

---

## 2. Plant parameters (not in ABR operator path)

These values define simulated physical behavior. They appear inside `integrate_plant` and
the conventional controller. They are **not** admitted into the ABR operator path.

| Parameter | Symbol | Value | Units | Location in code |
|---|---|---:|---|---|
| Arm mass | — | 3.0 | kg | `MASS` |
| Arm length | — | 0.5 | m | `ARM` |
| Gravity | — | 9.806 | m/s² | `G` |
| Effective inertia | I_eff | 0.75 | kg·m² | `I_EFF = MASS * ARM * ARM` |
| Joint damping | — | 0.5 | N·m·s/rad | `DAMPING` |

`I_EFF` is a derived constant, not a separately measured quantity.

---

## 3. Declared motor constants (admitted into ABR coupling edges)

These appear explicitly in the declared `KernelV8Topology` coupling edges.
They are the physically declared relations between quantities at adjacent loci.

| Constant | Symbol | Value | Units | Role |
|---|---|---:|---|---|
| Winding resistance | R_W | 0.7 | Ω | Edge `I_to_V`: ΔI × R_W = ΔV |
| Torque constant | K_T | 1.0 | N·m/A | Edge `tau_to_I`: Δτ / K_T = ΔI |
| Bus voltage | V_BUS | 48.0 | V | Drive limit; admission boundary |
| Max admitted torque | τ_elec_max | 68.57 | N·m | `V_BUS × K_T / R_W` — M extent |

`τ_elec_max = V_BUS × K_T / R_W = 48.0 × 1.0 / 0.7 = 68.571 N·m`

---

## 4. Controller declaration parameters (admitted into C projection)

These appear in the C projection (declared positional authority, not a kernel operator).

| Parameter | Symbol | Value | Units | Location |
|---|---|---:|---|---|
| Target angle | θ_target | 20° = 0.3491 rad | rad | `THETA_TARGET` |
| Raise authority | V_RAISE | 19.0 | V | `V_RAISE` |
| Declared static load | τ_static | 14.709 | N·m | `MASS * G * ARM` |

`τ_static = 3.0 × 9.806 × 0.5 = 14.709 N·m` — declared design load at full horizontal.
This is a reference constant, not a step-by-step observation.

---

## 5. Scenario parameters

| Parameter | Symbol | Value | Units | Role |
|---|---|---:|---|---|
| Disturbance trigger angle | θ_cross | 10° = 0.1745 rad | rad | Activates when arm passes |
| Disturbance magnitude | τ_d | 25.0 | N·m | Applied by simulation; known experimental input |
| Simulation step | Δt | 1/24 | s | `DT = 1.0 / FPS` |
| Total steps | — | 144 | — | `6 × FPS` |

---

## 6. Quantity inventory by observable family

### 6.1 Electrical family

| Quantity | Symbol | Units | Source | Role classification |
|---|---|---|---|---|
| Commanded voltage | v_cmd | V | `c_projection(sigma_v, theta)` | **Controller output** |
| Applied voltage | v_applied | V | Not independently represented | **Not observable** in this simulation |
| Winding current | I | A | Not independently sensed | **Derived plant quantity** (inferred as v_cmd / R_W) |

**Provenance notes:**

`v_cmd` is the output of the C projection:
```
frac = clamp((THETA_TARGET - theta) / THETA_TARGET, -1.0, 1.0)
v_cmd = clamp(sigma_v + frac * V_RAISE, -V_BUS, V_BUS)
```
This is a controller output, not an independent sensor reading.

`v_applied` is not separately represented. The simulation applies `v_cmd` directly to
the plant via `tau_cmd_a = K_T * (v_cmd_a / R_W)`. A more complete motor simulation
would distinguish commanded from applied voltage through electrical transient state.
In this simulation, v_applied = v_cmd by construction of the simplified plant.

`I` (winding current) is not independently sensed. It is inferred as `v_cmd / R_W`
wherever it appears. The simulation does not model a current sensor.

### 6.2 Mechanical effort family

| Quantity | Symbol | Units | Source | Role classification |
|---|---|---|---|---|
| Motor torque | τ_m | N·m | `K_T × (v_cmd / R_W)` | **Derived plant quantity** from command |
| Load-side torque | τ_s | N·m | `integrate_plant` return value | **Simulated sensor observation** (O_t) |
| External disturbance | τ_d | N·m | Scenario parameter `DIST_MAG` | **Known experimental input** — not a controller observation |
| Net joint torque | τ_net | N·m | Calculated inside `integrate_plant` | **Derived plant quantity** — not observable from outside plant |

**Provenance notes (critical):**

`τ_m` (motor torque) is computed as:
```python
tau_cmd_a = K_T * (v_cmd_a / R_W)
```
This is derived from the command. It is not independently measured.

`τ_s` (load-side torque) is computed inside `integrate_plant` as:
```python
tau_sensed = MASS * G * ARM * math.sin(theta) + tau_disturbance
```
**The load-side sensor reports gravitational loading plus disturbance. It does not
report motor torque.** Motor torque and load-side torque are on opposite sides of
the mechanical balance. The sensor does not observe the motor's contribution.

`τ_net` is computed entirely inside `integrate_plant`:
```python
tau_net = tau_cmd - MASS * G * ARM * math.sin(theta) - DAMPING * omega - tau_disturbance
```
It is not returned and not observable to the controller.

`τ_d` is known by scenario construction. It is not a sensor reading arriving at the
controller through an observation channel. The controller's only channel for detecting
disturbance is through its effect on `τ_s`.

### 6.3 Mechanical motion family

| Quantity | Symbol | Units | Source | Role classification |
|---|---|---|---|---|
| Joint angle | θ | rad | Plant state returned by `integrate_plant` | **Simulated encoder observation** (O_t) |
| Angular velocity | ω | rad/s | Plant state returned by `integrate_plant` | **Available simulated state** — not currently a separate sensor; not fed to ABR kernel |
| Angular acceleration | α | rad/s² | `tau_net / I_EFF` inside plant | **Derived plant quantity** — not observable from outside plant |

**Provenance notes:**

`θ` (joint angle) is returned directly from `integrate_plant` as `clamped` (the new
post-integration angle, clamped to [0, π/2]). It enters the C projection and the
admission check indirectly. It is the encoder-equivalent observation.

`ω` (angular velocity) is returned from `integrate_plant` as `new_omega`. It is available
as a simulated state but is not currently admitted into the ABR operator chain (A, B, R,
Sigma). The conventional controller uses `-omega` in its Kd damping term.

`α` (angular acceleration) is computed inside `integrate_plant` and not returned.
It is not observable.

---

## 7. Admission boundary

The M admission check (from `KernelV8.step`):
```python
if abs(tau_s) > TAU_ELEC_MAX:
    return 0.0, 0.0, 0.0, 0.0, 0.0, [], False
```

The declared electrical extent boundary is `τ_elec_max = 68.571 N·m`.
The admitted quantity is `τ_s` (load-side torque). If `|τ_s| > τ_elec_max`, the
observation is rejected and the step produces no operator output.

At the 25 N·m disturbance: peak `τ_s` = 29.4675 N·m. Well within the admitted range.

---

## 8. What enters the ABR operator chain

The operator chain `O_t → M → A → B → [R] → Sigma → C → u_t` receives:

| Step input | Quantity | Value at disturbance peak (frame 5) | Units |
|---|---|---|---|
| O_t to A | τ_s (current) | 29.4675 | N·m |
| A kernel state | τ_s (prior, t−1) | 2.5363 | N·m |
| C input 1 | sigma_v (accumulated) | 1.7754 (before frame 5 addition) | V |
| C input 2 | θ (current angle) | 22.0642° (from prior plant step) | deg |

The actuator command `v_cmd` produced at each step enters the plant as:
```python
tau_cmd_a = K_T * (v_cmd_a / R_W)
```
which returns through `integrate_plant` to produce the next `τ_s`.

---

## 9. Quantities not admitted into the ABR operator chain

The following quantities are present in the simulation but are not admitted into the
ABR operator chain (A, B, R, Sigma) in V0.5.2:

- `v_applied` — not independently represented
- `I` (winding current) — not independently sensed
- `τ_m` (motor torque) — derived from command, not observed
- `τ_net` (net torque) — internal to plant
- `τ_d` (disturbance) — known scenario input, not a controller observation
- `ω` (angular velocity) — available but not admitted
- `α` (acceleration) — internal to plant

This inventory is observational. Whether any of these quantities belongs in the
relational declaration is determined by Document 2 (`motor_sensor_actuator_relational_declaration.md`).

---

## 10. Simulation step timing

Each frame:
1. `tau_s_in = a_tau_s` — capture current sensor reading (O_t from prior plant step)
2. `kernel.step(tau_s_in, ...)` — runs A → B → [R: OPEN] → Sigma
3. `v_cmd_a = c_projection(sigma_v, a_theta)` — C projection
4. `tau_cmd_a = K_T * (v_cmd_a / R_W)` — convert command to motor torque
5. `a_theta, a_omega, a_tau_s = integrate_plant(...)` — advance plant, produce next O_t

The sensor reading `τ_s` used at step t was produced by the plant at step t−1.
There is a one-step lag between plant state and controller observation.

---

*This document establishes the observational ground truth for the relational declaration
(Document 2) and the operator trace (Document 3). No computational elements are proposed here.*
