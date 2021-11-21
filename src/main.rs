use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct MakeCookie {
    /// set supervising faerie <String>
    #[structopt(name = "supervisor", default_value = "Puck", short = "s", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32,
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        /// magicality <number>
        #[structopt(short, parse(from_occurrences))]
        magicality: u64,
        /// color <string>>
        #[structopt(short)]
        color: String,
    },
    Finish(Finish),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt, Debug)]
struct Finish {
    /// <time> is expressed as an integer number
    #[structopt(short)]
    time: u32,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    /// finish_type is a Glaze and a Powder
    finish_type: FinishType,
}

// subsubcommand!
#[derive(StructOpt, Debug)]
enum FinishType {
    Glaze {
        /// <applications> is expressed as an integer number
        applications: u32,
    },
    Powder {
        flavor: String,
        dips: u32,
    }
}

fn main() {
    let command = Command::from_args();
    println!("{:?}", command);

    let make_cookie = MakeCookie::from_args();
    println!("{:?}", make_cookie);
}
