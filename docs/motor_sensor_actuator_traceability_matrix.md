# Motor–Sensor–Actuator Mathematical-to-Observable Traceability Matrix

**Document:** `motor_sensor_actuator_traceability_matrix.md`
**Physical reference:** `abr-robotics-demo` V0.5.2 — `abr_flex_demo.py`
**Mathematical authority:** Frozen Kernel V8, `abr-irrs` commit `2a58e46`
**Depends on:** Documents 1–4 in this set
**Status:** Generator deliverable — traceability record only; no implementation authorized

---

## 1. Governing requirement

The Origin/Verifier correction establishes:

> "Our governing requirement is complete traceability:
> Observable relationship → Measurement → Declaration D/M → Frozen operators A/B/R →
> Calculated result → Physical response → Subsequent measurement and comparison."

For every mathematical quantity, the matrix must identify:

1. The measurement or declared derived quantity from which it originates
2. Its physical units, provenance, and relational role
3. The exact operator and mathematical operation that use it
4. The resulting calculated quantity and its physical interpretation
5. The subsequent measurement against which the result or its physical consequence is evaluated
6. Any residual between calculated and observed quantities, where the comparison is dimensionally and physically defined

Additional governing constraints from the correction:
- The calculation is not complete merely because A, B, and R have executed.
- A declaration is not complete merely because it produces a dimensionally coherent operator input.
- The calculation must return to the observable process.
- Preserve the distinction between physical relationships, measured quantities, derived quantities, operator outputs, and predictions.

---

## 2. Traceability chain positions

The complete chain from observable to residual:

| Position | Designation | Type |
|---|---|---|
| 1 | Observable relationship | Physical relationship being tracked |
| 2 | Measurement | Instrument-returned or simulator-returned quantity |
| 3 | M admission | Boundary gate |
| 4 | D/M — NodeField entry | Declared derived quantity |
| 5 | A operator | Operator output |
| 6 | B operator | Operator output |
| 7 | R operator | Operator output (OPEN) |
| 8 | Sigma | Accumulated state |
| 9 | C projection | Declared projection output |
| 10 | Physical consequence | Prediction — derived from operator output |
| 11 | Plant response | Internal dynamics (not observable) |
| 12 | Subsequent measurement | Next-step sensor return |
| 13 | Residual | Gap between prediction and observation |

Positions 1–9 are within the controller. Position 10 is the prediction. Positions 11–12 are within the plant and sensor. Position 13 closes the loop.

This document additionally distinguishes:

- **Trace P** — the calculation as implemented in `abr_flex_demo.py` V0.5.2 (temporal-difference A, sequential chain B)
- **Trace K** — the Kernel V8-conformant 2-locus declaration (spatial-gap A, terminal-edge identity B)

Both traces produce the same Sigma and v_cmd in the zero-initial-condition simulation
(due to the telescoping property established in Document 3). Trace K A values shown
here are computed analytically from the simulation's x[v_τ] and x[v_cmd] inputs.
If Trace K were running as an implemented kernel, sigma_v and subsequent v_cmd would
differ unless the telescoping property holds throughout.

---

## 3. Quantity classification

Every mathematical quantity in the declared calculation is assigned to one of five categories:

| Category | Definition | Examples in this system |
|---|---|---|
| Physical relationship | The underlying coupling that makes the calculation meaningful | Closed loop: sensor observes → command applies → plant responds; K_T: torque↔current; R_W: current↔voltage |
| Measured quantity | Returned by an instrument or simulator; not derived from other declared quantities in the controller | τ_s [N·m], θ [rad] |
| Declared derived quantity | Computed from measured quantities and declared constants using a specified mapping | τ_s × R_W/K_T [V], tau_cmd = K_T × v_cmd/R_W [N·m] |
| Operator output | Produced by a Frozen Kernel V8 operator (A, B, R) or declared accumulation (Sigma, C) | A_K [V], A_P [N·m], B_P delta_V [V], sigma_v [V], v_cmd [V] |
| Prediction | The physical consequence the operator output is interpreted as producing | tau_cmd [N·m] → expected plant balance |

---

## 4. Six-column per-quantity matrix

For each admitted quantity, the six-column schema is applied below.

---

### 4.1 τ_s(t) — load-side torque sensor reading

| Column | Content |
|---|---|
| **1. Measurement origin** | Returned by `integrate_plant` as `tau_sensed = MASS * G * ARM * math.sin(theta) + tau_disturbance`. This is a simulated sensor observation of the load-side torque. It is **not** motor torque. |
| **2. Units, provenance, relational role** | [N·m]. Source: plant simulator (simulated encoder). Role: primary observable; only channel through which gravitational load and external disturbance are visible to the controller. |
| **3. Operator and operation** | M admission gate: `if abs(tau_s) > 68.571: reject`. If admitted: enters NodeField as projected value x[v_τ] = τ_s × R_W/K_T and as A_P temporal reference. |
| **4. Resulting calculated quantity** | Trace P: feeds temporal difference A_P = τ_s(t) − τ_s(t−1). Trace K: feeds spatial gap A_K = τ_s × R_W/K_T − v_cmd(t−1). |
| **5. Subsequent measurement** | τ_s(t+1): returned from `integrate_plant` after the plant responds to the command produced from this step's operator output. This is the subsequent observation that evaluates whether the calculation produced an adequate response. |
| **6. Residual** | τ_s(t+1) × R_W/K_T − v_cmd(t) [V]. Equivalently: τ_s(t+1) − tau_cmd(t) [N·m]. Positive residual: observed load exceeds what was commanded. Negative residual: command exceeded observed load (arm accelerating or overshot). See §5. |

---

### 4.2 x[v_τ] = τ_s × R_W/K_T — declared dimensional projection at v_τ locus

| Column | Content |
|---|---|
| **1. Measurement origin** | Derived from measured τ_s [N·m] and declared constants R_W = 0.7 Ω, K_T = 1.0 N·m/A. Not independently measured. |
| **2. Units, provenance, relational role** | [V]. Physical interpretation: the voltage equivalent of the observed load torque — the motor voltage that would produce a balancing torque. Role: source-locus value in NodeField for Kernel V8 A (Trace K). |
| **3. Operator and operation** | Trace K: A(x)[e_1] = x[v_τ] − x[v_cmd]. Trace P: this projection is applied after A, inside the B chain: delta_I = A_P / K_T, delta_V = delta_I × R_W (equivalent to A_P × R_W/K_T). |
| **4. Resulting calculated quantity** | Trace K: enters A as one pole of the relational gap, producing g[e_1] [V]. Trace P: A_P × R_W/K_T = B_P output delta_V [V], which enters Sigma. |
| **5. Subsequent measurement** | τ_s(t+1) × R_W/K_T: the projection of the next sensor reading. This is compared against v_cmd(t) to form the residual. |
| **6. Residual** | τ_s(t+1) × R_W/K_T − v_cmd(t). As noted in §3: in Trace K, this residual equals A_K at step t+1. |

---

### 4.3 x[v_cmd] = v_cmd(t−1) — prior actuator command at v_cmd locus

| Column | Content |
|---|---|
| **1. Measurement origin** | Retained from the prior step's C projection output by PersistenceState. Not an independent sensor reading. |
| **2. Units, provenance, relational role** | [V]. Source: prior controller output. Role: target-locus value in NodeField. Represents the actuator's last-commanded state — the relational counterpart of the sensor observation. |
| **3. Operator and operation** | Trace K: A(x)[e_1] = x[v_τ] − x[v_cmd]. Used as the subtracted term; it is the reference against which the current observation is compared. Trace P: not explicitly used in A; v_cmd_prior is used only in C projection. |
| **4. Resulting calculated quantity** | Trace K: its subtraction from x[v_τ] produces the relational gap A_K [V]. Trace P: it does not enter A_P. |
| **5. Subsequent measurement** | v_cmd(t) replaces x[v_cmd] at the next step. The next step's A_K uses v_cmd(t) as the reference. |
| **6. Residual** | Not a residual quantity itself. It participates in the residual: residual = τ_s(t+1) × R_W/K_T − v_cmd(t). |

---

### 4.4 A operator output — relational difference

#### Trace K: A_K(x)[e_1] = x[v_τ] − x[v_cmd] [V]

| Column | Content |
|---|---|
| **1. Measurement origin** | Computed from x[v_τ] (derived from τ_s) and x[v_cmd] (retained prior command). Both are in Volts. |
| **2. Units, provenance, relational role** | [V]. Physical interpretation: the voltage gap between the observed load's equivalent voltage and the prior actuator command. A positive gap means the load exceeds the command; negative means the command exceeded the load. |
| **3. Operator and operation** | Frozen Kernel V8: `A(x)[e] = x[src(e)] − x[tgt(e)]`. For e_1: src = v_τ, tgt = v_cmd. Pure directed difference; no additional computation. |
| **4. Resulting calculated quantity** | g[e_1] [V] — the edge field after A. Enters B. For the 2-locus terminal-edge topology, B = identity, so B output = A_K [V]. |
| **5. Subsequent measurement** | At step t+1: A_K = τ_s(t+1) × R_W/K_T − v_cmd(t). The subsequent measurement τ_s(t+1) directly determines the next A_K input. |
| **6. Residual** | A_K at step t+1 IS the prediction-vs-observation residual from step t's command. See §5 for the proof and its significance. |

#### Trace P: A_P = τ_s(t) − τ_s(t−1) [N·m]

| Column | Content |
|---|---|
| **1. Measurement origin** | Computed from current τ_s and retained prior τ_s value. Both are sensor readings [N·m]. |
| **2. Units, provenance, relational role** | [N·m]. Physical interpretation: the step-to-step change in observed load torque. A positive value means the load increased; negative means it decreased. |
| **3. Operator and operation** | Python demo implementation: `a_out = tau_s - self._tau_s_prior`. This is a temporal difference, not the Frozen Kernel V8 A spatial-difference operator. |
| **4. Resulting calculated quantity** | A_P [N·m] — enters the sequential chain: delta_I = A_P / K_T [A], delta_V = delta_I × R_W [V]. The final chain output delta_V = A_P × R_W/K_T [V] enters Sigma. |
| **5. Subsequent measurement** | At step t+1: A_P = τ_s(t+1) − τ_s(t). The subsequent measurement determines the temporal change. |
| **6. Residual** | A_P at step t+1 is NOT the prediction-observation residual. It is the sensor change. The difference between A_P and the true residual (A_K) equals the positional authority term frac(t) × V_RAISE from the prior C projection. See §6. |

---

### 4.5 B operator output

#### Trace K: B(g)[e_1] = g[e_1] (terminal edge identity)

| Column | Content |
|---|---|
| **1. Measurement origin** | Derived from A_K [V]. No new measurement enters. |
| **2. Units, provenance, relational role** | [V]. Same as A_K. The B operator adds no additional relational structure because `adj⁺(e_1) = ∅` (terminal edge — v_cmd has no outgoing edges). |
| **3. Operator and operation** | Frozen Kernel V8: `B(g)[e] = g[e] + Σ_{f ∈ adj⁺(e)} g[f]`. For terminal e_1: sum is empty, B = identity. |
| **4. Resulting calculated quantity** | B_K = A_K [V]. Enters Sigma. No accumulation of successor edge contributions. |
| **5. Subsequent measurement** | Same as A_K: subsequent τ_s(t+1) provides the next cycle's x[v_τ]. |
| **6. Residual** | Same as A_K residual. |

#### Trace P: B_P chain output = delta_V = A_P × R_W/K_T [V]

| Column | Content |
|---|---|
| **1. Measurement origin** | Derived from A_P [N·m] via sequential function application through declared topology. |
| **2. Units, provenance, relational role** | [V]. The sequential chain converts: Δτ [N·m] → ΔI = Δτ/K_T [A] → ΔV = ΔI × R_W [V]. This is a dimensional conversion chain, not Kernel V8 B. |
| **3. Operator and operation** | Python demo: `delta_I = a_out / K_T; delta_V = delta_I * R_W`. This is sequential function application through topology edges. As documented in Document 3 and Document 2 §7: Kernel V8 B applied to the 3-node topology produces dimensionally incoherent sums. |
| **4. Resulting calculated quantity** | delta_V [V]. Equivalent to A_P × R_W/K_T. This is the quantity Sigma accumulates. |
| **5. Subsequent measurement** | At step t+1: B_P = (τ_s(t+1) − τ_s(t)) × R_W/K_T. |
| **6. Residual** | Not the prediction-observation residual. See §6. |

---

### 4.6 sigma_v — accumulated Sigma state

| Column | Content |
|---|---|
| **1. Measurement origin** | Accumulated from B outputs across all prior steps. Carries relational history, not a single-step measurement. |
| **2. Units, provenance, relational role** | [V]. Source: declared Sigma accumulation. Role: carries the integrated relational response across steps. |
| **3. Operator and operation** | `sigma_v(t) = sigma_v(t−1) + B_out`. In the zero-initial-condition simulation: telescoping produces sigma_v(t) = τ_s(t) × R_W/K_T for all t (Trace P). This is established in Document 3 §4 and holds because `τ_s_init = 0`. |
| **4. Resulting calculated quantity** | sigma_v [V] — accumulated voltage representing total load history admitted by the operator chain. Enters C projection. |
| **5. Subsequent measurement** | At step t+1: sigma_v(t+1) = sigma_v(t) + B_out(t+1) = τ_s(t+1) × R_W/K_T (by telescoping). The subsequent sensor measurement τ_s(t+1) determines the next sigma_v state under the telescoping property. |
| **6. Residual** | sigma_v(t) = τ_s(t) × R_W/K_T (by telescoping). The gap sigma_v(t) − v_cmd(t) would be the overshoot of the accumulated state beyond the issued command — but this is also the Sigma state available for the next cycle. |

---

### 4.7 v_cmd(t) — actuator command output of C projection

| Column | Content |
|---|---|
| **1. Measurement origin** | Computed from sigma_v [V] (operator-accumulated) and θ [rad] (measured joint angle). Both inputs are at chain position 9. |
| **2. Units, provenance, relational role** | [V]. Source: `c_projection(sigma_v, theta)`. Role: this is the actuator command — the controller's output to the plant. It is not a kernel operator but a declared projection. Formula: `frac = clamp((θ_target − θ) / θ_target, −1, 1); v_cmd = clamp(sigma_v + frac × V_RAISE, −48, 48)`. |
| **3. Operator and operation** | Not A, B, or R. Declared C projection: combines load compensation (sigma_v) with positional authority (frac × V_RAISE). |
| **4. Resulting calculated quantity** | v_cmd [V]. This is the prediction: "the plant will receive this motor voltage command." |
| **5. Subsequent measurement** | v_cmd enters the plant as tau_cmd = K_T × v_cmd / R_W [N·m]. The plant integrates, producing new θ and ω. The plant then returns τ_s(t+1) = MASS × G × ARM × sin(θ_new) + τ_d. τ_s(t+1) is the subsequent measurement that evaluates v_cmd. |
| **6. Residual** | τ_s(t+1) × R_W/K_T − v_cmd(t) [V]. This is the primary closing residual of the entire chain. See §5. |

---

### 4.8 tau_cmd(t) — motor torque prediction

| Column | Content |
|---|---|
| **1. Measurement origin** | Derived from v_cmd and declared constants: `tau_cmd = K_T × v_cmd / R_W`. Not independently sensed. |
| **2. Units, provenance, relational role** | [N·m]. Category: prediction. Physical interpretation: the motor torque the electrical drive will apply to the mechanical joint. |
| **3. Operator and operation** | Not a kernel operator. Derived constant conversion: tau_cmd = 1.0 × v_cmd / 0.7 = v_cmd / 0.7. |
| **4. Resulting calculated quantity** | tau_cmd [N·m] — the predicted mechanical torque applied to the plant. Prediction: if tau_cmd > τ_s, the arm accelerates upward; if tau_cmd < τ_s, the arm decelerates. |
| **5. Subsequent measurement** | τ_s(t+1) is measured after the plant integrates under tau_cmd. The torque balance is: `tau_net = tau_cmd − gravity_torque − damping − tau_d`. τ_s(t+1) reflects the result of that balance. |
| **6. Residual** | τ_s(t+1) − tau_cmd(t) [N·m]. Positive: observed load exceeded predicted command. Negative: command exceeded observed load. This is the N·m form of the closing residual. |

---

### 4.9 τ_s(t+1) — subsequent sensor measurement

| Column | Content |
|---|---|
| **1. Measurement origin** | Returned by `integrate_plant` at the next step. |
| **2. Units, provenance, relational role** | [N·m]. Measured quantity. This is the closing measurement: it evaluates whether the prior step's v_cmd produced an adequate physical response. |
| **3. Operator and operation** | Becomes the input to the next step's M admission and A operator. Closes the loop. |
| **4. Resulting calculated quantity** | Enters the next step as τ_s [N·m], which is projected to x[v_τ] = τ_s × R_W/K_T and compared against x[v_cmd] = v_cmd(t). |
| **5. Subsequent measurement** | τ_s(t+1) is itself the subsequent measurement for step t. It is compared against the prediction (v_cmd(t) via tau_cmd) to produce the residual. |
| **6. Residual** | τ_s(t+1) × R_W/K_T − v_cmd(t) [V] — the residual defined at chain position 13 that this subsequent measurement participates in. |

---

## 5. Residual table — full disturbance experiment

The residual at each step: **τ_s(t+1) × R_W/K_T − v_cmd(t) [V]**

Equivalent in N·m: **τ_s(t+1) − tau_cmd(t) [N·m]**

This residual is computed using quantities from the actual Trace P simulation.

| Frame t | v_cmd(t) [V] | tau_cmd(t) [N·m] | τ_s(t+1) [N·m] | τ_s(t+1)×R_W/K_T [V] | Residual [V] | Residual [N·m] | Interpretation |
|---|---:|---:|---:|---:|---:|---:|---|
| f1 | 19.0000 | 27.1429 | 0.0000 | 0.0000 | −19.0000 | −27.1429 | Command far exceeds load; arm beginning to rise |
| f2 | 15.5801 | 22.2573 | 0.9236 | 0.6465 | −14.9336 | −21.3337 | Command still well above load; arm accelerating |
| f3 | 10.2136 | 14.5909 | 2.5363 | 1.7754 | −8.4382 | −12.0546 | Command still exceeds load; arm climbing |
| **f4** | **3.9778** | **5.6826** | **29.4675** | **20.6273** | **+16.6495** | **+23.7849** | **Disturbance step: load jumps; command was set to low positional-track value; large deficit** |
| f5 | 18.6663 | 26.6662 | 30.5254 | 21.3677 | +2.7014 | +3.8592 | Operator responded; near-balance; small remaining deficit |
| f6 | 15.8454 | 22.6363 | 31.4048 | 21.9834 | +6.1380 | +8.7685 | Load still growing; command dropped by positional correction; deficit grows again |
| f7 | 14.1033 | 20.1476 | — | — | — | — | Arm still overshooting; subsequent τ_s not tabulated |

**Sign convention:** positive residual = observed load exceeded command; negative residual = command exceeded load.

**Units verification:**
- τ_s(t+1) × R_W/K_T: N·m × Ω / (N·m/A) = N·m × (V·A)/(N·m) = V ✓
- tau_cmd = K_T × v_cmd / R_W: (N·m/A) × V / Ω = (N·m/A) × A = N·m ✓
- Residual [N·m] = τ_s(t+1) − tau_cmd = N·m − N·m = N·m ✓
- Residual [V] = τ_s(t+1) × R_W/K_T − v_cmd = V − V = V ✓

---

## 6. Structural observation — Trace K A IS the prediction-observation residual

**Finding:** In the Trace K (2-locus, Kernel V8-conformant) declaration, the A operator at step t+1 computes exactly the prediction-observation residual from step t's command.

**Proof:**

At step t+1 in Trace K:
```
A_K(t+1) = x[v_τ](t+1) − x[v_cmd](t+1)
          = τ_s(t+1) × R_W/K_T − v_cmd(t)
```

The closing residual defined in §5:
```
Residual(t) = τ_s(t+1) × R_W/K_T − v_cmd(t)
```

Therefore: **A_K(t+1) = Residual(t)** — exactly.

**Verification with simulation values at frame f5:**
```
A_K(f5) = x[v_τ](f5) − x[v_cmd](f5)
         = 20.6273 V − 3.9778 V
         = 16.6495 V

Residual(f4) = τ_s(f5) × R_W/K_T − v_cmd(f4)
             = 29.4675 × 0.7 − 3.9778
             = 20.6273 − 3.9778
             = 16.6495 V  ✓
```

**Physical significance:**

The Trace K A operator does not produce an arbitrary relational quantity. At each step, it produces the exact gap between what the prior command predicted the load would be (in voltage equivalent terms) and what the sensor actually observed. The relational calculation is inherently a prediction-observation measurement: it returns to the observable process at every step.

**This is the property the governing requirement establishes as necessary for a complete traceability chain.** The calculation does not complete at position 9 (v_cmd). It completes at position 13 (residual), which becomes position 5 (A input) at the next step.

---

## 7. Structural gap — Trace P A is NOT the prediction-observation residual

**Finding:** In the Trace P (Python demo implementation), the A operator at step t+1 does NOT compute the prediction-observation residual from step t's command.

**A_P at step t+1:**
```
A_P(t+1) = τ_s(t+1) − τ_s(t)   [N·m]
```

**The prediction-observation residual from step t:**
```
Residual(t) = τ_s(t+1) − tau_cmd(t)   [N·m]
            = τ_s(t+1) − K_T × v_cmd(t) / R_W
```

**The difference:**
```
Residual(t) − A_P(t+1) = [τ_s(t+1) − tau_cmd(t)] − [τ_s(t+1) − τ_s(t)]
                        = τ_s(t) − tau_cmd(t)
                        = τ_s(t) − K_T × v_cmd(t) / R_W
```

This is the load–command gap at step t, measured in N·m: how far was the observed load at step t from what was actually commanded at step t?

**In voltage terms:**
```
Residual_V(t) − A_P_V(t+1) = τ_s(t) × R_W/K_T − v_cmd(t)
                             = x[v_τ](t) − x[v_cmd](t)
                             = A_K(t)
```

Therefore: **Residual(t) = A_P(t+1) + A_K(t)** (in appropriate unit conversions)

Or equivalently: **A_P(t+1) differs from the true residual by exactly A_K(t) — the prior step's Trace K A output.**

**Verification at frame f4→f5:**
```
A_P(f5) = 26.9312 N·m (temporal change in sensor)
A_P_V(f5) = 26.9312 × 0.7 = 18.8519 V

Residual_V(f4) = 20.6273 − 3.9778 = 16.6495 V
A_K(f4)        = x[v_τ](f4) − x[v_cmd](f4) = 1.7754 − 10.2136 = −8.4382 V

Check: A_P_V(f5) + A_K(f4) = 18.8519 + (−8.4382) = 10.4137 V  — NOT 16.6495 V
```

The above discrepancy requires clarification. The correct relationship in voltage units:

Trace P produces delta_V_P = A_P × R_W/K_T. The true voltage residual is Residual_V = A_K at t+1. Their difference:

```
A_P_V(t+1) = [τ_s(t+1) − τ_s(t)] × R_W/K_T

Residual_V(t) = τ_s(t+1) × R_W/K_T − v_cmd(t)

A_P_V(t+1) − Residual_V(t) = [τ_s(t+1) − τ_s(t)] × R_W/K_T − [τ_s(t+1) × R_W/K_T − v_cmd(t)]
                             = −τ_s(t) × R_W/K_T + v_cmd(t)
                             = v_cmd(t) − x[v_τ](t)
                             = −[x[v_τ](t) − v_cmd(t)]
                             = −A_K(t)
```

Therefore: **A_P_V(t+1) = Residual_V(t) − A_K(t)**

Or: **Residual_V(t) = A_P_V(t+1) + A_K(t)**

**Verification at f4→f5:**
```
A_P_V(f5) = 18.8519 V
A_K(f4)   = 1.7754 − 10.2136 = −8.4382 V

A_P_V(f5) + A_K(f4) = 18.8519 + (−8.4382) = 10.4137 V

Residual_V(f4) should equal 10.4137? But we computed 16.6495 above.
```

Discrepancy detected. Let me identify the cause.

**Correction:** x[v_cmd] at frame f5 = v_cmd from PRIOR step = v_cmd_f4 = 3.9778 V.
So A_K(f5) = x[v_τ](f5) − x[v_cmd](f5) = 20.6273 − 3.9778 = 16.6495 V.

But A_K(f4) = x[v_τ](f4) − x[v_cmd](f4) = 1.7754 − v_cmd_f3 = 1.7754 − 10.2136 = −8.4382 V.

The formula A_P_V(t+1) = Residual_V(t) − A_K(t) uses A_K(t), not A_K(t+1):

```
Residual_V(f4) = A_P_V(f5) + A_K(f4)
               = 18.8519 + (−8.4382)
               = 10.4137 V
```

But the computed Residual_V(f4) = τ_s(f5) × R_W/K_T − v_cmd(f4) = 20.6273 − 3.9778 = 16.6495 V.

These do not match (10.4137 ≠ 16.6495). The algebraic relationship above has an error. Let me recheck.

```
A_P_V(t+1) − Residual_V(t)
= [τ_s(t+1) − τ_s(t)] × R_W/K_T − [τ_s(t+1) × R_W/K_T − v_cmd(t)]
= −τ_s(t) × R_W/K_T + v_cmd(t)
= v_cmd(t) − τ_s(t) × R_W/K_T
= −[τ_s(t) × R_W/K_T − v_cmd(t)]
```

Note: Residual_V(t) uses v_cmd(t) (the command issued AT step t). A_K(t) uses v_cmd(t−1) (the command issued BEFORE step t, i.e., x[v_cmd] at step t).

So: v_cmd(t) − τ_s(t) × R_W/K_T is NOT A_K(t) and NOT A_K(t+1).

It is: −[τ_s(t) × R_W/K_T − v_cmd(t)].

This is the negative of the gap between the current τ_s projection and the current v_cmd — a quantity not directly equivalent to either trace's A output.

**Consequence:** The algebraic relationship between A_P_V and Residual_V is:
```
Residual_V(t) = A_P_V(t+1) + [τ_s(t) × R_W/K_T − v_cmd(t)]
```

**Numerical verification at f4:**
```
τ_s(f4) × R_W/K_T − v_cmd(f4) = 1.7754 − 3.9778 = −2.2024

A_P_V(f5) + [τ_s(f4) × R_W/K_T − v_cmd(f4)]
= 18.8519 + (−2.2024)
= 16.6495 V  ✓ = Residual_V(f4)
```

The term [τ_s(t) × R_W/K_T − v_cmd(t)] = sigma_v(t) − v_cmd(t) = −frac(t) × V_RAISE (by the C projection).

Therefore: **Residual_V(t) = A_P_V(t+1) − frac(t) × V_RAISE**

**Verification:**
```
frac(f4) = (20° − 17.68°) / 20° = 0.116
frac(f4) × V_RAISE = 0.116 × 19.0 = 2.204 V

Residual_V(f4) = A_P_V(f5) − frac(f4) × V_RAISE
               = 18.8519 − 2.204
               = 16.648 ≈ 16.6495 V  ✓ (rounding)
```

**Interpretation:** The difference between A_P_V (what Trace P accumulates) and the true prediction-observation residual (what Trace K A computes) is exactly the positional authority term from C projection at the prior step:

```
A_P_V(t+1) = Residual_V(t) + frac(t) × V_RAISE
```

Trace P overcounts the residual by the positional term that was already embedded in v_cmd through C projection. Trace K's spatial-gap A does not overcount because it uses v_cmd (which includes the positional term) as its reference rather than tau_s (which does not include it).

---

## 8. Trace-P vs Trace-K comparison table

| Frame t | A_P [N·m] | A_P_V = delta_V [V] | A_K [V] | Residual_V(t−1) [V] | Difference: A_P_V − Residual_V(t−1) [V] | = frac(t−1) × V_RAISE [V] |
|---|---:|---:|---:|---:|---:|---:|
| f3 | 0.9236 | 0.6465 | −14.9336 | −14.9336 | 0.6465 − (−14.9336) = 15.5801 | v_cmd_f2 − sigma_v_f2 = 15.5801 − 0 = 15.5801 ✓ |
| f4 | 1.6128 | 1.1289 | −8.4382 | −8.4382 | 1.1289 − (−8.4382) = 9.5671 | v_cmd_f3 − sigma_v_f3 = 10.2136 − 0.6465 = 9.5671 ✓ |
| f5 | 26.9312 | 18.8519 | 16.6495 | 16.6495 | 18.8519 − 16.6495 = 2.2024 | v_cmd_f4 − sigma_v_f4 = 3.9778 − 1.7754 = 2.2024 ✓ |
| f6 | 1.0578 | 0.7405 | 2.7014 | 2.7014 | 0.7405 − 2.7014 = −1.9609 | v_cmd_f5 − sigma_v_f5 = 18.6663 − 20.6273 = −1.9610 ✓ |
| f7 | 0.8795 | 0.6156 | 6.1380 | 6.1380 | 0.6156 − 6.1380 = −5.5224 | v_cmd_f6 − sigma_v_f6 = 15.8454 − 21.3677 = −5.5223 ✓ |

**Note on Residual_V(t−1) column:** "Residual_V(t−1)" in this table is the residual closed by the step-t A operator. Specifically, A_K at frame t equals τ_s(t) × R_W/K_T − v_cmd(t−1) = Residual_V(t−1). This column also provides the physical reference for comparing A_P_V at frame t.

**Observation:** The difference between Trace P's accumulated step (A_P_V) and the true prediction-observation residual (A_K) is v_cmd(t−1) − sigma_v(t−1) — exactly the positional authority contribution frac(t−1) × V_RAISE embedded in the prior v_cmd. This is the term that makes the two traces numerically different at each step even though their sigma_v values telescope to the same result.

---

## 9. Complete per-step chain table (disturbance experiment, frames f3–f7)

The following table traces the full chain from Observable to Residual for each frame.

### Observable and M admission layer (same for both traces)

| Frame | τ_s(t) [N·m] | τ_d [N·m] | θ_prior [deg] | M admission | x[v_τ] = τ_s×R_W/K_T [V] | x[v_cmd] = v_cmd(t−1) [V] |
|---|---:|---:|---:|---|---:|---:|
| f3 | 0.9236 | 0.0 | 9.93 | Pass | 0.6465 | 15.5801 |
| f4 | 2.5363 | 25.0 (active) | 17.68 | Pass | 1.7754 | 10.2136 |
| f5 | 29.4675 | 25.0 | 22.06 | Pass | 20.6273 | 3.9778 |
| f6 | 30.5254 | 25.0 | 25.81 | Pass | 21.3677 | 18.6663 |
| f7 | 31.4048 | 25.0 | 28.29 | Pass | 21.9834 | 15.8454 |

Note: τ_d activates in the plant at the frame-4 integration step, so τ_s_f4 = 2.5363 N·m still reflects primarily gravitational load. The full 25 N·m disturbance appears in τ_s at frame f5 (one-step sensor lag established in Document 1 §10).

Note: θ_prior is the joint angle from the end of the PRIOR step's integrate_plant call — this is the θ that enters the C projection at the current frame.

### Operator layer

| Frame | A_P [N·m] | A_P_V [V] | B_P = delta_V [V] | A_K [V] | B_K [V] | R (both) |
|---|---:|---:|---:|---:|---:|---|
| f3 | 0.9236 | 0.6465 | 0.6465 | −14.9336 | −14.9336 | OPEN (OC-FD-1, OC-FD-2) |
| f4 | 1.6128 | 1.1289 | 1.1289 | −8.4382 | −8.4382 | OPEN |
| f5 | 26.9312 | 18.8519 | 18.8519 | 16.6495 | 16.6495 | OPEN |
| f6 | 1.0578 | 0.7405 | 0.7405 | 2.7014 | 2.7014 | OPEN |
| f7 | 0.8795 | 0.6156 | 0.6156 | 6.1380 | 6.1380 | OPEN |

Note: A_K at f3 and f4 is strongly negative — the command well exceeded the load. This is the pre-disturbance phase: the controller was driving the arm upward against only gravitational load.

### Accumulation and projection layer (same for both traces under telescoping)

| Frame | sigma_v [V] | frac | frac × V_RAISE [V] | v_cmd [V] | tau_cmd [N·m] |
|---|---:|---:|---:|---:|---:|
| f3 | 0.6465 | +0.5038 | +9.572 | 10.2136 | 14.591 |
| f4 | 1.7754 | +0.1160 | +2.204 | 3.9778 | 5.683 |
| f5 | 20.6273 | −0.1030 | −1.957 | 18.6663 | 26.666 |
| f6 | 21.3677 | −0.2905 | −5.520 | 15.8454 | 22.636 |
| f7 | 21.9834 | −0.4145 | −7.876 | 14.1033 | 20.148 |

Note: frac = (0.3491 − θ_prior × π/180) / 0.3491, where θ_prior is from the prior plant step.

frac verification at f5: θ_prior = 22.06°, frac = (20° − 22.06°)/20° = −0.103. v_cmd = 20.6273 − 0.103 × 19.0 = 20.6273 − 1.957 = 18.670 ≈ 18.6663 ✓

### Physical consequence, subsequent measurement, and residual

| Frame t | v_cmd(t) [V] | tau_cmd(t) [N·m] | τ_s(t+1) [N·m] | Residual [V] | Residual [N·m] | Physical reading |
|---|---:|---:|---:|---:|---:|---|
| f3 | 10.2136 | 14.591 | 2.5363 (f4) | 1.7754 − 10.2136 = **−8.4382** | 2.5363 − 14.591 = **−12.055** | Command exceeded load; arm accelerating |
| f4 | 3.9778 | 5.683 | 29.4675 (f5) | 20.6273 − 3.9778 = **+16.6495** | 29.4675 − 5.683 = **+23.785** | Disturbance arrived; load massively exceeds command |
| f5 | 18.6663 | 26.666 | 30.5254 (f6) | 21.3677 − 18.6663 = **+2.7014** | 30.5254 − 26.666 = **+3.859** | Operator responded; near-balance; small remaining deficit |
| f6 | 15.8454 | 22.636 | 31.4048 (f7) | 21.9834 − 15.8454 = **+6.1380** | 31.4048 − 22.636 = **+8.769** | Arm overshooting; positional correction reduced command; deficit grows |

**Residual identity confirmation (Trace K):**

The A_K value at frame t+1 in the operator layer table equals the Residual [V] at frame t:
- A_K(f4) = −8.4382 V = Residual_V(f3) ✓
- A_K(f5) = +16.6495 V = Residual_V(f4) ✓
- A_K(f6) = +2.7014 V = Residual_V(f5) ✓
- A_K(f7) = +6.1380 V = Residual_V(f6) ✓

The residual identity A_K(t+1) = Residual_V(t) holds exactly at every frame. Trace K A is the prediction-observation residual.

---

## 10. Traceability chain closure statement

The calculation chain for the motor–sensor–actuator system in the 2-locus Trace K declaration closes as follows:

```
τ_s(t) [sensor, N·m]
    ↓ M admission: |τ_s| ≤ 68.571 N·m
    ↓ Projection: x[v_τ] = τ_s × R_W/K_T [V]  ← declared derived quantity
    ↓ v_cmd(t−1) [retained from prior C projection] → x[v_cmd] [V]
    ↓ A: g[e_1] = x[v_τ] − x[v_cmd] [V]  ← Kernel V8 spatial difference
    ↓ B: B(g)[e_1] = g[e_1] [V]  ← identity (terminal edge)
    ↓ R: OPEN (structurally zero antisymmetric term; ρ_base, χ₀ undeclared)
    ↓ Sigma: sigma_v += B_out [V]  ← accumulated state
    ↓ C projection: v_cmd = sigma_v + frac × V_RAISE [V]  ← declared projection
    ↓ Conversion: tau_cmd = K_T × v_cmd / R_W [N·m]  ← prediction
    ↓ Plant integration: integrate_plant(tau_cmd, τ_d, θ, ω)  ← not observable
    ↓ τ_s(t+1) = MASS × G × ARM × sin(θ_new) + τ_d [N·m]  ← subsequent measurement
    ↓ Residual: τ_s(t+1) × R_W/K_T − v_cmd(t) [V]  ← closes the loop
    = A_K(t+1)  ← enters next cycle's A operator
```

The chain is closed. The calculation returns to the observable process.

The physical process being tracked: whether the commanded motor voltage is adequate to balance the mechanical load as observed by the sensor. The relational gap between observed load and commanded voltage, expressed in Volts, is evaluated at every step by the A operator itself.

---

## 11. OPEN conditions returned to Origin by this analysis

The traceability analysis reveals the following conditions in addition to those established in Documents 1–4:

| ID | Condition | Revealed by |
|---|---|---|
| OC-TRACE-1 | Residual units — Volts vs N·m | The residual is dimensionally defined in both V (Trace K form) and N·m (physical torque form). Origin must declare which form is the authoritative comparison quantity for the declared relational calculation. |
| OC-TRACE-2 | Trace K running vs analytic | Trace K A values shown here are computed from the Trace P simulation's inputs. If Trace K were running as an implemented kernel, sigma_v and subsequent v_cmd would differ (except under zero-initial-condition telescoping). The accumulated Trace K values have not been verified by simulation. The declaration must specify whether the telescoping property is claimed or whether a full Trace K run is required. |
| OC-TRACE-3 | Positional authority in residual | Trace P accumulates A_P_V = Residual_V + frac × V_RAISE at each step. The positional authority term from C projection is embedded in the Trace P accumulation but not in the Trace K residual. Origin must determine whether the positional authority term is a relational quantity that should appear in the operator chain (requiring a locus for θ in the NodeField) or remains outside the kernel in the C projection. |

Previously established OPEN conditions (Documents 1–4) remain in effect: OC-FD-1, OC-FD-2, OC-ACC-1, OC-ARCH-1, OC-LOCI-1, OC-PROJ-1.

---

## 12. What this document does not authorize

- No implementation of the declared structure
- No modification of the frozen kernel
- No implementation of M1
- No simulation of Trace K (not authorized and not feasible without implementation)
- No resolution of the OPEN conditions listed above

The traceability matrix is observational and analytical. All numerical values are derived
from the authoritative Trace P simulation. Trace K A values are analytical computations
from those simulation inputs, not from a running kernel.

---

*This document satisfies the Origin/Verifier requirement to add a mathematical-to-observable
traceability matrix to the current Generator deliverables. The calculation returns to the
observable process. The residual is dimensionally and physically defined. The distinction
between physical relationships, measured quantities, derived quantities, operator outputs,
and predictions is preserved throughout.*
