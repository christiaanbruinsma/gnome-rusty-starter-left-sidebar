# Changelog

## 0.1.0 — 2026-08-17

- Establish Gnome Rusty Starter Left Sidebar as the left-sidebar layout template of the Rust + GTK4 + libadwaita starter suite.
- Provide a two-pane layout: a left navigation sidebar and a main area, built on an `adw::OverlaySplitView` with a manual `adw::Breakpoint` that collapses the sidebar on narrow widths (the Data Inspector pattern, without the right Inspector).
- Implement sidebar-driven screen switching: a `gtk::ListBox` styled with the `navigation-sidebar` CSS class selects an `adw::ViewStack` page in the main area.
- Ship three placeholder screens as the reference navigation set.
- Add reusable platform plumbing: `config.rs` identity, `i18n.rs` gettext boundary, `icons.rs` centralized semantic icon module with a debug runtime audit.
- Add `application.rs` with `quit` and `about` app actions and accelerators.
- Add Meson build with a `default`/`development` profile that drives a consistent application ID across desktop, MetaInfo, icon installation, About, and runtime registration.
- Add a `clippy` Meson run target using `-D warnings` across all targets and features.
- Add stable and development Flatpak manifests with minimal, justified runtime permissions.
- Add native GNU gettext localization for English (fallback), Dutch, German, French, Spanish, Italian, and Portuguese, with localized desktop and AppStream metadata.
- Add the suite-standard application icon pair (main and symbolic) and bundled `hicolor` fallbacks for the semantic UI icons that cannot be relied on in every runtime/theme.
- Add engineering documentation: Golden Standard, Native Icon Audit, and a Release Checklist.
