use std::ops::{Bound, RangeBounds};
use std::process::Command;
use std::str;
use GengOption::*;

const SUPPRESS_AUXILIARY_OUTPUT: &str = "-q";
const GENG_EXECUTABLE: &str = "geng";

pub enum GengOption {
    Connected,
    Biconnected,
    TriangleFree,
    FourCycleFree,
    FiveCycleFree,
    K4Free,
    Chordal,
    Split,
    Perfect,
    ClawFree,
    Bipartite,
    MinimumDegree(usize),
    MaximumDegree(usize),
    SaveMemory,
}

impl GengOption {
    fn to_string(&self) -> String {
        match self {
            Connected => String::from("-c"),
            Biconnected => String::from("-C"),
            TriangleFree => String::from("-t"),
            FourCycleFree => String::from("-f"),
            FiveCycleFree => String::from("-p"),
            K4Free => String::from("-k"),
            Chordal => String::from("-t"),
            Split => String::from("-S"),
            Perfect => String::from("-P"),
            ClawFree => String::from("-F"),
            Bipartite => String::from("-b"),
            MinimumDegree(n) => format!("-d{}", n),
            MaximumDegree(n) => format!("-D{}", n),
            SaveMemory => String::from("-m"),
        }
    }
}

pub fn call_geng_with_args(
    vertices: usize,
    m: impl RangeBounds<usize>,
    options: &[GengOption],
) -> String {
    let mut args: Vec<String> = options.iter().map(|option| option.to_string()).collect();
    args.push(vertices.to_string());
    let left = match m.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => x + 1,
        Bound::Unbounded => 0,
    };
    let right = match m.end_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => x - 1,
        Bound::Unbounded => (vertices * (vertices - 1)) / 2,
    };
    args.push(dbg!(format!("{}:{}", left, right)));
    args.push(SUPPRESS_AUXILIARY_OUTPUT.to_owned());
    get_command_output_string(Command::new(GENG_EXECUTABLE).args(args))
}

fn get_command_output_string(command: &mut Command) -> String {
    let output = command.output().expect("failed to execute process");
    to_string(&output.stdout)
}

fn to_string(vec: &[u8]) -> String {
    str::from_utf8(&vec).expect("Invalid UTF-8").to_string()
}
