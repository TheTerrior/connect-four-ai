mod neuralnet;

use std::{fs, io::Error};
use clap::{arg, command, value_parser, ArgAction, Command, Arg};
use ncurses as nc;
use cursive::views::{Dialog, TextView};

/*
Game options:

Player vs AI Local
Player vs AI Online
Player vs Player Local
Player vs Player Online
*/


fn main() {
    /*
    let args = command!()
        .arg(arg!(
            --host "Host an online match that another player can join"
        ))
        .arg(arg!(
            --join <lobby_code> "Join an online match hosted by another player"
        ))
        //.arg(
        //    Arg::new("2player").long("2player").action(ArgAction::SetTrue).help("Play a two-player game of Connect Four")
        //)
        .get_matches(); // run clap, can be omitted to save layout to a variable
    */

    //let target: String = String::from("tempdatabase.gn");
    //let result = fs::read(target).unwrap();
    //println!("hi");


    //testing_ncurses();
    testing_cursive();

}

fn testing_cursive() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
                         .title("Cursive")
                         .button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}

fn testing_ncurses() {
    const REGULAR_PAIR: i16 = 0;
    const HIGHLIGHT_PAIR: i16 = 1;

    /* Start ncurses. */
    nc::initscr();

    nc::noecho();       //disable key typing
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);    //disable cursor

    nc::start_color();
    nc::init_pair(REGULAR_PAIR, nc::COLOR_WHITE, nc::COLOR_BLACK);
    nc::init_pair(HIGHLIGHT_PAIR, nc::COLOR_BLACK, nc::COLOR_WHITE);

    let items = vec![
        "Write the todo app",
        "Buy a bread",
        "Make a cup of tea",
    ];

    let mut item_curr: usize = 0; //the item that is selected by default on startup

    /* Wait for a key press. */
    //let key: i32 = getch();

    let mut running = true;
    while running {
        for (index, item) in items.iter().enumerate() {
            let pair = {
                if item_curr == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            nc::attron(nc::COLOR_PAIR(pair));   //start coloring
            nc::mv(index as i32, 1);            //move to the position (y,x), so in this case one off of the start of the 'index' row
            nc::addstr(*item);                  //print
            nc::attroff(nc::COLOR_PAIR(pair));  //end coloring
        }

        nc::refresh();

        let key = nc::getch();

        // match keys that do not have a character equivalent
        match key {
            65 => if item_curr > 0 {item_curr -= 1;},                   //up
            66 => if item_curr < items.len() - 1 {item_curr += 1;},     //down
            _ => {

                //match keys that have a character equivalent
                match key as u8 as char {
                    'q' => running = false,
                    '\n' => running = false,
                    'k' => if item_curr > 0 {item_curr -= 1;},
                    'j' => if item_curr < items.len() - 1 {item_curr += 1;}
                    _ => {},
                }
            }
        }

    }

    /* Terminate ncurses. */
    nc::endwin();

}