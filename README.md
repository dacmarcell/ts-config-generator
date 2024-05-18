<h1 align="center">
  <a href='#'><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="600px"/></a>
  <br>
  <br>
  <div>
    <a href="https://github.com/marcelldac/GameRave/issues">
      <img src="https://img.shields.io/github/issues/marcelldac/GameRave?color=fab387&labelColor=303446&style=for-the-badge">
    </a>
    <a href="https://github.com/marcelldac/GameRave/stargazers">
      <img src="https://img.shields.io/github/stars/marcelldac/GameRave?color=ca9ee6&labelColor=303446&style=for-the-badge">
    </a>
    <a href="https://github.com/marcelldac/GameRave">
      <img src="https://img.shields.io/github/repo-size/marcelldac/GameRave?color=ea999c&labelColor=303446&style=for-the-badge">
    </a>
  </div>
</h1>

# TypeScript Config Generator

Welcome to the **TypeScript Config Generator**! This repository contains a simple yet powerful command-line tool designed to help you quickly generate `tsconfig.json` files tailored to various environments and setups.

## Features

- **Interactive CLI**: Uses a colorful and intuitive CLI to guide you through the process of selecting a TypeScript configuration template.
- **Multiple Templates**: Choose from several predefined `tsconfig.json` templates, including:
  - Recommended
  - Node 21
  - Node 20
  - Node 19
  - Bun
- **ASCII Art**: Enjoy a fun ASCII art welcome message.

## Usage

To use this tool, simply run the executable. You will be presented with an interactive menu to choose your preferred TypeScript configuration template.

### Prerequisites

- Rust (to compile the tool)
- `dialoguer` crate for interactive CLI
- `console` crate for styling the CLI

### Installation

Clone the repository and navigate into its directory:

```bash
git clone https://github.com/yourusername/tsconfig-generator.git
cd tsconfig-generator
