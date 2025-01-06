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
    /// ## Todo
    /// Change handling of edge case where the two `GridFunctions` have
    /// different `Grids`. Currently, the `Grid` from the current `GridFunction`
    /// is used. This may not be the best way to handle this case.
    ///
    pub fn add(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference: i32 =
            (function_values_1.len() as i32) - (function_values_2.len() as i32);

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
    /// ## Todo
    /// Change handling of edge case where the two `GridFunctions` have
    /// different `Grids`. Currently, the `Grid` from the current `GridFunction`
    /// is used. This may not be the best way to handle this case.
    ///
    pub fn subtract(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference: i32 =
            (function_values_1.len() as i32) - (function_values_2.len() as i32);

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
    /// ## Todo
    /// Change handling of edge case where the two `GridFunctions` have
    /// different `Grids`. Currently, the `Grid` from the current `GridFunction`
    /// is used. This may not be the best way to handle this case.
    ///
    pub fn multiply(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference: i32 =
            (function_values_1.len() as i32) - (function_values_2.len() as i32);

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
    /// ## Todo
    /// Change handling of edge case where the two `GridFunctions` have
    /// different `Grids`. Currently, the `Grid` from the current `GridFunction`
    /// is used. This may not be the best way to handle this case.
    ///
    pub fn divide(self: &Self, grid_func: &GridFunction) -> Self {
        let grid = self.grid.clone();

        let function_values_1 = self.function_values.clone();
        let mut function_values_2 = grid_func.function_values.clone();
        let length_difference: i32 =
            (function_values_1.len() as i32) - (function_values_2.len() as i32);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn test_arithmetic_operations() {
        let grid = Grid::new_uniform_grid(0.0, 1.0, 6);
        let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 4.0);
        let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 2.0);

        // Test addition.
        let grid_func_sum = grid_func_1.add(&grid_func_2);
        assert_eq!(
            grid_func_sum.function_values,
            vec![6.0; 6],
            "Addition failed."
        );

        // Test subtraction.
        let grid_func_difference = grid_func_1.subtract(&grid_func_2);
        assert_eq!(
            grid_func_difference.function_values,
            vec![2.0; 6],
            "Subtraction failed."
        );

        // Test multiplication.
        let grid_func_product = grid_func_1.multiply(&grid_func_2);
        assert_eq!(
            grid_func_product.function_values,
            vec![8.0; 6],
            "Multiplication failed."
        );

        // Test division.
        let grid_func_quotient = grid_func_1.divide(&grid_func_2);
        assert_eq!(
            grid_func_quotient.function_values,
            vec![2.0; 6],
            "Division failed."
        );
    }

    #[test]
    fn test_arithmetic_operations_different_grids() {
        // Case 1: grid_func_1 has more elements than grid_func_2.
        let grid = Grid::new_uniform_grid(0.0, 1.0, 6);
        let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 4.0);
        let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 2.0);
        let mut grid_func_2_short = grid_func_2.clone();
        grid_func_2_short.function_values.pop();

        // Test addition.
        let grid_func_sum = grid_func_1.add(&grid_func_2_short);
        assert_eq!(
            grid_func_sum.function_values,
            vec![6.0, 6.0, 6.0, 6.0, 6.0, 4.0],
            "Case 1 addition failed."
        );

        // Test subtraction.
        let grid_func_difference = grid_func_1.subtract(&grid_func_2_short);
        assert_eq!(
            grid_func_difference.function_values,
            vec![2.0, 2.0, 2.0, 2.0, 2.0, 4.0],
            "Case 1 subtraction failed."
        );

        // Test multiplication.
        let grid_func_product = grid_func_1.multiply(&grid_func_2_short);
        assert_eq!(
            grid_func_product.function_values,
            vec![8.0, 8.0, 8.0, 8.0, 8.0, 0.0],
            "Case 1 multiplication failed."
        );

        // Test division.
        let grid_func_quotient = grid_func_1.divide(&grid_func_2_short);
        assert_eq!(
            grid_func_quotient.function_values,
            vec![2.0, 2.0, 2.0, 2.0, 2.0, f64::INFINITY],
            "Case 1 division failed."
        );

        // Case 2: grid_func_2 has more elements than grid_func_1.
        let grid = Grid::new_uniform_grid(0.0, 1.0, 6);
        let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 4.0);
        let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 2.0);
        let mut grid_func_1_short = grid_func_1.clone();
        grid_func_1_short.function_values.pop();

        // Test addition.
        let grid_func_sum = grid_func_1_short.add(&grid_func_2);
        assert_eq!(
            grid_func_sum.function_values,
            vec![6.0; 5],
            "Case 2 addition failed."
        );

        // Test subtraction.
        let grid_func_difference = grid_func_1_short.subtract(&grid_func_2);
        assert_eq!(
            grid_func_difference.function_values,
            vec![2.0; 5],
            "Case 2 subtraction failed."
        );

        // Test multiplication.
        let grid_func_product = grid_func_1_short.multiply(&grid_func_2);
        assert_eq!(
            grid_func_product.function_values,
            vec![8.0; 5],
            "Case 2 multiplication failed."
        );

        // Test division.
        let grid_func_quotient = grid_func_1_short.divide(&grid_func_2);
        assert_eq!(
            grid_func_quotient.function_values,
            vec![2.0; 5],
            "Case 2 division failed."
        );
    }

    #[test]
    fn test_arithmetic_operations_empty_grid_function() {
        let grid = Grid::new_uniform_grid(0.0, 1.0, 0);
        let grid_func_1 = GridFunction::new_constant_grid_function(&grid, 4.0);
        let grid_func_2 = GridFunction::new_constant_grid_function(&grid, 2.0);

        // Test addition.
        let grid_func_sum = grid_func_1.add(&grid_func_2);
        assert_eq!(grid_func_sum.function_values, vec![], "Addition failed.");

        // Test subtraction.
        let grid_func_difference = grid_func_1.subtract(&grid_func_2);
        assert_eq!(
            grid_func_difference.function_values,
            vec![],
            "Subtraction failed."
        );

        // Test multiplication.
        let grid_func_product = grid_func_1.multiply(&grid_func_2);
        assert_eq!(
            grid_func_product.function_values,
            vec![],
            "Multiplication failed."
        );

        // Test division.
        let grid_func_quotient = grid_func_1.divide(&grid_func_2);
        assert_eq!(
            grid_func_quotient.function_values,
            vec![],
            "Division failed."
        );
    }

    #[test]
    fn test_divide_by_zero() {
        // Divides the zero function by grid_func
        let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
        let grid_func = GridFunction::new_constant_grid_function(&grid, 4.0);
        let grid_func_zero =
            GridFunction::new_constant_grid_function(&grid, 0.0);
        let grid_func_quotient = grid_func_zero.divide(&grid_func);
        assert_eq!(
            grid_func_quotient.function_values,
            vec![0.0; 11],
            "Division of zero function by GridFunction failed."
        );

        // Divides grid_func by the zero function.
        let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
        let grid_func = GridFunction::new_constant_grid_function(&grid, 4.0);
        let grid_func_zero =
            GridFunction::new_constant_grid_function(&grid, 0.0);
        let grid_func_quotient = grid_func.divide(&grid_func_zero);
        assert!(
            grid_func_quotient
                .function_values
                .iter()
                .all(|&x| x.is_infinite()),
            "Division of GridFunction by zero function failed."
        );

        // Divides the zero function by itself.
        let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
        let grid_func_zero =
            GridFunction::new_constant_grid_function(&grid, 0.0);
        let grid_func_quotient = grid_func_zero.divide(&grid_func_zero);
        assert!(
            grid_func_quotient
                .function_values
                .iter()
                .all(|&x| x.is_nan()),
            "Division of zero function by zero function failed."
        );
    }

    #[test]
    fn test_scale() {
        // scales grid_func by 2
        let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
        let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
        let scaled_grid_func = grid_func.scale(2.0);
        assert_eq!(
            scaled_grid_func.function_values,
            vec![4.0; 11],
            "Scaling by 2 failed."
        );

        // scales grid_func by 0
        let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
        let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
        let scaled_grid_func = grid_func.scale(0.0);
        assert_eq!(
            scaled_grid_func.function_values,
            vec![0.0; 11],
            "Scaling by 0 failed."
        );

        // scales grid_func by -2
        let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
        let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
        let scaled_grid_func = grid_func.scale(-2.0);
        assert_eq!(
            scaled_grid_func.function_values,
            vec![-4.0; 11],
            "Scaling by -2 failed."
        );
    }

    #[test]
    fn test_scale_empty_grid_function() {
        // scales an empty grid function by 2
        let grid = Grid::new_uniform_grid(0.0, 1.0, 0);
        let grid_func = GridFunction::new_constant_grid_function(&grid, 2.0);
        let scaled_grid_func = grid_func.scale(2.0);
        assert_eq!(scaled_grid_func.function_values, vec![], "Scaling failed.");
    }
}
