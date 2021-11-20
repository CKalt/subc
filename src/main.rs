use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "RSHOT")]
struct Opt {
    /// Dump configuration and exit.
    #[structopt(short = "g", long = "show-config")]
    show_config: bool,
    #[structopt(short = "f", long = "config-file")]
    config_file: Option<String>,
}

#[allow(dead_code)]
#[derive(StructOpt, Debug)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Git {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "i")]
        interactive: bool,
        #[structopt(short = "p")]
        patch: bool,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>
    },
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(long = "dry-run")]
        dry_run: bool,
        #[structopt(long = "all")]
        all: bool,
        repository: Option<String>
    },
    #[structopt(name = "commit")]
    Commit {
        #[structopt(short = "m")]
        message: Option<String>,
        #[structopt(short = "a")]
        all: bool
    }
}

fn main () {
    let opt = Opt::from_args();
    println!("opt={:?}", opt);

    let git = Git::from_args();
    println!("git={:?}", git);

    /*

    match Git::from_args() {
        Git::Add { interactive, patch, files } => {
            println!("interactive={:?}", interactive);
            println!("patch={:?}", patch);
            println!("files={:?}", files);
        },
        Git::Commit { message, all } => {
            println!("message={:?}", message);
            println!("all={:?}", all);
        }
        _ => (),
    }
    */
}

