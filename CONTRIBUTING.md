# Contributing to this Project

Thank you for your interest in contributing to this SDK! This document provides a basic guide for contributions, with an emphasis on following the Conventional Commits standard.

## Prerequisites

- **Language Version**: Ensure you have the latest version of Rust installed.
- **Code Style**: Follow the code style outlined in this project.
- **Testing**: Run all tests and ensure they pass before submitting your changes.

## How to Contribute

1. **Fork the Repository**: Create a fork of this repository and clone it locally.
2. **Create a Branch**: Make a new branch for your feature or bug fix.
3. **Write and Test Your Code**: Implement your changes, making sure to test thoroughly.
4. **Use Conventional Commits**: Structure your commit messages following the Conventional Commits specification (see below).
5. **Push and Submit a PR**: Push your branch and submit a pull request. Provide a clear description of your changes.

## Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/) to structure commit messages. This ensures clarity in the commit history and helps automate releases.

### Commit Message Format

Each commit message should follow this format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Commit Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Code style changes (e.g., whitespace, formatting, missing semi-colons)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding or correcting tests
- **chore**: Changes to build process or auxiliary tools

#### Example Commit Messages

- `feat(auth): add login functionality`
- `fix(api): correct endpoint URL`
- `docs: update contribution guidelines`
- `chore: update dependencies`

Thank you for contributing, and please feel free to reach out if you have questions!
