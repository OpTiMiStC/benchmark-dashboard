# Benchmark Dashboard

A small Yew-based web app for viewing and sorting mock cloud server CPU benchmark results.

## Project purpose

This project demonstrates a lightweight benchmark dashboard for comparing cloud server CPU instances across AWS and Google Cloud. It is designed as a simple frontend app that can be built to WebAssembly, tested in CI, and deployed automatically to GitHub Pages.

The dashboard supports:
- searching benchmark rows
- sorting by instance, benchmark, score, cost per run, and test date
- presenting a clean browser-based view of benchmark data

## Tech stack

- Rust
- Yew
- WebAssembly (`wasm32-unknown-unknown`)
- Trunk for bundling and serving the app
- HTML/CSS for page structure and styling
- GitHub Actions for CI/CD
- GitHub Pages for hosting

## Branch strategy

The current workflow configuration implies this branch model:

- `feature/**`: feature branches for day-to-day development work
- `dev`: shared integration branch
- `main`: production branch

CI runs on pushes to `feature/**`, `dev`, and `main`, plus pull requests targeting `main`.

Deployment is restricted to pushes to `main`.

## CI/CD flow

### CI

The CI workflow is defined in [.github/workflows/ci.yml](.github/workflows/ci.yml).

On supported branches, it:
1. checks out the repository
2. installs the Rust toolchain and `wasm32-unknown-unknown` target
3. installs Trunk
4. restores Cargo cache
5. checks formatting with `cargo fmt --all -- --check`
6. runs linting with `cargo clippy --all-targets --all-features -- -D warnings`
7. runs tests with `cargo test`
8. builds the WebAssembly app with `trunk build --release --public-url /benchmark-dashboard/`

### CD

The deployment workflow is defined in [.github/workflows/deploy.yml](.github/workflows/deploy.yml).

On every push to `main` (or manual dispatch), it:
1. checks out the repository
2. configures GitHub Pages
3. installs the WebAssembly target
4. installs Trunk
5. builds the site into `dist/`
6. uploads the built artifact
7. deploys the artifact to GitHub Pages

## How deployment works

Deployment is handled by GitHub Actions and GitHub Pages:

- Trunk builds the Yew app into static assets under `dist/`
- the deploy workflow uploads `dist/` as the Pages artifact
- GitHub Pages serves the built site
- the app is built with `--public-url /benchmark-dashboard/`, so asset paths are correct for a repository-hosted Pages site

## Synthetic data note

All benchmark rows shown in the dashboard are synthetic mock data.

They are included for demonstration, UI testing, sorting behavior, and CI/CD validation only. They should not be treated as real AWS or Google Cloud benchmark measurements, pricing data, or performance claims.

## Local development

Run the app locally:

```bash
trunk serve --open
```

Run tests locally:

```bash
cargo test
```
