// This sets up the structure of the application and handles creating a canvas and input from the user.
// Everything in this is based on a minesweeper coding example of this program's parent, the TUI library cursive.
// Going forward, segmenting this more might make sense; maybe move the help section to another file, for example.

mod canvas;

use std::fs;
use std::env;
use regex::Regex;


use cursive::views::{Button, Dialog, LinearLayout, Panel, EditView, ListView, TextView, SliderView, ViewRef};
use cursive::theme::{BorderStyle, Color, Theme, Palette, PaletteColor::*};
use cursive::traits::*;
use cursive::event::Event;
use cursive::traits::Identifiable;
use cursive::Cursive;
use cursive::Vec2;
use cursive::backends;
use cursive_buffered_backend::BufferedBackend;

#[cfg(target_os = "windows")]
fn backend() -> Box<BufferedBackend> {
    let crossterm_backend = backends::crossterm::Backend::init().unwrap();
    let buffered_backend = cursive_buffered_backend::BufferedBackend::new(crossterm_backend);
    Box::new(buffered_backend)
}

#[cfg(not(target_os = "windows"))]
fn backend() -> Box<BufferedBackend> {
    let termion_backend = backends::termion::Backend::init().unwrap();
    let buffered_backend = cursive_buffered_backend::BufferedBackend::new(termion_backend);
    Box::new(buffered_backend)
}

fn main() {
    let mut siv = Cursive::new(|| {
        backend()
    });



    theme_light(&mut siv);

    siv.add_layer(
        Dialog::new()
            .title("kakikun - 描きくん")
            .padding_lrtb(2, 2, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("  New Canvas   ", show_options))
                    .child(Button::new_raw("  Credits   ", show_credits))
                    .child(Button::new_raw("    Exit     ", |s| s.quit())),
            ),
    );

    siv.run();
}

fn show_options(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Creating a new Canvas...")
            .button("Ok", |siv| {
                let mut size = Vec2::new(50, 25);
                siv.call_on_name("edit_width", |edit: &mut EditView| {
                    let width = edit.get_content();
                    let trim_w = width.trim();
                    match trim_w.parse::<usize>() {
                        Ok(i) => {size.x = i},
                        Err(..) => {} // Maybe do something here?
                    };
                });

                siv.call_on_name("edit_height", |edit: &mut EditView| {
                    let height = edit.get_content();
                    let trim_h = height.trim();
                    match trim_h.parse::<usize>() {
                        Ok(i) => {size.y = i},
                        Err(..) => {}
                    };
                });


                new_canvas(siv, size);
            })

            .dismiss_button("Back")
            .content(
                LinearLayout::vertical().child(
                ListView::new()
                    .child(
                        "Width:",
                                EditView::new()
                                    .filler(" <-           ")
                                    .with_name("edit_width")
                                    .fixed_width(20),
                    )
                    .child(
                        "Height:",
                                EditView::new()
                                    .filler(" <-         ")
                                    .with_name("edit_height")
                                    .fixed_width(20),
                    )
                )
                .child(TextView::new("\n\nPro Tip: Since cells in your terminal are rectangular,\n\
                                       a square canvas has double the width here.\n\
                                       Try not to make these larger than 80x40.")),

        )
    )
}

fn show_credits(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
        .content(
            LinearLayout::vertical()
            .child(TextView::new("Thank you for using 描きくん!"))
            .child(TextView::new("\n\n\n"))
            .child(TextView::new("描きくん is open source, written in the Rust programming langauge and prominently uses the Text User Interface library Cursive."))
            .child(TextView::new("If you encounter any problems, you can share them at github.com/file-acomplaint/kakikun"))
            .child(TextView::new("or send me, fi-le, some mail. The address is: info at file.net"))
            .child(TextView::new("(Drawings and ASCII art created in kakikun is also appreciated.)"))
            .child(TextView::new("\n"))
            .child(TextView::new(" v 0.1.0 - MIT License"))
        )
        .button("Back", |s| {
            s.pop_layer();
        })
    )
}

fn interpret_command (s: &mut Cursive, name: &str) {
    // TODO: Check Regex for possible illegal inputs like - save foo.bar.png; maybe implement filepaths?

    let re_brush = Regex::new("brush .").unwrap();
    let re_save_unicode = Regex::new("save .+[.]txt").unwrap();
    let re_save_image = Regex::new("save .+[.](jpg|png|jpeg)").unwrap();
    let re_save = Regex::new("save .+([.]kkun|)").unwrap();
    let re_load = Regex::new("load .+([.]kkun|)").unwrap();
    let re_load_image = Regex::new("load .+[.](jpg|png|jpeg)").unwrap();

    if re_brush.is_match(name) {
        s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.changebrushsymbol(name.chars().nth(6).unwrap())});
    }

    else if re_save_unicode.is_match(name) {
        let filename = get_filename(name.to_string());
        let mut success = false;

        s.call_on_name("canvas", |view: &mut canvas::CanvasView| {
            let text = view.totext();

            let mut path = env::current_dir().unwrap();
            path.push(&filename);
            let result = fs::write(path, text);
            match result {
                Ok(_i) => success = true,
                Err(_e) => {},
            }
        });

        if success {give_feedback(s, format!("Saved {} as unicode.", filename));}
        else {give_feedback(s, "Something went wrong.\n  Does the filename contain weird characters?".to_string());}
    }

    else if re_save_image.is_match(name) {
        let filename = get_filename(name.to_string());
        let mut success = false;

        s.call_on_name("canvas", |view: &mut canvas::CanvasView| {
            let img = view.toimage();

            let mut path = env::current_dir().unwrap();
            path.push(&filename);
            let result = img.save(path);
            match result {
                Ok(_i) => success = true,
                Err(_e) => {},
            }
        });

        if success {give_feedback(s, format!("Saved {} as image.", filename));}
        else {give_feedback(s, "Something went wrong.\n  Does the filename contain weird characters?".to_string());}

    }

    else if re_save.is_match(name) {
        let filename = get_filename(name.to_string());
        let mut success = false;

        s.call_on_name("canvas", |view: &mut canvas::CanvasView| {
            let text = view.tofile();

            let mut path = env::current_dir().unwrap();
            path.push(&filename);
            let result = fs::write(path, text);
            match result {
                Ok(_i) => success = true,
                Err(_e) => {},
            }
        });

        if success {give_feedback(s, format!("Saved {}.", filename));}
        else {give_feedback(s, "Something went wrong.\n  Does the filename contain weird characters?".to_string());}
    }

    else if re_load_image.is_match(name) {
        let filename = get_filename(name.to_string());

        let mut path = env::current_dir().unwrap();
        path.push(&filename);
        let img = image::open(path);
        match img {
            Ok(i) => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.fromimage(i, None);});
                      give_feedback(s, format!("Loaded {}.", filename))}
            Err(e) => {give_feedback(s, format!("{}.", e));}
        }
    }

    else if re_load.is_match(name) {

        let filename = get_filename(name.to_string());
        let mut i = 1;
        for n in name.split_whitespace() {
            if i == 2 {
                let mut path = env::current_dir().unwrap();
                path.push(&filename);

                let text = fs::read_to_string(path);
                match text {
                    Ok(i) => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.fromfile(i);});
                              give_feedback(s, format!("Loaded {}.", n))}
                    Err(e) => {give_feedback(s, format!("{}.", e));}
                }
            }

            i += 1;
        }
    }


    else if name == "help" {
        s.add_layer(
            Dialog::new()
                .title("Help")
                .content(
                    LinearLayout::vertical()

                    .child(
                        ListView::new()
                            .child("\t", TextView::new("Welcome to Painting in the Console with kakikun!"))
                            .child("\t", TextView::new(""))
                            .child("\t", TextView::new("In this program, you can paint with colour and a lot of unicode characters on the canvas below."))
                            .child("\t", TextView::new("To choose a colour, click on the palette on the left. If you want a different hue, try moving the slider next to it."))
                            .child("\t", TextView::new("A left click on the palette selects a background tone, a right click selects the character's colour."))
                            .child("\t", TextView::new("To choose a character other than a blank space to paint with, you can use this command:"))
                            .child("\t", TextView::new(""))
                            .child("brush ", TextView::new("Change the symbol with which you draw to any unicode character. Example: brush é"))
                            .child("\t", TextView::new(""))
                            .child("\t", TextView::new("Since there a quite a few more of those commands, feel free to refer to the sections below. Have fun!")),
                        )
                    )
                .button("   General   ", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Help")
                            .content(
                                LinearLayout::vertical()
                                .child(
                                    ListView::new()
                                        .child("\t", TextView::new("Some general commands to try:"))
                                        .child("\t", TextView::new(""))
                                        .child("back    | Ctrl+Z", TextView::new("Undoes one step of painting. Kakikun will at most remember 250 actions."))
                                        .child("pipette | Ctrl+P", TextView::new("Lets you pick colours and symbols from the canvas. Reverts to brush automatically."))
                                        .child("clear", TextView::new("Clears the canvas to white background."))
                                        .child("fill all", TextView::new("Fills the whole of the canvas with the current brush setting."))
                                        .child("save", TextView::new("Saves the background colours in image format, the characters as text or everything as a kakikun project."))
                                        .child("\t", TextView::new("Examples: save image.png, save img.jpg, save ascii_art.txt, save everything.kkun"))
                                        .child("load", TextView::new("Loads an image or a kakikun project. Scales images down to a console-friendly size."))
                                        .child("theme", TextView::new("Loads a theme. light, dark and default are Available. Example: theme dark"))
                                        .child("quit", TextView::new("Closes the program."))
                                    )
                                )
                            .button("Back", |s| {
                                s.pop_layer();
                            }))
                })
                .button("   Image Operations   ", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Help")
                            .content(
                                LinearLayout::vertical()
                                .child(
                                    ListView::new()
                                        .child("\t", TextView::new("These little tricks will only affect the background colours:"))
                                        .child("\t", TextView::new(""))
                                        .child("flip | Ctrl+F", TextView::new("Flips the canvas. Examples: flip, flip -v"))
                                        .child("blur", TextView::new("Blurs the canvas."))
                                        .child("grayscale", TextView::new("Converts to greyscale."))
                                        .child("brighten", TextView::new("Brightens up everything. :)"))
                                        .child("darken", TextView::new("Darkens down everything. :("))
                                        .child("rotate hue", TextView::new("Shifts the hue around the colour wheel."))
                                        .child("contrast", TextView::new("Increases the contrast."))
                                        .child("decontrast", TextView::new("Decreases the contrast."))
                                        .child("", TextView::new(""))
                                        .child("rotate", TextView::new("Rotates by 90° clockwise. Squares off everything to keep the proportions, so use carefully."))
                                    )
                                )
                            .button("Back", |s| {
                                s.pop_layer();
                            }))
                })
                .button("Back", |s| {
                    s.pop_layer();
                }),
        );
    }

    match name {
        "clear" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.clear();});},
        "fill all" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.fill_all()});},
        //"flip" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.flip()});},
        "flip -h" | "flip" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.fliph()});},
        "flip -v" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.flipv()});},
        "rotate" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.rotate90()});},
        "blur" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.blur(0.4)});}, // TODO: Implement blurring with different sigma, like "blur 0.5"
        "grayscale" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.grayscale()});},
        "brighten" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.brighten(1)});}, // TODO: Implement brightening with different values
        "darken" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.brighten(-1)});},
        "rotate hue" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.huerotate(1)});},
        "contrast" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.adjust_contrast(0.2)});},
        "decontrast" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.adjust_contrast(-0.2)});},
        "pipette" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.set_tool(canvas::Tool::Pipette)});},
        "back" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.back()});},
        "sargent" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {let sargent = include_bytes!("assets/sargent.kkun");
                                                                                view.fromfile(String::from_utf8_lossy(sargent).to_string());});}
        "fi-le" => {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {let file = include_bytes!("assets/file.kkun");
                                                                                view.fromfile(String::from_utf8_lossy(file).to_string());});}
        "width" => {let mut canvas: ViewRef<canvas::CanvasView> = s.find_name("canvas").unwrap();
                    let width: u32 = canvas.get_width();

                    give_feedback(s, format!("The canvas is {} cells wide.", width))},

        "height" => {let mut canvas: ViewRef<canvas::CanvasView> = s.find_name("canvas").unwrap();
                    let height: u32 = canvas.get_height();

                    give_feedback(s, format!("The canvas is {} cells high.", height))},

        "cells" => {let mut canvas: ViewRef<canvas::CanvasView> = s.find_name("canvas").unwrap(); // Only in here for debugging
                    let length: u32 = canvas.get_overlay_len();

                    give_feedback(s, format!("The canvas is {} cells high.", length))},

        "theme light" => {theme_light(s)},
        "theme dark" => {theme_dark(s)},
        "theme default" => {theme_default(s)},
        "quit" | "exit" => {s.quit()},


        _ => {},
    }

    match name {
        "clear" | "fill all" | "grayscale" | "sargent" | "height" | "width" | "theme dark" | "theme light" | "fi-le" | "theme default" | "pipette"  => {clear_pseudoconsole(s);},
        _ => {}
    }
}

fn give_feedback(siv: &mut Cursive, s: String) {
    let mut feedback: ViewRef<TextView> = siv.find_name("feedback").unwrap();
    feedback.set_content(String::from("  ") + &s);
}

fn clear_pseudoconsole(siv: &mut Cursive) {
    let mut terminal: ViewRef<EditView> = siv.find_name("pseudoterminal").unwrap();
    terminal.set_content("");
}

fn get_filename(input: String) -> String {
    let words: Vec<&str> = input.split(' ').collect();

    words.last().clone().unwrap().to_string()

}

fn theme_light(siv: &mut Cursive) {
    let mut palette = Palette::default();
    palette[Background] = Color::Rgb(191,171,150);
    palette[Primary] = Color::Rgb(28, 15, 3);
    palette[Secondary] = Color::Rgb(185, 61, 42);
    palette[TitlePrimary] = Color::Rgb(185, 61, 42);
    palette[Highlight] = Color::Rgb(185, 61, 42);
    palette[HighlightText] = Color::Rgb(241, 215, 190);
    palette[View] = Color::Rgb(241, 215, 190);
    palette[Shadow] = Color::Rgb(173,135,100);
    siv.set_theme(Theme {shadow: true, borders: BorderStyle::Simple, palette: palette});
}

fn theme_dark(siv: &mut Cursive) {
    let mut palette = Palette::default();
    palette[Background] = Color::Rgb(35,33,30);
    palette[Primary] = Color::Rgb(199,188,170);
    palette[Secondary] = Color::Rgb(230, 72, 57);
    palette[TitlePrimary] = Color::Rgb(230, 72, 57);
    palette[Highlight] = Color::Rgb(185, 61, 42);
    palette[HighlightText] = Color::Rgb(241, 215, 190);
    palette[View] = Color::Rgb(60,54,51);
    palette[Shadow] = Color::Rgb(24,21,19);
    siv.set_theme(Theme {shadow: true, borders: BorderStyle::Simple, palette: palette});
}

fn theme_default(siv: &mut Cursive) {
    let palette = Palette::default();
    siv.set_theme(Theme {shadow: true, borders: BorderStyle::Simple, palette: palette});
}

fn new_canvas(siv: &mut Cursive, size: Vec2) {
    // This is where we set up the layout of the main painting. A current issue is that altough canvas size may change over this layers' lifetime, the palette et cetera stay the same.
    let _board = canvas::Board::new(size);
    let picker_height = ((size.y as i32 + 25) / 2) - ((size.y as i32 - 25).abs() / 2); // This is just a fancy way to get the minimum of 25 and size.y

    // Let's add some fun keybindings
    siv.add_global_callback(Event::CtrlChar('z'), |s| {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.back()});});
    siv.add_global_callback(Event::CtrlChar('p'), |s| {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.set_tool(canvas::Tool::Pipette)});});
    siv.add_global_callback(Event::CtrlChar('f'), |s| {s.call_on_name("canvas", |view: &mut canvas::CanvasView| {view.fliph()});});
    siv.add_layer(
        Dialog::new()
            .title("kakikun - 描きくん")
            .content(
                LinearLayout::vertical()
                .child(
                    Panel::new(
                    LinearLayout::horizontal()
                        .child(
                            SliderView::vertical(picker_height as usize)
                            .on_change(|s, n| {s.call_on_name("picker", |view: &mut canvas::PickView| {
                                view.set_hue((360. * (n as f32) / (view.height) as f32) as u16);
                            });
                        }))

                        .child(canvas::PickView::new(10, picker_height as u8).with_name("picker"))
                        .child(canvas::CanvasView::new(size).with_name("canvas")),
                ))
                .child(
                    LinearLayout::horizontal()
                    .child(TextView::new("> "))
                    .child(
                    EditView::new()
                        .on_submit(interpret_command)
                        .with_name("pseudoterminal")
                        .min_width(size.x + 10),

                    )
                )
                .child(TextView::new("  Try typing 'help' above").with_name("feedback"))
            )
            .button("Quit Painting", |s| {
                s.pop_layer();
            }),
    );
}
