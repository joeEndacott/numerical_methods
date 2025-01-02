use crate::boundary_conditions::BoundaryConditions;
use crate::grid_function::GridFunction;
use nalgebra::LU;

/// # Get Jacobian matrix
///
/// ## Description
/// `get_jacobian_matrix` takes a DE function, `de_func`, and a trial solution,
/// `grid_func`, as inputs, and returns the Jacobian matrix of `de_func` at
/// `grid_func`. The Jacobian matrix is stored as a flat vector, in row-major
/// order.
///
/// To enforce the boundary conditions (BCs), the first and last rows of the
/// Jacobian matrix are set equal to [1, 0, 0, ..., 0] and [0, 0, ..., 0, 1],
/// respectively.
///
/// For the internal rows, the element in row i and column j of the Jacobian
/// matrix is equal to the partial derivative of the ith component of the
/// residual vector, `de_func(&grid_func)`, with respect to the jth component
/// of `grid_func`.
///
/// ## Example use case
/// Suppose we have a DE function, `differential_equation_function`, and a trial
/// solution, `initial_guess_func`. The code below calculates the Jacobian
/// matrix of `differential_equation_function` at `initial_guess_func`.
/// ```
/// let grid = Grid::new_uniform_grid(0.0, 1.0, 10);
/// let initial_guess_grid_func =
///     GridFunction::new_grid_function(&grid, initial_guess_func);
/// let jacobian_matrix = get_jacobian_matrix(
///     differential_equation_function,
///     &initial_guess_grid_func,
///     1e-6,
/// );
/// ```
///
pub fn get_jacobian_matrix<F>(
    de_func: &F,
    grid_func: &GridFunction,
    step_size: f64,
) -> Vec<f64>
where
    F: Fn(&GridFunction) -> GridFunction,
{
    // The Jacobian matrix is a square matrix.
    // The number of rows/columns in the Jacobian matrix is equal to the number
    // of function values in grid_func.
    let matrix_size = grid_func.function_values.len();

    // Pre-allocates memory for the Jacobian matrix.
    let mut jacobian_matrix: Vec<f64> =
        Vec::with_capacity(matrix_size * matrix_size);

    // Iterate over the matrix elements in row-major order.
    // i iterates over the matrix rows and j iterates over the matrix columns.
    for i in 0..(matrix_size) {
        for j in 0..matrix_size {
            if i == 0 {
                // Handles the first row of the Jacobian matrix.
                jacobian_matrix.push(if j == 0 { 1.0 } else { 0.0 });
            } else if i == matrix_size - 1 {
                // Handles the last row of the Jacobian matrix.
                jacobian_matrix.push(if j == matrix_size - 1 {
                    1.0
                } else {
                    0.0
                });
            } else {
                // Handles the interior rows of the Jacobian matrix.
                let mut perturbed_grid_func = grid_func.clone();
                perturbed_grid_func.function_values[j] += step_size;

                // Calculates the (i, j) element of the Jacobian matrix using
                // the forwards difference approximation.
                let jacobian_entry = (de_func(&perturbed_grid_func)
                    .function_values[i]
                    - de_func(&grid_func).function_values[i])
                    / step_size;
                jacobian_matrix.push(jacobian_entry);
            }
        }
    }

    jacobian_matrix
}

/// # Get residual vector
///
/// ## Description
/// `get_residual_vector` takes a DE function, `de_func`, a trial solution,
/// `grid_func`, and boundary conditions, `boundary_conditions`, as inputs, and
/// returns the residual vector of `de_func` at `grid_func`.
///
/// The residual vector is the vector of function values of `de_func` evaluated
/// at `grid_func`, i.e. `de_func(&grid_func).function_values`.
///
/// The first and last elements of the residual vector are modified to drive the
/// solution to satisfy the boundary conditions.
///
/// ## Example use case
/// Suppose we have a DE function, `de_func`, a trial solution,
/// `grid_func_guess`, and boundary conditions, `boundary_conditions`. The code
/// below calculates the residual vector of `de_func` at `grid_func_guess`.
/// ```
/// let residual_vector = get_residual_vector(
///     de_func,
///     &grid_func_guess,
///     &boundary_conditions,
/// );
/// ```
///
/// ## Todo
/// Modify the function so that it can handle mixed and Neumann BCs.
///
fn get_residual_vector<F>(
    de_func: &F,
    grid_func: &GridFunction,
    boundary_conditions: &BoundaryConditions,
) -> Vec<f64>
where
    F: Fn(&GridFunction) -> GridFunction,
{
    let mut residual_vector = de_func(grid_func).function_values;
    let length = residual_vector.len();

    // Modifies the first and last elements of the residual vector to enforce
    // the boundary conditions.
    residual_vector[0] =
        grid_func.function_values[0] - boundary_conditions.left_bc;
    residual_vector[length - 1] =
        grid_func.function_values[length - 1] - boundary_conditions.right_bc;

    residual_vector
}

/// # Solve linear system
///
/// ## Description
/// `solve_linear_system` takes a matrix `matrix`, a vector `vector`, the size
/// of the matrix `matrix_size` as inputs, and returns the solution,
/// `solution`, to the system of linear equations `matrix * solution = vector`.
///
/// `matrix` is a flat vector that represents a square matrix in row-major
/// order. `vector` is a flat vector that represents a column vector.
///
/// `solve_linear_system` uses an LU decomposition algorithm from the nalgebra
/// library to solve the system of linear equations.
///
/// ## Example use case
/// Suppose we have a matrix `matrix` and a vector `vector`. The code below
/// calculates the solution to the system of linear equations `matrix * x =
/// vector`.
/// ```
/// let matrix = vec![1.0, 2.0, 3.0, 4.0];
/// let vector = vec![5.0, 6.0];
/// let matrix_size = 2;
/// let solution = solve_linear_system(&matrix, &vector, matrix_size);
/// ```
///
fn solve_linear_system(
    matrix: &Vec<f64>,
    vector: &Vec<f64>,
    matrix_size: usize,
) -> Vec<f64> {
    // Creates a dense matrix from matrix.
    let matrix =
        nalgebra::DMatrix::from_row_slice(matrix_size, matrix_size, &matrix);

    // Creates a dense vector from vector.
    let vector = nalgebra::DVector::from_column_slice(&vector);

    // Perform LU decomposition and solve the system of linear equations.
    let lu = LU::new(matrix);
    let solution = lu.solve(&vector).unwrap();

    // Convert the solution to a Vec<f64>.
    solution.data.as_vec().clone()
}

/// # Newton's method step
///
/// ## Description
/// `newtons_method_step` takes a DE function, `de_func` and a trial solution,
/// `grid_func_guess`, as inputs, and returns the updated the trial solution
/// of the DE using Newton's method.
///
/// `step_size` is the step size used in the finite difference approximation
/// when calculating the Jacobian matrix. `step_size` should be small.
///
/// ## Example use case
/// Suppose we have a DE function, `differential_equation_function`, and a trial
/// solution, `initial_guess_func`. The code below calculates the updated trial
/// solution of the DE using Newton's method.
/// ```
/// let grid = Grid::new_uniform_grid(0.0, 1.0, 10);
/// let initial_guess_grid_func = GridFunction::new_grid_function(&grid,
/// initial_guess_func);
/// let updated_guess_grid_func = newtons_method_step
/// (differential_equation_function, &initial_guess_grid_func, 1e-6);
/// ```
///
/// ## Todo
/// Currently, to calculate the updated guess, newtons_method_step uses an LU
/// decomposition algorithm to solve a system of linear equations. I would like
/// to experiment with other algorithms and see which algorithm is the fastest.
///
fn newtons_method_step<F>(
    de_func: F,
    grid_func_guess: &GridFunction,
    boundary_conditions: &BoundaryConditions,
    step_size: f64,
) -> GridFunction
where
    F: Fn(&GridFunction) -> GridFunction,
{
    let jacobian_matrix =
        get_jacobian_matrix(&de_func, &grid_func_guess, step_size);
    let residual_vector =
        get_residual_vector(&de_func, &grid_func_guess, &boundary_conditions);
    let matrix_size = grid_func_guess.function_values.len();

    // Solves the system of linear equations J * Δ = -F for Δ, where J is the
    // Jacobian matrix, Δ is the update to grid_func_guess, and F is the
    // residual vector.
    let grid_func_update =
        solve_linear_system(&jacobian_matrix, &residual_vector, matrix_size);

    // Adds grid_func_update to grid_func_guess to get the next guess.
    let updated_guess_values: Vec<f64> = grid_func_guess
        .function_values
        .iter()
        .zip(grid_func_update.iter())
        .map(|(x, y)| x + y)
        .collect();

    GridFunction {
        grid: grid_func_guess.grid.clone(),
        function_values: updated_guess_values,
    }
}

/// # Newton's method
///
/// ## Description
/// `newtons_method` takes a DE function, `de_func`, boundary conditions
/// `boundary_conditions`a trial solution, `grid_func_initial_guess`, and a
/// maximum number of iterations, `num_iterations`, as inputs, and returns the
/// approximate solution of the DE using Newton's method.
///
/// ## Example use case
/// Todo: add example use case
///
/// ## Todo
/// Modify the `newtons_method` so that it stops iterating when the solution is
/// within a certain tolerance of the true solution, rather than when a maximum
/// number of iterations is reacher.
///
/// Currently, this algorithm isn't working - I'm still troubleshooting.
///
pub fn newtons_method<F>(
    de_func: F,
    boundary_conditions: &BoundaryConditions,
    grid_func_initial_guess: &GridFunction,
    num_iterations: usize,
) -> GridFunction
where
    F: Fn(&GridFunction) -> GridFunction,
{
    let mut grid_func_guess = grid_func_initial_guess.clone();

    for _ in 0..num_iterations {
        // Updates the guess using Newton's method.
        grid_func_guess = newtons_method_step(
            &de_func,
            &grid_func_guess,
            boundary_conditions,
            1e-6,
        );
    }
    grid_func_guess
}
