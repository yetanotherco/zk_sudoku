// Sample initial puzzle (can be replaced with a real generator)
// const samplePuzzle = "2...8.3...6..7..84.3.5..2.9...1.54.8.........4.27.6...3.1..7.4.72..4..6...4.1...3";
const samplePuzzle = "245981376169273584837564219976125438513498627482736951391657842728349165654812793"

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
    // In a real app, fetch a new puzzle from a server or generate one
    loadPuzzle(samplePuzzle);
    document.getElementById("response").innerHTML = "";
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
    const initialState = samplePuzzle; // Replace with actual initial state
    const solution = getSolution();
    const data = {
        initial_state: initialState,
        solution: solution
    };

    // Show spinner
    const responseDiv = document.getElementById("response");
    responseDiv.innerHTML = '<span class="spinner"></span> Submitting...';

    try {
        const response = await fetch("http://localhost:9090/check_solution", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        });

        if (!response.ok) throw new Error("Server error");

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
loadPuzzle(samplePuzzle);