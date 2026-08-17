# Gnome Rusty Starter Left Sidebar v0.1.0 Release Candidate QA

## Build identity

- [ ] Build the release manifest: `io.github.christiaanbruinsma.GnomeRustyStarterLeftSidebar.yml`.
- [ ] Confirm application ID: `io.github.christiaanbruinsma.GnomeRustyStarterLeftSidebar`.
- [ ] Confirm About/version reports `0.1.0`.
- [ ] Preserve the generated `Cargo.lock` in the release source before final publication.

## Static Rust gates

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --locked --all-targets --all-features -- -D warnings`
- [ ] `cargo test --locked`

If `Cargo.lock` does not yet exist, run one normal Cargo build/generate-lockfile first, review the lockfile, then rerun the locked gates.

## Runtime smoke

- [ ] Application launches.
- [ ] Left navigation sidebar is shown next to the main area.
- [ ] Sidebar contains three placeholder screen rows.
- [ ] Selecting a sidebar row switches the visible main-area screen.
- [ ] Sidebar toggle button shows/hides the sidebar.
- [ ] On a narrow window, the sidebar collapses (breakpoint) and the main area stays usable.
- [ ] Primary menu opens with About and Quit entries.
- [ ] About dialog shows application name, icon, developer, version, and license.
- [ ] Quit action (`<primary>q` and menu entry) exits the application.

## Localization smoke

- [ ] Installed Flatpak follows the system language with English as fallback.
- [ ] Smoke `LANGUAGE=nl`, `de`, `fr`, `es`, `it`, and `pt`.
- [ ] Confirm translated window title, sidebar rows, menu, tooltips, About comments, and placeholder hint.

## Installed Flatpak QA

- [ ] Export/install the stable-ID Flatpak.
- [ ] Launch the installed app successfully.
- [ ] Confirm semantic icons follow the active host icon theme.
- [ ] Confirm no missing-icon placeholders.
- [ ] Confirm the application icon appears in the launcher, Software/AppStream, and About dialog.
- [ ] Confirm the symbolic application icon resolves in the installed runtime.

## Release metadata

- [ ] AppStream metadata validates in the release environment.
- [ ] Desktop metadata validates in the release environment.
- [ ] README and CHANGELOG describe v0.1.0.
- [ ] No obsolete product name remains in release content.
- [ ] Final source archive contains the intended icons/assets and `Cargo.lock`.
