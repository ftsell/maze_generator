use anyhow::{ensure, Result};
use petgraph::algo;
use quickcheck::TestResult;

use crate::prelude::*;

pub(crate) fn convert_seed(seed: u128) -> [u8; 32] {
    let mut result: [u8; 32] = [0; 32];
    let seed: [u8; 16] = seed.to_ne_bytes();
    result[0..16].copy_from_slice(&seed);
    result
}

fn generate_maze(gen: &mut impl Generator, width: i32, height: i32) -> Result<Maze> {
    ensure!(width > 0 && height > 0, "Invalid size");
    gen.generate(width, height)
}

macro_rules! test_all_coordinates_have_fields {
    ($generator_name:ty) => {
        quickcheck! {
            fn test_all_coordinates_have_fields(seed: u128, width: i32, height: i32) -> quickcheck::TestResult {
                let gen = <$generator_name>::new(Some(crate::test_util::convert_seed(seed)));
                crate::test_util::test_all_coordinates_have_fields(gen, width, height)
            }
        }
    };
}

pub(crate) fn test_all_coordinates_have_fields(
    mut gen: impl Generator,
    width: i32,
    height: i32,
) -> TestResult {
    match generate_maze(&mut gen, width, height) {
        Err(e) => match e.downcast_ref::<GenericGeneratorError>() {
            Some(_) => TestResult::failed(),
            None => TestResult::discard(),
        },
        Ok(maze) => {
            for ix in 0..maze.size.0 {
                for iy in 0..maze.size.1 {
                    if maze.get_field(&(ix, iy).into()).is_none() {
                        return TestResult::failed();
                    }
                }
            }

            TestResult::passed()
        }
    }
}

macro_rules! test_route_from_start_to_goal_exists {
    ($generator_name:ty) => {
        quickcheck! {
            fn test_route_from_start_to_goal_exists(seed: u128, width: i32, height: i32) -> quickcheck::TestResult {
                let gen = <$generator_name>::new(Some(crate::test_util::convert_seed(seed)));
                crate::test_util::test_route_from_start_to_goal_exists(gen, width, height)
            }
        }
    };
}

pub(crate) fn test_route_from_start_to_goal_exists(
    mut gen: impl Generator,
    width: i32,
    height: i32,
) -> TestResult {
    match generate_maze(&mut gen, width, height) {
        Err(e) => match e.downcast_ref::<GenericGeneratorError>() {
            Some(_) => TestResult::failed(),
            None => TestResult::discard(),
        },
        Ok(maze) => {
            let start = maze.start;
            let goal = maze.goal;

            if start == goal && width == 1 && height == 1 {
                return TestResult::passed(); // degenerate example: single cell maze
            }

            if start == goal {
                return TestResult::failed();
            }

            let graph: MazeGraph = maze.into();

            quickcheck::TestResult::from_bool(algo::has_path_connecting(&graph, start, goal, None))
        }
    }
}

macro_rules! test_all_fields_connected {
    ($generator_name:ty) => {
        quickcheck! {
            fn test_all_fields_connected(seed: u128, width: i32, height: i32) -> quickcheck::TestResult {
                let gen = <$generator_name>::new(Some(crate::test_util::convert_seed(seed)));
                crate::test_util::test_all_fields_connected(gen, width, height)
            }
        }
    };
}

pub(crate) fn test_all_fields_connected(
    mut gen: impl Generator,
    width: i32,
    height: i32,
) -> TestResult {
    match generate_maze(&mut gen, width, height) {
        Err(e) => match e.downcast_ref::<GenericGeneratorError>() {
            Some(_) => TestResult::failed(),
            None => TestResult::discard(),
        },
        Ok(maze) => {
            let graph: MazeGraph = maze.into();
            TestResult::from_bool(algo::connected_components(&graph) == 1)
        }
    }
}

macro_rules! test_generation_is_deterministic {
    ($generator_name:ty) => {
        quickcheck! {
            fn test_generation_is_deterministic(seed: u128, width: i32, height: i32) -> quickcheck::TestResult {
                let gen1 = <$generator_name>::new(Some(crate::test_util::convert_seed(seed)));
                let gen2 = <$generator_name>::new(Some(crate::test_util::convert_seed(seed)));
                crate::test_util::test_generation_is_deterministic(gen1, gen2, width, height)
            }
        }
    };
}

pub(crate) fn test_generation_is_deterministic<T>(
    mut gen1: T,
    mut gen2: T,
    width: i32,
    height: i32,
) -> TestResult
where
    T: Generator,
{
    match generate_maze(&mut gen1, width, height) {
        Err(e) => match e.downcast_ref::<GenericGeneratorError>() {
            Some(_) => TestResult::failed(),
            None => TestResult::discard(),
        },
        Ok(maze1) => match generate_maze(&mut gen2, width, height) {
            Err(e) => match e.downcast_ref::<GenericGeneratorError>() {
                Some(_) => TestResult::failed(),
                None => TestResult::discard(),
            },
            Ok(maze2) => TestResult::from_bool(maze1 == maze2),
        },
    }
}
