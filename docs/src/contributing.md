
# Contributing to the Project

To maintain consistency and quality across contributions, please follow the guidelines outlined below.

## Contribution Workflow

### 1. Clone the Repository
If you're new to GitHub, you can follow these instructions:
1. **Clone the repository**: Click the "Code" button on the repository page and copy the URL. Then run the following command in your terminal:

   ```bash
   git clone https://github.com/Diwan-editor/Diwan.git
   ```

2. **Navigate into the project directory**:

   ```bash
   cd Diwan
   ```

### 2. Create a New Branch
Create a new branch for your contribution using the following convention:

- Branch name format: `<scope>_<name>`
  - For example: `SCR_add_text_zoom_feature`

### 3. Make Changes
Make your changes in the new branch.

### 4. Commit Changes
When committing your changes, use the following PR naming convention:

- PR name format: `[scope]type:name`
  - For example: `[SCR]feat:add text zoom feature`

### 5. Open a Pull Request (PR)
1. Push your branch to the repository:

   ```bash
   git push origin <your-branch-name>
   ```

2. Navigate to the repository and click on the "Pull Requests" tab.
3. Click the "New Pull Request" button.
4. Select your branch and ensure it is compared against the `main` branch.
5. Provide a clear description of your changes.

### 6. Get Approval
Your PR requires at least **1 approval** before it can be merged.

### 7. Rebase, Squash, and Merge
Once your PR is approved, you need to :
1. **Rebase** your branch on the latest `main` branch, either using github UI (if available) or by running the following commands:

   ```bash
   git fetch main
   git checkout <your-branch-name>
   git rebase main
   git push origin <your-branch-name> --force
   ```

2. **Squash & merge** your commits into a single commit by clicking the "Squash and Merge" button on the PR page. the button will be available once you have at least one approval.

## Scopes and Types

### Scopes

| Scope                                | Abbreviation |
|--------------------------------------|--------------|
| Screen                               | SCR          |
| Status line                          | STL          |
| Commands interpreter + args + hotkeys| CIA          |
| File management                      | FM           |
| Docs                                 | DOCS         |
| Other                                | OTHER        |

### Types

| Type    | Description                                    |
|---------|------------------------------------------------|
| Feat    | Feature to be merged to main                   |
| Fix     | Fix to a bug or security risk                  |
| Stage   | Release a stage version that is “done”         |
| Release | Major version release                          |

## Additional Notes

- Please ensure your code is well-documented and adheres to the project's specification.
- If you're unsure about anything, feel free to ask for help by opening an issue or reaching out to the maintainers.

We look forward to your contributions!
