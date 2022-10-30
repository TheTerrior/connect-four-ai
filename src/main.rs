mod neuralnet;

use std::{fs, io::Error};
use clap::{arg, command, value_parser, ArgAction, Command, Arg};
use ncurses;


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


    testing();

}

fn testing() {

    /* Start ncurses. */
    ncurses::initscr();

    /* Print to the back buffer. */
    ncurses::addstr("Hello, world!");

    /* Print some unicode(Chinese) string. */
    // addstr("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Update the screen. */
    ncurses::refresh();

    /* Wait for a key press. */
    //let key: i32 = getch();

    let mut running = true;
    while running {
        let key_raw = ncurses::getch();
        let key: char = key_raw as u8 as char;

        if key == 'q' {
            running = false;
        } else {
            //addstr(&key.to_string());
            //refresh();
        }
    }

    /* Terminate ncurses. */
    ncurses::endwin();

}