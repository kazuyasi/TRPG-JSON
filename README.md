# TRPG-JSON Repository

A JSON-based data management system for TRPG (Tabletop Role-Playing Game) game systems.

## Overview

This repository provides tools for managing and querying TRPG game data (such as monsters, characters, and items) in JSON format. It includes a command-line interface for searching and filtering game data.

## Usage

This is a command-line tool for managing TRPG game data in JSON format.

### Basic Commands

```bash
# Search for a monster by name
gm find "monster name"

# List monsters matching a pattern
gm list "pattern"

# Query with filters
gm select -l 6 -c "Category"

# Add a new monster
gm add monster.json

# Delete a monster
gm delete "monster name"
```
