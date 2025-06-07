use adw::{glib, Application, ApplicationWindow};
use adw::{prelude::*, HeaderBar};
use clap::{Arg, Args, Command};
use gtk::{
    gdk, Align, Box, Entry, Label, ListBox, ListBoxRow, Orientation, PolicyType, ScrolledWindow,
};
use std::cell::Cell;
use std::cmp::Ordering;
use std::io;
use std::rc::Rc;

const APP_ID: &str = "net.koteya.pipemenu";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct AppOptions {
    window_title: String,
    prompt: Option<String>,
}

fn main() -> glib::ExitCode {
    let cli = Command::new("pipemenu")
        .author("https://github.com/soanvig/pipemenu")
        .version(VERSION)
        .about("Gnome (GTK4 + libadwaita) dmenu alternative")
        .arg(
            Arg::new("title")
                .long("title")
                .short('t')
                .value_name("TITLE")
                .help("set window title")
                .default_value("pipemenu"),
        )
        .arg(
            Arg::new("prompt")
                .long("prompt")
                .short('p')
                .value_name("PROMPT")
                .help("set prompt text"),
        )
        .override_usage("<stdin> | pipemenu\tEXAMPLE: ls | pipemenu")
        .get_matches();

    let app_options = AppOptions {
        window_title: cli.get_one::<String>("title").unwrap().to_string(),
        prompt: cli.get_one::<String>("prompt").cloned(),
    };

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| build_ui(app, &app_options));
    app.run_with_args::<glib::GString>(&[])
}

fn build_ui(app: &Application, options: &AppOptions) {
    let selected_entry = Rc::new(Cell::new(0));
    let entries: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let search = Entry::new();
    search.set_hexpand(true);
    let entry_list = ListBox::new();

    let search_row = Box::new(Orientation::Horizontal, 12);

    let search_prompt = options
        .prompt
        .as_ref()
        .map(|prompt| Label::new(Some(prompt)));

    if let Some(search_prompt) = search_prompt {
        search_row.append(&search_prompt);
    }

    search_row.append(&search);

    let entry_list_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&entry_list)
        .vexpand(true)
        .build();

    let content_box = Box::new(Orientation::Vertical, 24);
    content_box.set_property("margin-start", 24);
    content_box.set_property("margin-end", 24);
    content_box.set_property("margin-bottom", 24);
    content_box.append(&search_row);
    content_box.append(&entry_list_scroll);

    let root = Box::new(Orientation::Vertical, 24);
    root.append(&HeaderBar::new());
    root.append(&content_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&options.window_title)
        .content(&root)
        .default_width(1000)
        .default_height(600)
        .build();

    connect_keyboard_controller(
        &window,
        entry_list.clone(),
        selected_entry.clone(),
        search.clone(),
    );

    rebuild_entry_list(&entry_list, &entries);

    connect_search_change(
        entries,
        search.clone(),
        entry_list.clone(),
        entry_list_scroll.clone(),
        selected_entry.clone(),
    );

    connect_entry_click(entry_list.clone());

    connect_search_activate(search.clone(), entry_list.clone());

    window.present();
}

fn connect_search_change(
    entries: Vec<String>,
    search: Entry,
    entry_list: ListBox,
    entry_list_scroll: ScrolledWindow,
    selected_entry: Rc<Cell<i32>>,
) {
    search.connect_changed(move |search| {
        let text = search.buffer().text().to_string().to_lowercase();

        let mut filtered: Vec<(&String, usize, Ordering)> = entries
            .iter()
            .map(|item| (item, item.to_lowercase()))
            .map(|(item, lowercased)| {
                (
                    item,
                    textdistance::str::lcsseq(&lowercased, &text),
                    if lowercased.contains(&text) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    },
                )
            })
            .collect();

        filtered.sort_by(
            |(_, d1, exact1), (_, d2, exact2)| match exact2.cmp(exact1) {
                Ordering::Equal => d2.cmp(d1),
                v => v,
            },
        );

        rebuild_entry_list(
            &entry_list,
            &filtered.iter().map(|(entry, _, _)| entry).collect(),
        );

        selected_entry.set(0);
        entry_list_scroll.vadjustment().set_value(0.);
    });
}

fn rebuild_entry_list<S: AsRef<str>>(entry_list: &ListBox, entries: &Vec<S>) {
    entry_list.remove_all();

    for entry in entries {
        let label = Label::new(Some(entry.as_ref()));

        label.set_property("margin-start", 12);
        label.set_property("margin-top", 12);
        label.set_property("margin-end", 12);
        label.set_property("margin-bottom", 12);
        label.set_halign(Align::Start);

        entry_list.append(&label);
    }

    entry_list.select_row(entry_list.row_at_index(0).as_ref());
}

fn connect_search_activate(search: Entry, entry_list: ListBox) {
    search.connect_activate(move |_| {
        handle_row_selection(entry_list.selected_row().as_ref());
    });
}

fn connect_entry_click(entry_list: ListBox) {
    entry_list.connect_row_activated(|_, row| {
        handle_row_selection(Some(row));
    });
}

fn handle_row_selection(row: Option<&ListBoxRow>) {
    let selected_entry = row
        .and_then(|row| row.child())
        .and_then(|child| child.dynamic_cast::<Label>().ok())
        .and_then(|label| Some(label.text().as_str().to_string()));

    if let Some(selected_entry) = selected_entry {
        println!("{}", selected_entry);
        std::process::exit(0);
    }
}

fn connect_keyboard_controller(
    window: &ApplicationWindow,
    entry_list: ListBox,
    selected_entry: Rc<Cell<i32>>,
    search: Entry,
) {
    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Escape => {
                std::process::exit(1);
            }
            gdk::Key::Down => {
                let next_index = selected_entry.get() + 1;
                let row_to_select = entry_list.row_at_index(next_index);

                if let Some(row_to_select) = row_to_select {
                    row_to_select.grab_focus();
                    search.grab_focus_without_selecting();
                    entry_list.select_row(Some(&row_to_select));
                    selected_entry.set(next_index);
                }
            }
            gdk::Key::Up => {
                let next_index = (selected_entry.get() - 1).max(0);
                let row_to_select = entry_list.row_at_index(next_index);

                if let Some(row_to_select) = row_to_select {
                    row_to_select.grab_focus();
                    search.grab_focus_without_selecting();
                    entry_list.select_row(Some(&row_to_select));
                    selected_entry.set(next_index);
                }
            }
            _ => (),
        }

        // This ensures that if search box is selected, pressing TAB or arrow want lose focus from the widget
        glib::Propagation::Stop
    });

    window.add_controller(event_controller);
}
