use adw::prelude::*;
use gtk::{Align, Orientation, SelectionMode, gio};

use crate::{i18n::gettext, icons};

/// A placeholder application screen shown in the main area.
struct Screen {
    title: String,
    page: gtk::Widget,
}

pub fn build_window(app: &adw::Application) -> adw::ApplicationWindow {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(&gettext("Gnome Rusty Starter"))
        .default_width(1040)
        .default_height(720)
        .build();
    window.set_size_request(560, 480);

    let screens: Vec<Screen> = [
        placeholder_screen(&gettext("Screen One")),
        placeholder_screen(&gettext("Screen Two")),
        placeholder_screen(&gettext("Screen Three")),
    ]
    .into_iter()
    .collect();

    let content_toolbar = adw::ToolbarView::new();
    let content_header = adw::HeaderBar::new();
    let content_title = adw::WindowTitle::new(&gettext("Gnome Rusty Starter"), "");
    content_header.set_title_widget(Some(&content_title));
    content_toolbar.add_top_bar(&content_header);

    let stack = adw::ViewStack::new();
    stack.set_vexpand(true);
    stack.set_hhomogeneous(false);
    stack.set_vhomogeneous(false);
    for screen in &screens {
        stack.add_titled(&screen.page, None, &screen.title);
    }
    if let Some(first) = screens.first() {
        stack.set_visible_child(&first.page);
    }
    content_toolbar.set_content(Some(&stack));

    let (sidebar_toolbar, toggle) = build_sidebar(&screens, &stack);

    let split = adw::OverlaySplitView::new();
    split.set_sidebar_width_fraction(0.26);
    split.set_min_sidebar_width(220.0);
    split.set_max_sidebar_width(300.0);
    split.set_sidebar(Some(&sidebar_toolbar));
    split.set_content(Some(&content_toolbar));

    split
        .bind_property("show-sidebar", &toggle, "active")
        .bidirectional()
        .sync_create()
        .build();

    window.set_content(Some(&split));

    let narrow = adw::Breakpoint::new(
        adw::BreakpointCondition::parse("max-width: 760sp")
            .expect("valid starter sidebar breakpoint"),
    );
    let split_apply = split.clone();
    narrow.connect_apply(move |_| {
        split_apply.set_collapsed(true);
    });
    let split_unapply = split.clone();
    narrow.connect_unapply(move |_| {
        split_unapply.set_collapsed(false);
    });
    window.add_breakpoint(narrow);

    window
}

fn build_sidebar(
    screens: &[Screen],
    stack: &adw::ViewStack,
) -> (adw::ToolbarView, gtk::ToggleButton) {
    let toolbar = adw::ToolbarView::new();
    let header = adw::HeaderBar::new();
    header.set_show_title(false);

    let menu = gio::Menu::new();
    menu.append(Some(&gettext("About")), Some("app.about"));
    menu.append(Some(&gettext("Quit")), Some("app.quit"));
    let menu_button = gtk::MenuButton::builder()
        .icon_name(icons::MAIN_MENU)
        .tooltip_text(&gettext("Main Menu"))
        .primary(true)
        .build();
    menu_button.set_menu_model(Some(&menu));
    header.pack_end(&menu_button);

    let toggle = gtk::ToggleButton::builder()
        .icon_name(icons::LEFT_SIDEBAR)
        .tooltip_text(&gettext("Toggle Sidebar"))
        .active(true)
        .build();
    header.pack_start(&toggle);

    toolbar.add_top_bar(&header);

    let list = gtk::ListBox::new();
    list.set_selection_mode(SelectionMode::Single);
    list.set_vexpand(true);
    list.add_css_class("navigation-sidebar");

    for screen in screens {
        list.append(&navigation_row(&screen.title));
    }

    // Selecting a sidebar row switches the visible ViewStack page by widget,
    // so the navigation does not depend on auto-generated child names.
    let pages: Vec<gtk::Widget> = screens.iter().map(|s| s.page.clone()).collect();
    let stack_for_selection = stack.clone();
    list.connect_row_selected(move |_, row| {
        let Some(row) = row else {
            return;
        };
        let index = row.index() as usize;
        if let Some(page) = pages.get(index) {
            stack_for_selection.set_visible_child(page);
        }
    });

    if let Some(first_row) = list.first_child() {
        if let Some(first_row) = first_row.downcast_ref::<gtk::ListBoxRow>() {
            list.select_row(Some(first_row));
        }
    }

    let sidebar_box = gtk::Box::new(Orientation::Vertical, 0);
    sidebar_box.append(&list);
    toolbar.set_content(Some(&sidebar_box));

    (toolbar, toggle)
}

fn navigation_row(title: &str) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();
    let label = gtk::Label::builder().label(title).xalign(0.0).build();
    label.set_margin_top(10);
    label.set_margin_bottom(10);
    label.set_margin_start(12);
    label.set_margin_end(12);
    row.set_child(Some(&label));
    row
}

fn placeholder_screen(title: &str) -> Screen {
    let page = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Fill)
        .valign(Align::Center)
        .spacing(12)
        .build();
    let title_label = gtk::Label::builder().label(title).build();
    title_label.add_css_class("title");
    let hint = gtk::Label::builder()
        .label(&gettext("Replace this placeholder with your application content."))
        .build();
    hint.add_css_class("dim-label");
    page.append(&title_label);
    page.append(&hint);
    Screen {
        title: title.to_string(),
        page: page.upcast(),
    }
}
