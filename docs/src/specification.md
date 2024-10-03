## General Specification for the project !   
> This is a temporary initial specification to help achieve the first user journey, consisting of opening a file editing and saving it into the filesystem !    
--- 
> This specification is still under maintaining any ideas please refer it to issue

### General Directory of the project !

The project adopts the notion of workspaces where it will be more oragnized during development.

   
```
Diwan
├── Cargo.lock
├── Cargo.toml
├── diwan-lib
│   ├── Cargo.toml
│   └── src
│       ├── commands
│       │   ├── edit_commands.rs
│       │   ├── file_commands.rs
│       │   └── mod.rs
│       ├── debuger
│       │   ├── mode_enum.rs
│       │   ├── mod.rs
│       │   └── statusbar_struct.rs
│       ├── editor
│       │   ├── editor.rs
│       │   ├── mod.rs
│       │   ├── renderer.rs
│       │   └── text_buffer.rs
│       ├── lib.rs
│       ├── screen
│       │   ├── keymap.rs
│       │   ├── mainscreen.rs
│       │   ├── mod.rs
│       │   ├── statusbar.rs
│       │   ├── ui.rs
│       │   └── widget.rs
│       └── utils
│           ├── file_manager.rs
│           ├── mod.rs
│           └── syntax_highlighter.rs
├── diwan-term
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── license
└── README.md 
```

## Explanation

- **diwan-lib/src/main.rs**: The entry point of the `diwan-term` application. This file contains the `main` function and initializes the terminal application.
  
- **diwan-lib/src/commands**: Contains modules related to commands:
  - **mod.rs**: The module file that re-exports other command modules.
  - **file_commands.rs**: Handles file-related commands (open, save, etc.).
  - **edit_commands.rs**: Manages commands related to text editing (insert, delete, etc.).

- **diwan-lib/src/debugger**: Contains modules for debugging:
  - **mod.rs**: The module file that re-exports other debugger modules.
  - **mode_enum.rs**: Defines different debugging modes.
  - **statusbar_struct.rs**: Represents the structure of the status bar used for debugging.

- **diwan-lib/src/editor**: Contains the core components of the editor:
  - **mod.rs**: The module file that re-exports other editor modules.
  - **editor.rs**: Handles the main editor logic.
  - **text_buffer.rs**: Manages the text buffer and cursor positioning.
  - **renderer.rs**: Responsible for rendering the editor content to the terminal.

- **diwan-lib/src/screen**: Manages screen-related components:
  - **mod.rs**: The module file that re-exports other screen modules.
  - **keymap.rs**: Manages key mappings for the screen.
  - **mainscreen.rs**: Handles the main screen rendering.
  - **statusbar.rs**: Manages the status bar at the bottom of the screen.
  - **ui.rs**: Contains user interface logic for the editor.
  - **widget.rs**: Defines UI widgets used within the editor.

- **diwan-lib/src/utils**: Contains utility modules:
  - **mod.rs**: The module file that re-exports other utility modules.
  - **file_manager.rs**: Handles file operations.
  - **syntax_highlighter.rs**: Applies syntax highlighting to the text buffer.

- **diwan-lib/src/lib.rs**: The main library file for the `diwan-lib` crate. It defines common functionality shared across the project.

- **diwan-term/src/main.rs**: The entry point for the terminal application. This file contains the `main` function and sets up the terminal-based editor.

## General Taxonomy ! 

1. Structs

    - Prefix with Editor: For structs related to the core editor functionality.
    - Prefix with Command: For structs related to commands.
    - Prefix with File: For structs related to file operations.
    - Prefix with Text: For structs related to text manipulation.
    - Prefix with Render: For structs related to rendering.
    - Prefix with Syntax: For structs related to syntax highlighting.

2. Files

    - mod.rs: For module files that re-export other modules.
    - file_commands.rs: For file-related commands.
    - edit_commands.rs: For text editing commands.
    - text_buffer.rs: For text buffer management.
    - renderer.rs: For rendering the text buffer.
    - file_manager.rs: For file operations.
    - syntax_highlighter.rs: For syntax highlighting.

3. Functions

    - Prefix with handle_: For functions that handle specific actions.
    - Prefix with render_: For functions that render content.
    - Prefix with process_: For functions that process input or commands.
    - Prefix with load_: For functions that load data.
    - Prefix with save_: For functions that save data.
    - Prefix with apply_: For functions that apply changes.

4. Traits

    - Prefix with EditorTrait: For traits related to the core editor functionality.
    - Prefix with CommandTrait: For traits related to commands.
    - Prefix with FileTrait: For traits related to file operations.
    - Prefix with TextTrait: For traits related to text manipulation.
    - Prefix with RenderTrait: For traits related to rendering.
    - Prefix with SyntaxTrait: For traits related to syntax highlighting.

5. Constants

    - Prefix with MAX_: For maximum values.
    - Prefix with MIN_: For minimum values.
    - Prefix with DEFAULT_: For default values.
    - Prefix with ERROR_: For error codes or messages.

6. Macros

    - Prefix with macro_: For general-purpose macros.
    - Prefix with log_: For logging-related macros.
    - Prefix with assert_: For assertion-related macros.
    - Prefix with debug_: For debugging-related macros.

## Versioning ! 
> using SemVer !   
### Semantic Versioning (SemVer)

- SemVer defines three components of a version number:

    - MAJOR: Incremented for incompatible API changes.
    - MINOR: Incremented for adding functionality in a backwards-compatible manner.
    - PATCH: Incremented for backwards-compatible bug fixes.

- Additionally, pre-release versions and build metadata can be appended to the version number:

    - Pre-release: Denoted by a hyphen and a series of dot-separated identifiers (e.g., 1.0.0-alpha, 1.0.0-beta.2).
    - Build metadata: Denoted by a plus sign and a series of dot-separated identifiers (e.g., 1.0.0+build.123).

- When to Increment Versions

    - MAJOR Version (e.g., 1.0.0 to 2.0.0):
        - Increment when you make incompatible API changes.
        - This indicates that users may need to update their code to use the new version.

    - MINOR Version (e.g., 1.0.0 to 1.1.0):
        - Increment when you add functionality in a backwards-compatible manner.
        - This indicates new features that do not break existing functionality.

    - PATCH Version (e.g., 1.0.0 to 1.0.1):
        - Increment when you make backwards-compatible bug fixes.
        - This indicates that the release includes fixes but no new features or breaking changes.

- Setting Up Your Versioning System

    - Initial Version (e.g., 0.0.1):
        - Start with a version like 0.0.1 for initial development.
        - Use 0.x.x versions for pre-release, unstable, or experimental features.

    - Pre-release Versions (e.g., 1.0.0-alpha):
        - Use pre-release versions to indicate that the software is not yet stable.
        - Common pre-release identifiers include alpha, beta, rc (release candidate).

    - Stable Release (e.g., 1.0.0):
        - Increment to 1.0.0 when you consider the project stable and ready for production use.
        - This indicates that the API is stable and that breaking changes will be minimized.

    - Subsequent Releases:
        - Increment the PATCH version for bug fixes.
        - Increment the MINOR version for new features.
        - Increment the MAJOR version for breaking changes.

## Idioms and Code Practices !  
### Coding Idioms ! 

- TDD at the core : 
  - Core Tests for a feature [X] must be written first ! 
  - Push inital deltas to the remote origin without any initial business logic ! 
  - Write business code for the feature [X] 
  - Push the code and run the CI/CD pipeline and it must succed 
  - Code must be reviewed by other members and merged later ! 

- Hot Documentation logging : 
  - Write inline comments for small subtle explanation of some line of codes !  
  - For each new module , write new module documentation to explain the business logic !   
  - When adding new logic or altering the business logic code , a modification or extension of the module documentation is required !
