//! run this example with
//!   cargo run --example scrollable
//!
use crossterm::{
    cursor::Hide,
    cursor::Show,
    input::{input, InputEvent::*, KeyEvent::*},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::Color::*,
};
use std::io::{stderr, Write};
use termimad::*;
use minimad::*;

fn run_app(skin: MadSkin) -> Result<()> {
    // Create ClI with help message
    use clap::{App, AppSettings, Arg};
    let mut app = App::new("test_app")
        // Set the max term width the 3 short of  the actual width so that we don't wrap on the
        // help pager. Width is 3 shorter because of 1 char for the scrollbar and 1 char padding on
        // each side.
        .max_term_width(term_size::dimensions().map(|size| size.0 - 3).unwrap_or(0))
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("test_arg")
            .short("t")
            .long("test-arg")
            .long_help(concat!(
                "this is a lot of long text designed to stress the wrapping capapbility of the renderer. ",
                "this is a lot of long text designed to stress the wrapping capapbility of the renderer. ",
                "this is a lot of long text designed to stress the wrapping capapbility of the renderer. ",
                "this is a lot of long text designed to stress the wrapping capapbility of the renderer. ",
                "this is a lot of long text designed to stress the wrapping capapbility of the renderer. ",
            )));

    // Create doc template
    let doc_template = TextTemplate::from(
        r#"
    # Hello World

    ${help_message}
    "#,
    );
    // Expand help_message in doc
    let mut doc_expander = doc_template.expander();
    let mut help_message = vec![];
    app.write_long_help(&mut help_message).unwrap();
    let help_message = &String::from_utf8(help_message).unwrap();
    doc_expander.set_lines("help_message", help_message);
    let doc = doc_expander.expand();

    let mut w = stderr(); // we could also have used stdout
    queue!(w, EnterAlternateScreen)?;
    let _raw = RawScreen::into_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor

    let skin = make_skin();
    let mut area = Area::full_screen();
    area.pad(1, 1);
    let mut fmt_text = FmtText::from_text(&skin, doc.clone(), Some((area.width - 1) as usize));
    let mut view = TextView::from(&area, &fmt_text);
    let mut scroll = 0;

    let mut events = input().read_sync();
    let doc = doc.clone();
    loop {
        area = Area::full_screen();
        area.pad(1, 1);
        fmt_text = FmtText::from_text(&skin, doc.clone(), Some((area.width - 1) as usize));
        view = TextView::from(&area, &fmt_text);
        view.scroll = scroll;
        view.write_on(&mut w)?;
        w.flush()?;

        view.write_on(&mut w)?;
        if let Some(Keyboard(key)) = events.next() {
            match key {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            };
        }

        scroll = view.scroll;
    }
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

fn main() -> Result<()> {
    let skin = make_skin();
    run_app(skin)
}