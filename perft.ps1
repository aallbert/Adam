### Use for testing
### In console type: perftree powershell.exe -File .\perft.ps1

# Define the path to your Chessbot executable
$CHESSBOT_EXEC = ".\target\release\Chess_Bot_Battle.exe"

# Check if the executable exists
if (-not (Test-Path $CHESSBOT_EXEC -PathType Leaf)) {
    Write-Host "Error: Chessbot executable not found at '$CHESSBOT_EXEC'." -ForegroundColor Red
    Write-Host "Please ensure you have built your Rust project: 'cargo build --release'" -ForegroundColor Yellow
    Write-Host "And verified the correct path and name of the executable." -ForegroundColor Yellow
    Exit 1
}

# Extract arguments
$DEPTH = $args[0]
$FEN = $args[1]
$MOVES = $args[2]

# Validate arguments
if ([string]::IsNullOrEmpty($DEPTH) -or [string]::IsNullOrEmpty($FEN)) {
    Write-Host "Usage: .\your-perft.ps1 <depth> <fen> [moves]" -ForegroundColor Cyan
    Write-Host "  <depth>: The depth for the perft calculation (integer)." -ForegroundColor Cyan
    Write-Host "  <fen>: The FEN (Forsyth-Edwards Notation) string of the starting position." -ForegroundColor Cyan
    Write-Host "  [moves]: Optional. A comma-separated list of UCI moves to limit the perft calculation to (e.g., ""e2e4,g1f3"")." -ForegroundColor Cyan
    Exit 1
}

# Executing chessbot
& $CHESSBOT_EXEC $DEPTH $FEN $MOVES