/// # Boundary conditions
///
/// ## Description
/// `BoundaryConditions` stores the boundary conditions (BCs) for a 1D boundary
/// value problem (BVP).
///
/// Currently, only Dirichlet BCs are supported. Support for more BCs will be
/// implemented in the future.
///
/// The Dirichlet BCs are stored as a two numbers, `left_bc` and `right_bc`,
/// which represent the values of the function at the left and right boundaries
/// of the domain, respectively.
///
/// ## Example use case
/// Suppose that we have a BVP for a function f(x), with Dirichlet BCs f(0) = 0
/// and f(1) = 1. We can represent the BCs with a `BoundaryConditions` struct
/// with the code below.
/// ```
/// left_bc = 0.0;
/// right_bc = 1.0;
/// let dirichlet_bcs = BoundaryConditions { left_bc, right_bc };     
/// ```
///
/// ## Todo
/// Add support for more boundary conditions.
///
#[derive(Debug, Clone)]
pub struct BoundaryConditions {
    pub left_bc: f64,
    pub right_bc: f64,
}

impl BoundaryConditions {
    /// # New Dirichlet boundary conditions
    ///
    /// ## Description
    /// `new_dirichlet_bcs` creates a new `BoundaryConditions` struct with the
    /// left boundary condition `left_bc` and the right boundary condition
    /// `right_bc`.
    ///
    /// ## Example use case
    /// Suppose that we have a BVP for a function f(x), with Dirichlet BCs f(0)
    /// = 0 and f(1) = 1. We can represent the BCs with a `BoundaryConditions`
    /// struct with the code below.
    /// ```
    /// let dirichlet_bcs = BoundaryConditions::new_dirichlet_bcs(0.0, 1.0);
    /// ```
    ///
    pub fn new_dirichlet_bcs(left_bc: f64, right_bc: f64) -> Self {
        BoundaryConditions { left_bc, right_bc }
    }
}
