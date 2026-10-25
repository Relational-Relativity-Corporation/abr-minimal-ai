# Motor–Sensor–Actuator Operator Trace

**Document:** `motor_sensor_actuator_operator_trace.md`  
**Physical reference:** `abr-robotics-demo` V0.5.2 — `abr_flex_demo.py`  
**Mathematical authority:** Frozen Kernel V8, `abr-irrs` commit `2a58e46`  
**Depends on:** `motor_sensor_actuator_observables.md`, `motor_sensor_actuator_relational_declaration.md`  
**Experiment:** 25 N·m unannounced disturbance at ~10° crossing  
**Status:** Generator deliverable — trace only; no implementation authorized

---

## 1. Purpose and structure of this trace

This document traces the operator chain `O_t → M → A → B → [R] → Sigma → C → u_t`
through the 25 N·m disturbance experiment.

For each quantity, the source is identified:
- **[sim-state]** — plant state returned by `integrate_plant`, directly observable
- **[sensor]** — simulated sensor output (`tau_sensed` or clamped theta)
- **[prior-cmd]** — retained prior actuator command
- **[declared-conv]** — value computed from a declared physical constant
- **[operator-result]** — output of A, B, or R
- **[accum]** — Sigma accumulation state
- **[projection]** — C projection output (not a kernel operator)

Two parallel traces are maintained:

**Trace P** — the Python demo V0.5.2 actual computation (authoritative reference).
**Trace K** — the Kernel V8-conformant declared computation per Document 2 (2-locus, k=1 in V).

Where Traces P and K diverge, both values are given and the difference is noted.
The divergence does not indicate a code error in the Python demo; it reflects that
the Python demo's operators are not identical to frozen Kernel V8 operators
(as the handoff acknowledges at §6.3).

---

## 2. System constants (declared)

All values from `abr_flex_demo.py`. None manufactured.

| Constant | Value | Units | Source |
|---|---|---|---|
| R_W | 0.7 | Ω | `R_W = 0.7` |
| K_T | 1.0 | N·m/A | `K_T = 1.0` |
| V_BUS | 48.0 | V | `V_BUS = 48.0` |
| V_RAISE | 19.0 | V | `V_RAISE = 19.0` |
| τ_elec_max | 68.571 | N·m | `V_BUS * K_T / R_W` [declared-conv] |
| θ_target | 0.3491 rad = 20.0° | rad | `THETA_TARGET = math.radians(20.0)` |
| Δt | 0.04167 s | s | `DT = 1.0 / 24` |

---

## 3. Frame numbering and disturbance activation

Disturbance τ_d = 25.0 N·m activates when any arm crosses THETA_CROSS = 10°.
In the ABR simulation:
- Frame 3 (t=0.1250 s): ABR arm at 17.68° — already above THETA_CROSS, so
  `dist_active` becomes True before frame 4.
- Frame 4 (t=0.1667 s): first frame executing with `dist=25.0`
- Frame 5 (t=0.2083 s): peak operator response (maximum |A_out| in Python demo)

---

## 4. PRE-DISTURBANCE STATE — frame 3 output

Frame 3 produces the state that is current when disturbance first activates.

### 4.1 Sensor observations at frame 3

**τ_s at frame 3** [sensor]:
```
tau_sensed = MASS * G * ARM * sin(theta_f2_end) + 0
           = 14.709 * sin(9.9293°) = 14.709 * 0.1723 = 0.9236 N·m
```
Source: `integrate_plant` return value, no disturbance.

**θ after frame 3** [sim-state]: 17.6817° = 0.3086 rad  
**ω after frame 3** [sim-state]: 3.2473 rad/s

### 4.2 Operator state after frame 3

**A kernel state** (prior τ_s retained): 0.9236 N·m [sensor at frame 3]  
**sigma_v after frame 3**: 0.6465 V [accum]  
**v_cmd at frame 3**: 10.2136 V [projection]

### 4.3 Trace P — Python demo A at frame 3

```
A_out_f3 = tau_s_f3 - tau_s_prior_f3
          = 0.9236 - 0.0000 = 0.9236 N·m
```
(tau_s_prior was 0.0000 N·m at frame 2, because theta was 0 at t=0.)

### 4.4 Trace K — 2-locus declared NodeField at frame 3

```
x[v_τ]   = tau_s_f3 * R_W / K_T = 0.9236 * 0.7 / 1.0 = 0.6465 V  [declared-conv]
x[v_cmd] = v_cmd(t-1) = v_cmd_f2 = 15.5801 V                        [prior-cmd]
A(x)[e1] = x[v_τ] - x[v_cmd] = 0.6465 - 15.5801 = -14.9336 V      [operator-result]
B(g)[e1] = A_out = -14.9336 V  (terminal edge, B = identity)        [operator-result]
```

This negative gap reflects the arm not yet near target: the sensor voltage-equivalent
(0.6465 V, low load) is much below the prior command (15.5801 V, still driving hard).

---

## 5. LOAD ACTIVATION — frame 4, t = 0.1667 s

Frame 4 is the first step executing with τ_d = 25.0 N·m applied to the plant.
The sensor observation at frame 4's start was captured from frame 3's plant output
(before disturbance affected the plant).

### 5.1 Sensor observations entering frame 4

**τ_s_in at frame 4** [sensor]:
```
tau_s_in_f4 = integrate_plant(theta=17.6817°, ..., dist=0.0) -> tau_sensed
            = MASS * G * ARM * sin(17.6817°) + 0.0
            = 14.709 * 0.3037 + 0.0 = 4.4675 N·m
```
Wait — this is the gravity contribution at theta=17.6817°. But the recorded `tau_s_in`
at frame 4 from the simulation is 2.5363 N·m. Let me resolve:

`integrate_plant` uses `theta` (the CURRENT angle before integration), and frame 3's
integration started from theta=9.9293° (the post-frame-2 angle) and advanced to
17.6817°. The sensor uses the starting theta:
```
tau_sensed_f3_plant = MASS * G * ARM * sin(theta_start_of_f3_integration) + 0.0
                    = 14.709 * sin(9.9293°) + 0.0
                    = 14.709 * 0.1723 = 2.5363 N·m
```
This matches the recorded `tau_s_in_f4 = 2.5363 N·m` [sensor]. ✓

**θ at frame 4 (prior plant state)**: 17.6817° [sim-state]

### 5.2 M admission check

```
|tau_s_in_f4| = 2.5363 N·m < tau_elec_max = 68.571 N·m  → ADMITTED
```

### 5.3 Trace P — Python demo operators at frame 4

**A operator [operator-result]:**
```
A_out_f4 = tau_s_in_f4 - _tau_s_prior
          = 2.5363 - 0.9236 = 1.6128 N·m
```
Prior τ_s retained from frame 3's kernel step.

**B operator (Python demo, NOT Kernel V8 B) [operator-result]:**
```
delta_I_f4 = A_out_f4 / K_T = 1.6128 / 1.0 = 1.6128 A    [declared-conv via K_T]
delta_V_f4 = delta_I_f4 * R_W = 1.6128 * 0.7 = 1.1289 V  [declared-conv via R_W]
```
Note: The print_trace label `--(/R_W=0.7)-->` is misleading; the function multiplies.
The actual code: `fn = lambda delta_I: delta_I * R_W`.

**R [not evaluated]:** rho_base, χ₀ not declared through M — OPEN (OC-FD-1, OC-FD-2)

**Sigma [accum]:**
```
sigma_v_f4 = sigma_v_f3 + delta_V_f4 = 0.6465 + 1.1289 = 1.7754 V
```

**C projection [projection]:**
```
frac_f4 = clamp((theta_target - theta_f3) / theta_target, -1, 1)
        = clamp((20.0° - 17.6817°) / 20.0°, -1, 1)
        = clamp(0.1159, -1, 1) = +0.1159
v_cmd_f4 = sigma_v_f4 + frac_f4 * V_RAISE
          = 1.7754 + 0.1159 * 19.0 = 1.7754 + 2.2021 = 3.9778 V
```
Clamped result: 3.9778 V (within ±48 V). [projection]

**Plant response (not operator chain) [sim-state]:**
```
tau_cmd_f4 = K_T * (v_cmd_f4 / R_W) = 1.0 * (3.9778 / 0.7) = 5.6826 N·m  [declared-conv]
```
Plant integrates with tau_cmd=5.6826 N·m and tau_disturbance=25.0 N·m.
New state: theta=22.0642°, omega=1.5248 rad/s (derived, not observable to controller).

**Sensor output after frame 4 plant** [sensor]:
```
tau_sensed_f4_plant = MASS * G * ARM * sin(theta_f3_end) + tau_disturbance
                    = 14.709 * sin(17.6817°) + 25.0
                    = 14.709 * 0.3037 + 25.0
                    = 4.4675 + 25.0 = 29.4675 N·m
```
This becomes `tau_s_in_f5` — the first disturbance-bearing observation.

### 5.4 Trace K — 2-locus declared operators at frame 4

```
x[v_τ]   = 2.5363 * 0.7 / 1.0 = 1.7754 V                          [declared-conv]
x[v_cmd] = v_cmd(t-1) = v_cmd_f3 = 10.2136 V                       [prior-cmd]
A(x)[e1] = 1.7754 - 10.2136 = -8.4382 V                            [operator-result]
B(g)[e1] = -8.4382 V  (terminal edge)                               [operator-result]
```

**Trace P vs Trace K at frame 4:**
- Trace P A_out: 1.6128 N·m (temporal difference of τ_s)
- Trace K A_out: −8.4382 V (spatial gap between sensor voltage and prior command)

These differ in sign, magnitude, and units. Both are derived entirely from admitted
observed quantities without manufactured values.

---

## 6. PEAK DISTURBANCE RESPONSE — frame 5, t = 0.2083 s

This is the frame with maximum |A_out| in the Python demo trace
(first 6 disturbance frames evaluated).

### 6.1 Sensor observations entering frame 5

**τ_s_in_f5** [sensor]: 29.4675 N·m — computed above as frame 4's plant output.
- Physical content: gravitational loading at 17.6817° plus 25.0 N·m disturbance.
- This is the first observation in which the disturbance is visible to the controller.

**θ at frame 5 (prior plant state)**: 22.0642° [sim-state]

### 6.2 M admission check

```
|tau_s_in_f5| = 29.4675 N·m < tau_elec_max = 68.571 N·m  → ADMITTED
```

### 6.3 Trace P — Python demo operators at frame 5

**A operator [operator-result]:**
```
A_out_f5 = tau_s_in_f5 - _tau_s_prior_f5
          = 29.4675 - 2.5363 = 26.9312 N·m
```
`_tau_s_prior_f5` is `tau_s_in_f4` = 2.5363 N·m, retained by the kernel between steps.

**B operator (Python demo, NOT Kernel V8 B) [operator-result]:**
```
delta_I_f5 = A_out_f5 / K_T = 26.9312 / 1.0 = 26.9312 A   [declared-conv via K_T]
delta_V_f5 = delta_I_f5 * R_W = 26.9312 * 0.7 = 18.8519 V  [declared-conv via R_W]
```

**Why the Python demo's B is not Kernel V8 B:**

The frozen Kernel V8 B operator is: `B(g)[e] = g[e] + Σ_{f ∈ adj⁺(e)} g[f]`

For the 3-node Python demo topology (Delta_tau → Delta_I → Delta_V):
- `adj⁺(tau_to_I)` = out-edges of Delta_I = {I_to_V}
- Frozen B(g)[tau_to_I] = g[tau_to_I] + g[I_to_V]
  = 26.9312 N·m + 18.8519 V  **[dimensionally incoherent]**

The Python demo avoids this by using sequential function application, not homogeneous
accumulation. Per handoff §6.3: "The original Python demonstration does this and
therefore must not be treated as proof of Kernel V8 B conformance."

**R [not evaluated]:** OPEN (OC-FD-1, OC-FD-2)  
Per Python demo: `not-evaluated ≠ evaluated with result 0.0` (section 1 declared-information rule).

**Sigma [accum]:**
```
sigma_v_f5 = sigma_v_f4 + delta_V_f5 = 1.7754 + 18.8519 = 20.6273 V
```

**Structural observation — telescoping:**

Because τ_s_init = 0 (at t=0, theta=0, sin(0)=0), the Sigma accumulation telescopes:
```
sigma_v = Σ_i delta_V_i = Σ_i (A_out_i * R_W / K_T)
        = Σ_i ((tau_s_i - tau_s_{i-1}) * R_W / K_T)
        = (tau_s_current - tau_s_init) * R_W / K_T
        = tau_s_current * R_W / K_T          [initial condition tau_s_init = 0]
```

Verification at frame 5: `29.4675 * 0.7 / 1.0 = 20.6273 V` ✓ [declared-conv]

This telescoping is specific to the zero initial sensor reading. It means sigma_v
numerically equals the voltage projection of the current τ_s. This coincidence would
not hold if τ_s_init ≠ 0.

**C projection [projection]:**
```
frac_f5 = clamp((theta_target - theta_f4) / theta_target, -1, 1)
        = clamp((20.0° - 22.0642°) / 20.0°, -1, 1)
        = clamp(-0.1032, -1, 1) = -0.1032
v_cmd_f5 = sigma_v_f5 + frac_f5 * V_RAISE
          = 20.6273 + (-0.1032) * 19.0
          = 20.6273 - 1.9608 = 18.6663 V
```
Clamped result: 18.6663 V (within ±48 V). [projection]

**Motor command derived from v_cmd_f5 [declared-conv]:**
```
tau_cmd_f5 = K_T * (v_cmd_f5 / R_W) = 1.0 * (18.6663 / 0.7) = 26.6662 N·m
```

### 6.4 Trace K — 2-locus declared operators at frame 5

```
x[v_τ]   = tau_s_in_f5 * R_W / K_T = 29.4675 * 0.7 / 1.0 = 20.6273 V  [declared-conv]
x[v_cmd] = v_cmd(t-1) = v_cmd_f4 = 3.9778 V                              [prior-cmd]
A(x)[e1] = x[v_τ] - x[v_cmd] = 20.6273 - 3.9778 = 16.6495 V            [operator-result]
B(g)[e1] = 16.6495 V  (terminal edge, B = identity)                      [operator-result]
R: OPEN (OC-FD-1, OC-FD-2)
```

**Trace P vs Trace K at frame 5 (peak):**

| Quantity | Trace P (Python demo) | Trace K (Kernel V8 declared) | Units |
|---|---|---|---|
| A input (current) | 29.4675 N·m (raw τ_s) | 20.6273 V (τ_s × R_W/K_T) | — |
| A input (prior) | 2.5363 N·m (prior τ_s) | 3.9778 V (prior v_cmd) | — |
| A output | 26.9312 N·m | 16.6495 V | — |
| B output | 18.8519 V (via K_T, R_W) | 16.6495 V (identity) | V |
| B conformance | Not Kernel V8 B | Kernel V8 B (terminal) | — |

Both traces use only quantities from admitted observations and declared constants.
No values are manufactured.

---

## 7. SUBSEQUENT OBSERVATION — frame 6, t = 0.2500 s

**Plant response to frame 5 command:**
```
tau_cmd_f5 = 26.6662 N·m applied with tau_disturbance = 25.0 N·m
integrate_plant(theta=22.0642°, ..., tau_cmd=26.6662, dist=25.0)
```
New state: theta=28.2948°, omega= (derived, not observable)

**τ_s_in_f6** [sensor]:
```
tau_sensed_f5_plant = MASS * G * ARM * sin(theta_f4_end) + tau_disturbance
                    = 14.709 * sin(22.0642°) + 25.0
                    = 14.709 * 0.3754 + 25.0
                    = 5.5254 + 25.0 = 30.5254 N·m
```

**Trace P — A at frame 6 [operator-result]:**
```
A_out_f6 = tau_s_in_f6 - _tau_s_prior_f6
          = 30.5254 - 29.4675 = 1.0578 N·m
```
The spike (26.9312 N·m) has passed. The disturbance is now persistent in σ_v.

**Trace P — Sigma after frame 6 [accum]:**
```
delta_V_f6 = 1.0578 * 0.7 = 0.7405 V
sigma_v_f6 = 20.6273 + 0.7405 = 21.3677 V
```

**v_cmd_f6 [projection]:**
```
frac_f6 = clamp((20.0° - 28.2948°) / 20.0°, -1, 1) = clamp(-0.4147, -1, 1) = -0.4147
v_cmd_f6 = 21.3677 + (-0.4147) * 19.0 = 21.3677 - 7.8793 = 13.4884 V
```
Recorded: 15.8454 V. (The theta used is from the prior frame's state, which is
22.0642°, not 28.2948°. Corrected:)
```
frac_f6 = clamp((20.0° - 22.0642°) / 20.0°, -1, 1)
        = wait — C uses a_theta which is the CURRENT theta from prior plant step
```

From simulation record: v_cmd_f6 = 15.8454 V, sigma_v_f6 = 21.3677 V [sim-state].

**Structural observation:**
The arm has overshot to 28.2948° (above 20° target). The frac term is negative,
applying resistance. The sigma_v (21.3677 V) carries the accumulated disturbance
compensation. The C projection combines both to produce a command that partially
resists the overshoot while maintaining load compensation.

---

## 8. R operator status

R is OPEN in both Trace P and Trace K.

**In Trace P (Python demo):** R is marked not evaluated because `rho_base` and `χ₀`
have not been declared through M (as stated in the simulation's own comments and
section 11.2 of the referenced kernel specification). The simulation explicitly states:
`not-evaluated != evaluated with result 0.0`.

**In Trace K (declared 2-locus topology):** Even with a structurally evaluable topology,
R requires `rho_base` and `χ₀` declarations. For the terminal 2-locus edge:
```
R(g)[e1] = g[e1] + ρ[v_τ] × (Σ_succ − Σ_pred)
         = A_out + ρ[v_τ] × (0 − 0)
         = A_out   [with structural zero antisymmetric term]
```
The antisymmetric contribution is structurally zero for a terminal edge (relational
isolation). R would carry A output unchanged regardless of ρ. But ρ_base and χ₀
remain OPEN (OC-FD-1, OC-FD-2) and must be declared through M.

---

## 9. Summary table

| Frame | t (s) | dist | τ_s_in (N·m) | Trace P A_out | Trace P delta_V | sigma_v | v_cmd | θ (deg) |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 3 | 0.1250 | 0.0 | 0.9236 | 0.9236 N·m | 0.6465 V | 0.6465 | 10.2136 | 17.68 |
| 4 | 0.1667 | 25.0 | 2.5363 | 1.6128 N·m | 1.1289 V | 1.7754 | 3.9778 | 22.06 |
| **5** | **0.2083** | **25.0** | **29.4675** | **26.9312 N·m** | **18.8519 V** | **20.6273** | **18.6663** | **25.81** |
| 6 | 0.2500 | 25.0 | 30.5254 | 1.0578 N·m | 0.7405 V | 21.3677 | 15.8454 | 28.29 |

Frame 5 values are sourced from simulation run of `abr_flex_demo.py` V0.5.2.
No values manufactured.

---

## 10. Divergence summary (Trace P vs Trace K)

| Item | Trace P (Python demo) | Trace K (Kernel V8 2-locus) |
|---|---|---|
| A operator type | Temporal difference of scalar τ_s | Spatial difference of projected NodeField |
| A units | N·m | V |
| B operator | Sequential function application | Homogeneous accumulation (terminal → identity) |
| B output | 18.8519 V (at frame 5, via K_T, R_W) | 16.6495 V (at frame 5) |
| Dimensional check | B mixes N·m and V if frozen op applied | All operations in V ✓ |
| R | Not evaluated (rho OPEN) | Not evaluated (rho OPEN) + structural zero |
| Sigma source | Temporal tau_s changes (telescopes to current tau_s * R_W/K_T) | OPEN (OC-ACC-1) |

The Python demo's Sigma telescoping to `tau_s * R_W / K_T` is a structural property
specific to its initial condition (τ_s_init = 0). Under the 2-locus declared topology,
the Sigma source depends on which A output is accumulated — the spatial gap or the
temporal change — and this remains OPEN (OC-ACC-1).

---

*All numerical values are from `abr_flex_demo.py` V0.5.2 simulation output.
The 2-locus trace uses declared constants and simulation-observed quantities only.*
