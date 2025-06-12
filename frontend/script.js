// Store the initial state of the current puzzle.
let currentPuzzleInitialState = ""; // Will be set by generateNewPuzzle on load

// Helper functions for Sudoku generation
function shuffleArray(array) {
    for (let i = array.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [array[i], array[j]] = [array[j], array[i]];
    }
}

function isValidPlacement(board, row, col, num) {
    // Check row: Ensure number is not already in the current row
    for (let x = 0; x < 9; x++) {
        if (board[row * 9 + x] === num) {
            return false;
        }
    }
    // Check column: Ensure number is not already in the current column
    for (let y = 0; y < 9; y++) {
        if (board[y * 9 + col] === num) {
            return false;
        }
    }
    // Check 3x3 subgrid: Ensure number is not in the 3x3 subgrid
    const startRow = row - row % 3;
    const startCol = col - col % 3;
    for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
            if (board[(startRow + i) * 9 + (startCol + j)] === num) {
                return false;
            }
        }
    }
    return true; // Number can be placed
}

// Backtracking solver to fill a Sudoku board
// board is a 1D array of 81 numbers, 0 for empty
function solveSudoku(board) {
    for (let i = 0; i < 81; i++) {
        if (board[i] === 0) { // Find an empty cell
            let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
            shuffleArray(numbers); // Try numbers in a random order

            for (let num of numbers) {
                if (isValidPlacement(board, Math.floor(i / 9), i % 9, num)) {
                    board[i] = num; // Place the number
                    if (solveSudoku(board)) { // Recursively solve
                        return true; // Solution found
                    }
                    board[i] = 0; // Backtrack: reset cell and try next number
                }
            }
            return false; // No valid number found for this cell, trigger backtracking
        }
    }
    return true; // All cells are filled, solution found
}

// Generates a new Sudoku puzzle string (initial state)
function generateSudokuBoardString() {
    let board = Array(81).fill(0);
    solveSudoku(board); // Fill the board completely with a valid Sudoku solution

    // Remove some numbers to create a puzzle.
    // Aim for a puzzle with a reasonable number of clues (e.g., 25-35).
    // This means removing 81 - (25 to 35) = 46 to 56 cells.
    // Let's remove around 50 cells to leave 31 clues.
    let cellsToRemove = 50;
    let indices = Array.from({length: 81}, (_, k) => k); // Get all cell indices
    shuffleArray(indices); // Shuffle indices to remove cells randomly

    for (let i = 0; i < cellsToRemove; i++) {
        board[indices[i]] = 0; // Set cell to 0 (empty)
    }

    // Convert the board (0s for empty) to a string ('.' for empty)
    return board.map(num => (num === 0 ? "." : String(num))).join("");
}

function createGrid() {
    const grid = document.getElementById("sudoku-grid");
    grid.innerHTML = "";
    for (let i = 0; i < 81; i++) {
        const input = document.createElement("input");
        input.className = "cell";
        input.type = "text";
        input.maxLength = 1;
        input.pattern = "[1-9]";
        input.oninput = () => {
            if (!/[1-9]/.test(input.value)) input.value = "";
        };
        grid.appendChild(input);
    }
}

function loadPuzzle(puzzle) {
    const cells = document.querySelectorAll(".cell");
    for (let i = 0; i < 81; i++) {
        cells[i].value = puzzle[i] === "." ? "" : puzzle[i];
        cells[i].readOnly = puzzle[i] !== ".";
    }
}

function generateNewPuzzle() {
    // Generate a new Sudoku puzzle string
    currentPuzzleInitialState = generateSudokuBoardString();
    // Load the new puzzle into the grid
    loadPuzzle(currentPuzzleInitialState);
    // Clear any previous response messages
    document.getElementById("response").innerHTML = "";
}

function solveCurrentPuzzle() {
    // Convert the current puzzle string to a board array (0 for empty)
    let board = currentPuzzleInitialState.split("").map(char => (char === "." ? 0 : parseInt(char)));

    // Solve the Sudoku puzzle (solveSudoku modifies the board in place)
    if (solveSudoku(board)) {
        // Convert the solved board array back to a string
        const solutionString = board.map(num => String(num)).join("");
        // Load the solution onto the grid
        loadPuzzle(solutionString);
        document.getElementById("response").innerHTML = "Puzzle solved!";
    } else {
        // This should ideally not happen if generateSudokuBoardString always produces solvable puzzles
        document.getElementById("response").innerHTML = "Could not solve the puzzle.";
    }
}
function getSolution() {
    const cells = document.querySelectorAll(".cell");
    let solution = "";
    for (let i = 0; i < 81; i++) {
        solution += cells[i].value || ".";
    }
    return solution;
}

async function submitSolution() {
    const initialState = currentPuzzleInitialState; // Use the currently loaded puzzle's initial state
    const solution = getSolution();
    const data = {
        initial_state: initialState,
        solution: solution
    };

    // Show spinner
    const responseDiv = document.getElementById("response");
    responseDiv.innerHTML = '<span class="spinner"></span> Submitting... It may take 1 or 2 minutes.';

    try {
        const response = await fetch("http://localhost:9090/check_solution", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        });
        if (!response.ok) {
            let errorText = await response.json();
            throw new Error(errorText);
        }

        const result = await response.json();
        const batchMerkleRoot = result.batch_merkle_root
            .map(byte => byte.toString(16).padStart(2, "0"))
            .join("");
        const link = `http://localhost:4000/batches/0x${batchMerkleRoot}`;
        responseDiv.innerHTML =
            `Solution submitted! <a href="${link}" target="_blank">View Batch</a>`;
    } catch (error) {
        responseDiv.innerHTML =
            `Error: ${error.message}`;
    }
}

// Initialize grid on page load
createGrid();
// loadPuzzle(currentPuzzleInitialState); // Load the initial default puzzle
generateNewPuzzle(); // Generate and load a new random puzzle on page load
