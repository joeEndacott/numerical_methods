use crate::grid::Grid;

/// # Grid function
///
/// ## Description
/// `GridFunction` represents a real-valued function of a real variable sampled
/// on a grid of 1D points. The value of the function at each sampling point
/// corresponds to an element in the vector `function_values`. The sampling
/// points are contained in the `Grid` `grid`.
///
/// ## Example use case
/// Suppose that we want to sample the function f(x) = 2*x at the points
/// x = 0.0, 1.0, 2.0, 3.0, 4.0, 5.0. We can represent the sampled function as a
/// `GridFunction` with the code below.
/// ```
/// let grid_points = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// let function_values = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
/// let grid = Grid { grid_points };
/// let grid_func = GridFunction { grid, function_values };
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct GridFunction {
    pub grid: Grid,
    pub function_values: Vec<f64>,
}

impl GridFunction {
    /// # New grid function
    ///
    /// ## Description
    /// `new_grid_function` generates a `GridFunction`, given a real-valued
    /// function of a real variable `func` and a `Grid` `grid`. `func` is
    /// sampled at each grid point in `grid` and the values are stored
    /// in the vector `function_values`.
    ///
    /// ## Example use case
    /// Suppose that we want to sample the function f(x) = sin(x) at the points
    /// x = 0.0, 1.0, 2.0, 3.0, 4.0, 5.0. We can represent the sampled function
    /// as a `GridFunction` with the code below.
    /// ```
    /// fn func(x: f64) -> f64 {
    ///    f64::sin(x)
    /// }
    ///
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func = GridFunction::new_grid_function(&grid, func);
    /// ```
    ///
    pub fn new_grid_function<F>(grid: &Grid, func: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        // Creates a vector containing the value of func at each grid point.
        let function_values: Vec<f64> =
            grid.grid_points.iter().map(|&x| func(x)).collect();

        GridFunction {
            grid: grid.clone(),
            function_values,
        }
    }

    /// # New constant grid function
    ///
    /// ## Description
    /// `new_constant_grid_function` generates a constant (flat) `GridFunction`
    /// with height `scalar`, given a `Grid` `grid`.
    ///
    /// ## Example use case
    /// Suppose that we want to create a constant grid function with height 2.0
    /// sampled at the points x = 0.0, 1.0, 2.0, 3.0, 4.0, 5.0. We can
    /// represent the sampled function as a `GridFunction` with the code below.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
    /// ```
    ///
    pub fn new_constant_grid_function(grid: &Grid, scalar: f64) -> Self {
        let grid = grid.clone();

        let function_values: Vec<f64> =
            grid.grid_points.iter().map(|_| scalar).collect();

        GridFunction {
            grid,
            function_values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid_function() {
        // Test with the function sin(x).
        let grid = Grid::new_uniform_grid(0.0, 1.0, 6);
        let grid_func = GridFunction::new_grid_function(&grid, f64::sin);

        let expected_values: Vec<f64> =
            grid.grid_points.iter().map(|&x| f64::sin(x)).collect();
        assert_eq!(
            grid_func.function_values, expected_values,
            "new_grid_function failed to sample a sin function"
        );

        // Test with the function 1/x.
        let grid_func = GridFunction::new_grid_function(&grid, |x| 1.0 / x);
        let expected_values: Vec<f64> =
            grid.grid_points.iter().map(|x| 1.0 / x).collect();
        assert_eq!(
            grid_func.function_values, expected_values,
            "new_grid_function failed to sample the function 1/x"
        );
    }

    #[test]
    fn test_new_constant_grid_function() {
        let grid = Grid::new_uniform_grid(0.0, 1.0, 6);
        let scalar = 2.0;
        let grid_func = GridFunction::new_constant_grid_function(&grid, scalar);

        let expected_values: Vec<f64> = vec![scalar; grid.grid_points.len()];
        assert_eq!(
            grid_func.function_values, expected_values,
            "constant_grid_function failed."
        );
    }

    #[test]
    fn test_empty_grid() {
        let grid = Grid::new_uniform_grid(0.0, 1.0, 0);

        // Test new_grid_function.
        let grid_func = GridFunction::new_grid_function(&grid, f64::sin);
        assert!(
            grid_func.function_values.is_empty(),
            "new_grid_function failed with an empty grid."
        );

        // Test new_constant_grid_function
        let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
        assert!(
            grid_func.function_values.is_empty(),
            "new_constant_grid_function failed with an empty grid."
        );
    }

    #[test]
    fn test_grid_function_debug() {
        let grid_points = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let function_values = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
        let grid = Grid {
            grid_points: grid_points.clone(),
        };
        let grid_func = GridFunction {
            grid,
            function_values: function_values.clone(),
        };

        let debug_str = format!("{:?}", grid_func);
        let expected_str = format!(
            "GridFunction {{ grid: Grid {{ grid_points: {:?} }}, function_values: {:?} }}",
            grid_points, function_values
        );
        assert_eq!(debug_str, expected_str, "Debug failed.");
    }

    #[test]
    fn test_grid_function_clone() {
        let grid_points = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let function_values = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
        let grid = Grid { grid_points };
        let grid_func = GridFunction {
            grid,
            function_values,
        };

        let grid_func_clone = grid_func.clone();
        assert_eq!(grid_func.function_values, grid_func_clone.function_values);
        assert_eq!(
            grid_func.grid.grid_points, grid_func_clone.grid.grid_points,
            "Clone failed."
        );
    }

    #[test]
    fn test_grid_function_partial_eq() {
        let grid_points = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let function_values = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
        let grid = Grid { grid_points };
        let grid_func_1 = GridFunction {
            grid: grid.clone(),
            function_values: function_values.clone(),
        };

        // Test equality with itself.
        assert_eq!(grid_func_1, grid_func_1, "Equality failed with itself.");

        // Test equality with a clone.
        let grid_func_clone = grid_func_1.clone();
        assert_eq!(
            grid_func_1, grid_func_clone,
            "Equality failed with a clone."
        );

        // Test equality with a GridFunction defined using new_grid_function.
        let grid_func_2 = GridFunction::new_grid_function(&grid, |x| 2.0 * x);
        assert_eq!(
            grid_func_1, grid_func_2,
            "Equality failed with a GridFunction defined using new_grid_function."
        );
    }
}
