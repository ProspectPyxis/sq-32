# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- New `Error` enum to handle all program errors

### Changed
- Program now uses a superset of the Hub-2.1 protocol, changing from original syntax
- Use new `Worker` struct instead of `Container`

### Removed
- Removed `Container` struct
- Removed every old original syntax command, use Hub-2.1 commands instead

## [0.2.0] - 2021-10-12
### Added
- Moves can now be undone on the Game instance
- AI module with random AI implemented
- `go` command to order the AI to make a move
- `auto\_go` config option to have the AI go immediately on its turn
- Functions and commands to get the FEN and partial PDN of a game
- `move history` function and command to get previous moves made in the game
- `move undo` function and command to undo previous moves
- `rewind` function and command to get a board's previous state
- `exit` command to safely exit the bot

### Fixed
- Fixed a bug where pieces won't promote properly at the end of a capture chain

### Changed
- Major refactoring, fix most clippy style errors

## [0.1.0] - 2021-10-08
### Added
- New board module
- Function to set pieces on the board one by one
- Function to set the board based on a draughts FEN string
- Functions to fetch/make moves on board
- Game struct to hold all game data
- Game can check winners/losers
- Command line interface for interacting with the game
