# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Moves can now be undone on the Game instance
- AI module with random AI implemented
- New go command to order the AI to make a move
- New auto\_go config option to have the AI go immediately on its turn
- New function and command to get the FEN of a game

## Fixed
- Fixed a bug where pieces won't promote properly at the end of a capture chain

## [0.1.0] - 2021-10-08
### Added
- New board module
- Function to set pieces on the board one by one
- Function to set the board based on a draughts FEN string
- Functions to fetch/make moves on board
- Game struct to hold all game data
- Game can check winners/losers
- Command line interface for interacting with the game
