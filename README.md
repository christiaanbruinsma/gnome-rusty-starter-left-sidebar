# Gnome Rusty Starter — Left Sidebar

Rust + GTK4 + libadwaita starter for GNOME applications with a left navigation
sidebar. The sidebar acts as a menu for switching between application screens;
the main area renders the active screen. This template provides the reusable
platform plumbing (identity, localization, semantic icons, build, packaging,
QA) needed to fork into a real application without re-establishing the
engineering baseline each time.

It is the left-sidebar layout variant of the Rusty Starter suite: one left
sidebar plus a main area (two panes). See
[docs/GOLDEN-STANDARD.md](docs/GOLDEN-STANDARD.md) for the canonical engineering
reference shared across the suite.

## Layout pattern

- An `adw::OverlaySplitView` places the left navigation sidebar next to the
  main area, with a manual `adw::Breakpoint` that collapses the sidebar on
  narrow widths (the Data Inspector pattern, without the right Inspector).
- The sidebar contains a `gtk::ListBox` styled with the `navigation-sidebar`
  CSS class; each row selects one screen.
- The main area contains an `adw::ViewStack`; selecting a sidebar row switches
  the visible stack page.
- The application ships with three placeholder screens. Replace them with real
  application screens and add the corresponding rows to the sidebar list.

## Technical baseline

- Rust 2024 edition
- GTK4
- libadwaita
- Meson as the GNOME build layer
- Cargo for Rust compilation and tests
- GNU gettext for localization
- Flatpak with the GNOME Platform/SDK
- the `org.freedesktop.Sdk.Extension.rust-stable` SDK extension

Application-specific crates and feature modules are not part of the template.

## Layout files

- `src/main.rs` — process entrypoint.
- `src/lib.rs` — module declarations.
- `src/config.rs` — application identity (app ID, version, gettext package, locale directory).
- `src/i18n.rs` — gettext initialization and translation helpers behind one boundary.
- `src/icons.rs` — centralized semantic icon names and a debug runtime audit.
- `src/application.rs` — `adw::Application` lifecycle and app actions (quit, about).
- `src/window.rs` — left-sidebar window with `OverlaySplitView`, `ViewStack`, and three placeholder screens.

## Development build

Open `io.github.christiaanbruinsma.GnomeRustyStarterLeftSidebar.Devel.yml` in
GNOME Builder and use the normal **Build** action.

Development app ID:

`io.github.christiaanbruinsma.GnomeRustyStarterLeftSidebar.Devel`

## Release candidate

For the release identity use:

`io.github.christiaanbruinsma.GnomeRustyStarterLeftSidebar.yml`

Release app ID:

`io.github.christiaanbruinsma.GnomeRustyStarterLeftSidebar`

The release manifest builds with `-Dprofile=default`, which selects the
non-development app ID and Cargo release profile. The manifest currently
permits Cargo network access for dependency resolution; vendor dependencies
before publishing through a build service that requires fully offline sources.

## Engineering documentation

- [Golden Standard](docs/GOLDEN-STANDARD.md) — canonical engineering, native integration, packaging, and QA reference for the Rust GNOME app suite.
- [Native Icon Audit](docs/ICON-AUDIT.md) — audited semantic icon choices and runtime fallback policy.
- [Release Checklist](docs/RELEASE-CHECKLIST.md) — release-candidate QA gates.
