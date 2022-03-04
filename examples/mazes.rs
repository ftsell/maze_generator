//! Maze generator test program
//!
use maze_generator::ellers_algorithm::EllersGenerator;
use maze_generator::growing_tree::GrowingTreeGenerator;
use maze_generator::prelude::*;
use maze_generator::prims_algorithm::PrimsGenerator;
use maze_generator::recursive_backtracking::RbGenerator;

//use std::collections::HashMap;
use clap::{Arg, Command};
use std::time::Instant;

//use std::io::{self};

fn main() {
    // Define the CLI arguments
    let matches = Command::new("Maze Test Program")
        .version("0.1.0")
        .about("Test harness for maze_generator library crate")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .default_value("df")
                .help("A maze generator name (ellers|prims|growingtree). Uses recursive backtracing if not specified."),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .takes_value(true)
                .default_value("20")
                .validator(|s| s.parse::<usize>())
                .help("Maze width in cells"),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .takes_value(true)
                .default_value("20")
                .validator(|s| s.parse::<usize>())
                .help("Maze height in cells"),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .long("method")
                .takes_value(true)
                .default_value("0")
                .validator(|s| s.parse::<usize>())
                .help("Selection method"),
        )
        .arg(
            Arg::new("static")
                .short('s')
                .long("static")
                .takes_value(false)
                .help("Use a static rng seed"),
        )
        .arg(
            Arg::new("textoutput")
                .short('d')
                .long("textoutput")
                .takes_value(false)
                .help("Output the text/debug version to stdout"),
        )
        .arg(
            Arg::new("svgoutput")
                .short('v')
                .long("svgoutput")
                .takes_value(false)
                .help("Output the svg version to stdout"),
        )
        .get_matches();

    // Process the args
    let num_str = matches.value_of("width");
    let width = match num_str {
        None => 10,
        Some(s) => match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 11,
        },
    };

    let num_str = matches.value_of("height");
    let height = match num_str {
        None => 10,
        Some(s) => match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 11,
        },
    };

    let num_str = matches.value_of("method");
    let selectionmethod = match num_str {
        None => 0,
        Some(s) => match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 1,
        },
    };

    let mut rngseed = None;
    let isstatic = matches.is_present("static");
    //eprintln!("Static {:?}", isstatic);
    if isstatic {
        rngseed = Some([42; 32]);
    }

    let textoutput = matches.is_present("textoutput");
    let svgoutput = matches.is_present("svgoutput");

    // Generate the maze
    let start = Instant::now();
    let gentype = matches.value_of("type").unwrap().to_lowercase();
    let actualtype: String;
    let maze = match gentype.as_str() {
        "ellers" | "eller" => {
            actualtype = String::from("Eller's");
            let mut generator = EllersGenerator::new(rngseed);
            generator.generate(width, height)
        }
        "gt" | "growing" | "growingtree" => {
            actualtype = String::from("Growing tree");
            let mut generator = GrowingTreeGenerator::new(rngseed);
            generator.set_selectionmethod(selectionmethod);
            generator.generate(width, height)
        }
        "prim" | "prims" => {
            actualtype = String::from("Prim's");
            let mut generator = PrimsGenerator::new(rngseed);
            generator.generate(width, height)
        }
        _ => {
            // Default to RbGenerator, so no need to specify it
            actualtype = String::from("Recursive backtracing");
            let mut generator = RbGenerator::new(rngseed);
            generator.generate(width, height)
        }
    };

    let duration = start.elapsed();
    eprintln!(
        "Generate {:?}({}) as static {:?}, Size {} x {} in time {:?}",
        actualtype, gentype, isstatic, width, height, duration
    );

    if textoutput {
        print!("{:?}", maze);
    }

    // SVG generation options, lurid green lines and a smaller cellsize, use the defaults for everything else
    let myoptions = SVGoptions {
        strokecol: String::from("green"),
        padding: 8,
        ..Default::default()
    };

    if svgoutput {
    if svgoutput {
        let svg: String = match maze.to_svg(myoptions) { // Use my options
        //let svg: String = match maze.to_svg(SVGoptions::new()) { // Use default options
            Ok(val) => val,
            Err(_) => "".to_string(),
        };
        println!("{}", svg);
    }
}
