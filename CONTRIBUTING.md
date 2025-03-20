# **Rust Project Contribution Guide**

## **1. Forking & Cloning the Repository**

### **Forking the Repository**
1. Go to the **GitHub repository** you want to contribute to.
2. Click the **Fork** button (top-right corner).
3. GitHub will create a copy of the repo under your account.

### **Cloning the Forked Repository**
After forking, clone the repo locally:

```sh
git clone https://github.com/Dericko681/ChatMe.git
```
Replace `<your-username>` with your GitHub username and `<repo-name>` with the actual repository name.

### **Navigate into the Project**

```sh
cd chatme
```

### **Set the Upstream Remote (Original Repo) to Fetch Future Updates**

```sh
git remote add upstream https://github.com/Dericko681/ChatMe.git
git remote -v
```

Fetch and merge the latest changes from upstream:

```sh
git fetch upstream
git checkout main
git merge upstream/main
```

---

## **2. Rust Coding Standards & Naming Conventions**

### **Coding Standards**
Follow Rust’s official style guide using `cargo fmt` and `clippy`:

```sh
cargo fmt --all
cargo clippy -- -D warnings
```

- Use idiomatic Rust patterns (e.g., prefer `Result` over panics).
- Document public APIs using `///` Rust doc comments.
- Follow proper error handling with `thiserror` or `anyhow` crates.
- Write modular code by splitting logic into separate modules (`mod`).
- Use strong typing instead of `String` where possible (`struct`, `enum`).

### **Naming Conventions**

| Element           | Naming Convention | Example             |
|------------------|------------------|---------------------|
| Variables & Functions | `snake_case`    | `process_data()`   |
| Constants        | `UPPER_CASE_SNAKE` | `MAX_RETRIES`      |
| Structs & Enums  | `PascalCase`      | `struct UserProfile` |
| Traits          | `PascalCase`      | `trait Displayable` |
| Files & Modules  | `snake_case.rs`   | `auth_service.rs`   |

---

## **3. Submitting a Pull Request (PR) with atleast one reviewer**

### **Step 1: Create a Feature Branch**
Before making changes, create a new branch:

```sh
git checkout -b feature-branch-name
```

### **Step 2: Make Changes & Run Tests**
Modify the code, then format and lint:

```sh
cargo fmt --all
cargo clippy -- -D warnings
```
Run tests before pushing:

```sh
cargo test
```

### **Step 3: Commit & Push the Changes**

```sh
git add .
git commit -m "<commit message>"
git push origin feature-branch-name
```

### **Step 4: Create a Pull Request (PR)**
1. Go to your forked repo on GitHub.
2. Click **Compare & pull request**.
3. Set the base branch (e.g., `main` or `develop`).
4. Add a clear title and detailed description.
5. Assign at least **one reviewer** in the *Reviewers* section.
6. Click **Create Pull Request**.

---

## **4. Syncing Your Fork with Upstream**
Before working on new features, keep your fork updated:

```sh
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

This avoids merge conflicts when submitting PRs.