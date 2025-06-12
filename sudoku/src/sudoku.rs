/// Validates if a Sudoku solution is correct given an initial state.
/// Both inputs are strings representing a 9x9 grid (81 characters, digits 1-9 or placeholders).
/// Returns true if the solution is valid and respects the initial state, false otherwise.
pub fn is_valid_sudoku_solution(initial: &str, solution: &str) -> bool {
    // Get cleaned grids
    let initial_grid = match clean_grid(initial) {
        Some(grid) => grid,
        None => return false, // Invalid input length
    };
    let solution_grid = match clean_grid(solution) {
        Some(grid) => grid,
        None => return false, // Invalid input length
    };

    // Check if solution respects initial state
    for (init, sol) in initial_grid.iter().zip(solution_grid.iter()) {
        if *init != '.' && *init != '0' && *init != ' ' && *init != *sol {
            return false; // Solution doesn't match initial state's filled cells
        }
        if !('1'..='9').contains(sol) {
            return false; // Solution must contain only digits 1-9
        }
    }

    // Convert solution to a 9x9 grid for easier checking
    let mut grid = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            grid[i][j] = solution_grid[i * 9 + j].to_digit(10).unwrap_or(0) as u8;
            if grid[i][j] < 1 || grid[i][j] > 9 {
                return false; // Invalid digit
            }
        }
    }

    // Check rows
    for i in 0..9 {
        let mut seen = [false; 10]; // Index 0 unused, 1-9 for digits
        for j in 0..9 {
            let num = grid[i][j];
            if seen[num as usize] {
                return false; // Duplicate in row
            }
            seen[num as usize] = true;
        }
    }

    // Check columns
    for j in 0..9 {
        let mut seen = [false; 10];
        for i in 0..9 {
            let num = grid[i][j];
            if seen[num as usize] {
                return false; // Duplicate in column
            }
            seen[num as usize] = true;
        }
    }

    // Check 3x3 subgrids
    for block in 0..9 {
        let mut seen = [false; 10];
        let start_row = (block / 3) * 3;
        let start_col = (block % 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                let num = grid[start_row + i][start_col + j];
                if seen[num as usize] {
                    return false; // Duplicate in subgrid
                }
                seen[num as usize] = true;
            }
        }
    }

    true // Solution is valid
}

/// Helper to clean input strings: keep only digits and placeholders, ensure 81 chars
fn clean_grid(input: &str) -> Option<Vec<char>> {
    let cleaned: Vec<char> = input
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.' || *c == '0' || *c == ' ')
        .collect();
    if cleaned.len() == 81 {
        Some(cleaned)
    } else {
        None
    }
}
