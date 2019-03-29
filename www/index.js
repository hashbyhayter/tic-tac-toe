import { Game } from "tic-tac-toe";

let game = Game.new();
let player = 0;

const cells = Array.from(document.querySelectorAll("td"));
const cross = '<img class="cross" src="assets/cross.svg"/>';
const zero = '<img src="assets/nought.svg"/>';

const turnsRemaining = () => (
    game.get_board().filter(x => x === 0).length
);

const refreshBoard = () => {
    let board = game.get_board();
    let status = game.check_status();
    for (let i = 0; i < cells.length; i++) {
        if (board[i] == 1) {
            cells[i].innerHTML = cross;
        } else if (board[i] == 2) {
            cells[i].innerHTML = zero;
        } else {
            cells[i].innerHTML = '';
        }
    }
    if (status === 0 && turnsRemaining() === 0) {
        document.getElementById("winner").innerText = "Draw"
    } else if (status === player) {
        document.getElementById("winner").innerText = "You Win 🎉"
    } else if (status !== player && status !== 0) {
        document.getElementById("winner").innerText = "You Lose 😭"
    }
}

for (let i = 0; i < cells.length; i++) {
    cells[i].addEventListener("click", event => {
        if (event.target.id == "" || game.check_status() !== 0) {
            return;
        }
        let coor = event.target.id.split("-");
        game.play(coor[0], coor[1]);
        refreshBoard();
        if (game.check_status() == 0 && turnsRemaining() > 0) {
            game.next_turn();
        }
        refreshBoard();
    });
}

document.getElementById("reset").addEventListener("click", event => {
    game = Game.new();
    document.querySelector("table").classList.add("hidden");
    document.getElementById("reset").classList.add("hidden");
    document.getElementById("buttons").classList.remove("hidden");
    document.getElementById("winner").innerText = ""
});

const start = () => {
    document.querySelector("table").classList.remove("hidden");
    document.getElementById("reset").classList.remove("hidden");
    document.getElementById("buttons").classList.add("hidden");
}

document.getElementById("ai-first").addEventListener("click", event => {
    start();
    player = 2;
    game.next_turn();
    refreshBoard();
});

document.getElementById("me-first").addEventListener("click", event => {
    start();
    player = 1;
    refreshBoard();
});