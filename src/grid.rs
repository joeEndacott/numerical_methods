/// # Grid
///
/// ## Description
/// `Grid` represents a grid of points in 1D. The coordinate of each point
/// corresponds to an element in the vector `grid_points`.
///
/// ## Example use case
/// ```
/// let grid_points = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// let grid = Grid { grid_points };
/// ```
///
#[derive(Debug, Clone)]
pub struct Grid {
    pub grid_points: Vec<f64>,
}

impl Grid {
    /// # New uniform grid
    ///
    /// ## Description
    /// `new_uniform_grid` creates a uniform `Grid` of `num_points` points
    /// between `start_point` and `end_point` inclusive.
    ///
    /// If `start_point` is greater than or equal to `end_point` or `num_points`
    /// is equal to 0, the function returns an empty vector.
    ///
    /// If `num_points` is equal to 1, the function returns a vector containing
    /// only `start_point`.
    ///
    /// ## Example use case
    /// Suppose that we want to create a uniform grid with 11 points between
    /// 0.0 and 1.0 inclusive. The code below does this.
    /// ```
    /// let grid = Grid::new_uniform_grid(0.0, 1.0, 11);
    /// ```
    ///
    pub fn new_uniform_grid(
        start_point: f64,
        end_point: f64,
        num_points: usize,
    ) -> Self {
        // Error handling for edge case when start_point is greater than or
        // equal to end_point or num_points is equal to 0.
        // For this case, the function returns an empty vector.
        if start_point >= end_point || num_points == 0 {
            let grid_points: Vec<f64> = vec![];
            return Grid { grid_points };
        }

        // Error handling for edge case when num_points is equal to 1.
        // For this case, the function returns a vector containing only
        // start_point.
        if num_points == 1 {
            let grid_points: Vec<f64> = vec![start_point];
            return Grid { grid_points };
        }

        // step_size is the distance between adjacent grid points
        let step_size = (end_point - start_point) / (num_points as f64 - 1.0);

        // Creates a vector containing the grid points
        let grid_points: Vec<f64> = (0..num_points)
            .map(|i| start_point + (i as f64) * step_size)
            .collect();

        Grid { grid_points }
    }
}
