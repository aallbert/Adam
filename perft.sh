CHESSBOT_EXEC="./target/release/Chess_Bot_Battle"

# Check if the executable exists
if [ ! -f "$CHESSBOT_EXEC" ]; then
    echo "Error: Chessbot executable not found at '$CHESSBOT_EXEC'."
    echo "Please ensure you have built your Rust project: 'cargo build --release'"
    echo "And verified the correct path and name of the executable."
    exit 1
fi

# Extract arguments
DEPTH="$1"
FEN="$2"
MOVES="$3" # This will be empty if not provided

# Validate arguments
if [ -z "$DEPTH" ] || [ -z "$FEN" ]; then
    echo "Usage: ./your-perft.sh <depth> <fen> [moves]"
    echo "  <depth>: The depth for the perft calculation (integer)."
    echo "  <fen>: The FEN (Forsyth-Edwards Notation) string of the starting position."
    echo "  [moves]: Optional. A comma-separated list of UCI moves to limit the perft calculation to (e.g., \"e2e4,g1f3\")."
    exit 1
fi

# Executing chessbot
"$CHESSBOT_EXEC" "$DEPTH" "$FEN" "$MOVES"