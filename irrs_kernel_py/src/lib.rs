/// PyO3 binding for Frozen Kernel V8 (irrs-kernel 8.0.0)
///
/// Exposes the A and B operators, DeclaredRelations, NodeField, and EdgeField
/// to Python. R is intentionally NOT exposed: rho_base and chi_0 are OPEN
/// conditions (section 11.2 of kernel_reference.md) — they have not been
/// declared through M.
///
/// Calling operator_e (the full ABR chain) would panic via require_rho_declaration.
/// This binding prevents that by not exposing operator_e or operator_r.
///
/// Generator: Claude Sonnet 4.6 (ABR Minimal AI closed-loop experiment)
/// Does NOT modify abr-irrs or irrs-kernel in any way.

use pyo3::prelude::*;
use irrs_kernel::operators::{
    DeclaredRelations, EdgeProvenance, NodeField, EdgeField,
    operator_a as rust_operator_a,
    operator_b as rust_operator_b,
    relational_isolation as rust_relational_isolation,
};

// ─── EdgeProvenance integer codes ────────────────────────────────────────────
// These mirror the EdgeProvenance enum variants in the frozen kernel.
// Python callers use: PROVENANCE_CONTINUATION=0, PROVENANCE_COUPLING=1,
//                     PROVENANCE_PERSISTENCE=2
fn decode_provenance(code: u8) -> EdgeProvenance {
    match code {
        0 => EdgeProvenance::Continuation,
        1 => EdgeProvenance::Coupling,
        2 => EdgeProvenance::Persistence,
        _ => panic!(
            "Unknown EdgeProvenance code {}. Use 0=Continuation, 1=Coupling, 2=Persistence",
            code
        ),
    }
}

// ─── PyDeclaredRelations ─────────────────────────────────────────────────────

/// The foundational primitive encoded as a structure.
///
/// Wraps irrs-kernel DeclaredRelations. Construct with DeclaredRelations.from_edges().
/// Every edge must carry a provenance (0=Continuation, 1=Coupling, 2=Persistence).
/// Persistence edges are not declared here; use accumulator state instead.
#[pyclass(name = "DeclaredRelations")]
pub struct PyDeclaredRelations {
    inner: DeclaredRelations,
}

#[pymethods]
impl PyDeclaredRelations {
    /// from_edges(n_nodes, edges, provenance) -> DeclaredRelations
    ///
    /// edges: list of (src, tgt) node index tuples
    /// provenance: list of integer codes per edge
    ///   0 = Continuation (within-family spatial continuation)
    ///   1 = Coupling (cross-family, declared from physical coupling)
    ///   2 = Persistence (relational-evolutionary, across declared process steps)
    #[staticmethod]
    fn from_edges(
        n_nodes: usize,
        edges: Vec<(usize, usize)>,
        provenance: Vec<u8>,
    ) -> PyResult<Self> {
        if edges.len() != provenance.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "edges ({}) and provenance ({}) must have the same length",
                edges.len(),
                provenance.len()
            )));
        }
        let prov: Vec<EdgeProvenance> = provenance
            .iter()
            .map(|&p| decode_provenance(p))
            .collect();
        Ok(PyDeclaredRelations {
            inner: DeclaredRelations::from_edges_with_provenance(n_nodes, edges, prov),
        })
    }

    /// Number of declared nodes.
    #[getter]
    fn n_nodes(&self) -> usize {
        self.inner.n_nodes
    }

    /// Number of declared edges.
    fn n_edges(&self) -> usize {
        self.inner.n_edges()
    }

    /// Source node of edge e.
    fn src(&self, e: usize) -> usize {
        self.inner.src(e)
    }

    /// Target node of edge e.
    fn tgt(&self, e: usize) -> usize {
        self.inner.tgt(e)
    }

    /// Relational isolation check.
    /// Returns True when every edge has empty adj+ and adj-.
    /// In this case B and R's antisymmetric terms are structurally zero.
    fn is_isolated(&self) -> bool {
        rust_relational_isolation(&self.inner)
    }

    /// adj_plus[e]: list of successor edge indices (edges starting where e ends).
    fn adj_plus(&self, e: usize) -> Vec<usize> {
        self.inner.adj_plus[e].clone()
    }

    /// adj_minus[e]: list of predecessor edge indices (edges ending where e starts).
    fn adj_minus(&self, e: usize) -> Vec<usize> {
        self.inner.adj_minus[e].clone()
    }
}

// ─── PyNodeField ─────────────────────────────────────────────────────────────

/// Numerical projection through M over declared nodes.
///
/// data[component][node] = value. All values must be finite (∈ D).
/// This is the input to the A operator.
#[pyclass(name = "NodeField")]
pub struct PyNodeField {
    inner: NodeField,
}

#[pymethods]
impl PyNodeField {
    /// NodeField(data)
    ///
    /// data: list of lists, data[component][node] — all values must be finite.
    #[new]
    fn new(data: Vec<Vec<f64>>) -> PyResult<Self> {
        // NodeField::new panics on invalid input (non-finite or shape mismatch).
        // Let the panic propagate as a Rust panic; PyO3 converts it to RuntimeError.
        Ok(PyNodeField {
            inner: NodeField::new(data),
        })
    }

    /// Number of components (k).
    #[getter]
    fn k(&self) -> usize {
        self.inner.k
    }

    /// Number of nodes (n).
    #[getter]
    fn n(&self) -> usize {
        self.inner.n
    }

    /// Raw data: data[component][node].
    fn data(&self) -> Vec<Vec<f64>> {
        self.inner.data.clone()
    }
}

// ─── PyEdgeField ─────────────────────────────────────────────────────────────

/// Output of the A and B operators.
///
/// spatial[component][edge] — the spatial field values.
/// comp[pair][node] — component differences at each node (for declared component pairs).
/// comp_pairs — the declared component pair indices.
#[pyclass(name = "EdgeField")]
pub struct PyEdgeField {
    inner: EdgeField,
}

#[pymethods]
impl PyEdgeField {
    /// Spatial field: spatial[component][edge].
    #[getter]
    fn spatial(&self) -> Vec<Vec<f64>> {
        self.inner.spatial.clone()
    }

    /// Component differences: comp[pair][node].
    /// Empty for declarations with no component pairs.
    #[getter]
    fn comp(&self) -> Vec<Vec<f64>> {
        self.inner.comp.clone()
    }

    /// Declared component pair indices.
    #[getter]
    fn comp_pairs(&self) -> Vec<(usize, usize)> {
        self.inner.comp_pairs.clone()
    }

    /// Number of components (k).
    #[getter]
    fn k(&self) -> usize {
        self.inner.k
    }

    /// Number of spatial edges.
    fn n_edges(&self) -> usize {
        if self.inner.spatial.is_empty() {
            0
        } else {
            self.inner.spatial[0].len()
        }
    }
}

// ─── Operator functions ───────────────────────────────────────────────────────

/// operator_a(node_field, rel, pairs) -> EdgeField
///
/// A operator: A(x)[e] = x[src(e)] - x[tgt(e)] for each declared edge e.
/// pairs: list of (component_a, component_b) index tuples for cross-component terms.
///        Pass [] for no component pairs (the common case).
///
/// Returns EdgeField with spatial[c][e] = x[c][src(e)] - x[c][tgt(e)].
/// Section 11.1 of kernel_reference.md.
#[pyfunction]
fn operator_a(
    f: &PyNodeField,
    rel: &PyDeclaredRelations,
    pairs: Vec<(usize, usize)>,
) -> PyResult<PyEdgeField> {
    let ef = rust_operator_a(&f.inner, &rel.inner, &pairs);
    Ok(PyEdgeField { inner: ef })
}

/// operator_b(edge_field, rel) -> EdgeField
///
/// B operator: B(g)[e] = g[e] + sum_{f in adj+(e)} g[f]
/// For terminal edges (adj+ empty), B(g)[e] = g[e] (identity).
/// Section 11.1 of kernel_reference.md.
#[pyfunction]
fn operator_b(g: &PyEdgeField, rel: &PyDeclaredRelations) -> PyResult<PyEdgeField> {
    let ef = rust_operator_b(&g.inner, &rel.inner);
    Ok(PyEdgeField { inner: ef })
}

/// relational_isolation(rel) -> bool
///
/// Returns True when every edge has empty adj+ and adj-.
/// In this case B and R's antisymmetric terms are structurally zero.
/// Section 11.5 of kernel_reference.md.
#[pyfunction]
fn relational_isolation(rel: &PyDeclaredRelations) -> bool {
    rust_relational_isolation(&rel.inner)
}

// ─── Module ───────────────────────────────────────────────────────────────────

/// irrs_kernel_py — PyO3 binding for Frozen Kernel V8 operators A and B.
///
/// R is not exposed: rho_base and chi_0 are OPEN (not declared through M).
/// Calling operator_e would panic; this binding prevents that.
///
/// EdgeProvenance constants:
///   PROVENANCE_CONTINUATION = 0
///   PROVENANCE_COUPLING     = 1
///   PROVENANCE_PERSISTENCE  = 2
///
/// Usage:
///   from irrs_kernel_py import DeclaredRelations, NodeField, operator_a, operator_b
///   from irrs_kernel_py import PROVENANCE_COUPLING, relational_isolation
///
///   rel = DeclaredRelations.from_edges(2, [(0, 1)], [PROVENANCE_COUPLING])
///   f   = NodeField([[v_tau, v_cmd_prior]])   # data[component][node]
///   a   = operator_a(f, rel, [])             # pairs=[] for no component pairs
///   b   = operator_b(a, rel)                 # terminal edge: b == a
///   a_k = a.spatial[0][0]                    # spatial[component][edge]
#[pymodule]
fn irrs_kernel_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDeclaredRelations>()?;
    m.add_class::<PyNodeField>()?;
    m.add_class::<PyEdgeField>()?;
    m.add_function(wrap_pyfunction!(operator_a, m)?)?;
    m.add_function(wrap_pyfunction!(operator_b, m)?)?;
    m.add_function(wrap_pyfunction!(relational_isolation, m)?)?;

    // EdgeProvenance integer codes (mirrors the frozen kernel enum)
    m.add("PROVENANCE_CONTINUATION", 0u8)?;
    m.add("PROVENANCE_COUPLING", 1u8)?;
    m.add("PROVENANCE_PERSISTENCE", 2u8)?;

    Ok(())
}
