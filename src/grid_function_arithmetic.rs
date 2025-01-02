//use crate::grid::Grid;
use crate::grid_function::GridFunction;

impl GridFunction {
    /// # Grid function add
    ///
    /// ## Description
    /// `add` adds the `GridFunction` `grid_func` to the current `GridFunction`
    /// and returns the result.
    ///
    /// The current `GridFunction` and `grid_func` should both have the same
    /// `Grid`. If they have different `Grids`, the `Grid` from the current
    /// `GridFunction` will be used.
    ///
    /// ## Example use case
    /// Suppose that we have two `GridFunctions` `grid_func_1` and `grid_func_2`
    /// and we want to add them together. The code below does this.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 2.0);
    /// let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 3.0);
    /// let grid_func_sum = grid_func_1.add(&grid_func_2);
    /// ```
    ///
    pub fn add(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x + y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function subtract
    ///
    /// ## Description
    /// `subtract` subtracts the `GridFunction` `grid_func` from the current
    /// `GridFunction` and returns the result.
    ///
    /// The current `GridFunction` and `grid_func` should both have the same
    /// `Grid`. If they have different `Grids`, the `Grid` from the current
    /// `GridFunction` will be used.
    ///
    /// ## Example use case
    /// Suppose that we have two `GridFunctions` `grid_func_1` and `grid_func_2`
    /// and we want to subtract `grid_func_2` from `grid_func_1`. The code
    /// below does this.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 2.0);
    /// let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 3.0);
    /// let grid_func_difference = grid_func_1.subtract(&grid_func_2);
    /// ```
    ///
    pub fn subtract(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x - y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function multiply
    ///
    /// ## Description
    /// `multiply` multiplies the current `GridFunction` by the `GridFunction`
    /// `grid_func` and returns the result.
    ///
    /// The current `GridFunction` and `grid_func` should both have the same
    /// `Grid`. If they have different `Grids`, the `Grid` from the current
    /// `GridFunction` will be used.
    ///
    /// ## Example use case
    /// Suppose that we have two `GridFunctions` `grid_func_1` and `grid_func_2`
    /// and we want to calculate their product. The code below does this.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 2.0);
    /// let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 3.0);
    /// let grid_func_difference = grid_func_1.multiply(&grid_func_2);
    /// ```
    ///
    pub fn multiply(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x * y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function divide
    ///
    /// ## Description
    /// `divide` divides the current `GridFunction` by the `GridFunction`
    /// `grid_func` and returns the result.
    ///
    /// The current `GridFunction` and `grid_func` should both have the same
    /// `Grid`. If they have different `Grids`, the `Grid` from the current
    /// `GridFunction` will be used.
    ///
    /// ## Example use case
    /// Suppose that we have two `GridFunctions` `grid_func_1` and `grid_func_2`
    /// and we want to calculate their ratio. The code below does this.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 2.0);
    /// let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 3.0);
    /// let grid_func_difference = grid_func_1.divide(&grid_func_2);
    /// ```
    ///
    pub fn divide(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x / y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function scale
    ///
    /// ## Description
    /// `scale` multiplies the current `GridFunction` by a number `scalar` and
    /// returns the result.
    ///
    /// ## Example use case
    /// Suppose that we have a `GridFunction` `grid_func` and we want to scale
    /// it by a factor of 2. The code below does this.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 5.0, 6);
    /// let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
    /// let scaled_grid_func = grid_func.scale(2.0);
    /// ```
    ///
    pub fn scale(self: &Self, scalar: f64) -> Self {
        let grid = self.grid.clone();
        let function_values = self.function_values.clone();

        // Iterates over all the elements in function_values multiplies each
        // value by scalar
        let function_values: Vec<f64> =
            function_values.iter().map(|x| scalar * x).collect();

        GridFunction {
            grid,
            function_values,
        }
    }
}
