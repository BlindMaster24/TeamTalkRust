# Gemini Project Context: TeamTalk Registration System

## Project Overview

This project is a comprehensive bot solution for self-registration on TeamTalk 5 servers. It provides two main interfaces for users to register: a Telegram bot and a FastAPI-based web portal. The goal is to streamline user onboarding for TeamTalk communities by allowing users to register themselves, with an optional admin approval step.

The project is built with Python and utilizes several key technologies:
- **Telegram Bot:** `aiogram` is used for the Telegram bot interface.
- **Web Application:** `FastAPI` provides the web registration portal.
- **TeamTalk Integration:** `py-talk-ex` is used for interacting with the TeamTalk 5 SDK.
- **Database:** `SQLModel` (on top of SQLAlchemy and aiosqlite) is used for database operations. The database stores user registrations, pending approvals, and other relevant data.
- **Configuration:** `Pydantic Settings` is used to manage configuration from a `config.toml` file and environment variables.
- **Dependency Management:** `uv` is used for package installation and environment management.
- **Linting and Formatting:** `ruff` is used for linting and formatting the codebase.
- **Type Checking:** `mypy` is used for static type checking.

The project follows a modular architecture, with the core logic, Telegram bot, and FastAPI app separated into different directories under the `bot` directory. The main entry point is `run.py`, which orchestrates the startup of all components.

## Building and Running

1.  **Install Dependencies:**
    ```bash
    uv sync --all-extras
    ```

2.  **Compile Localization Files:**
    ```bash
    uv run pybabel compile -D messages -d locales
    ```

3.  **Configure the Application:**
    -   Create a `config.toml` file (you can copy `config.toml.example`).
    -   Fill in the required values, such as `tg_bot_token`, `admin_ids`, and TeamTalk server details.

4.  **Run the Application:**
    ```bash
    uv run run.py
    ```

## Development Conventions

-   **Dependency Management:** All dependencies are managed in `pyproject.toml` and locked with `uv.lock`. Use `uv sync` to install or update dependencies.
-   **Linting and Formatting:** The project uses `ruff` for linting and formatting. The configuration is in `pyproject.toml`.
-   **Type Checking:** The project uses `mypy` for static type checking. The configuration is in `pyproject.toml`.
-   **Localization:** The project uses `Babel` for localization. The `manage-locales.py` script is provided for managing localization files.
-   **Commit Messages:** Commit messages should be descriptive and follow conventional formats.
-   **Branching:** The project uses a `refactor` branch, suggesting a feature-branching workflow.
