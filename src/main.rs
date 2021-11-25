use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// set supervising faerie <String>
    #[structopt(name = "supervisor", default_value = "Puck", short = "s", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    sub_cmd: OptSubCmd,
}

#[derive(StructOpt, Debug)]
enum OptSubCmd {
    /// Pound acorns into flour for cookie dough.
    Pound {
        /// <acorns> is expressed as a numeric integer
        acorns: u32,
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        /// magicality is expressed as a numeric integer (u64)
        #[structopt(short, parse(from_occurrences))]
        magicality: u64,
        /// color <string>>
        #[structopt(short)]
        color: String,
    },
    #[structopt(name="job")]
    JobCmd(JobSubCmd),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt, Debug)]
struct JobSubCmd {
    /// <time> is expressed as an integer number
    #[structopt(short)]
    time: u32,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    /// job_cmd_type is a Glaze and a Powder
    job_cmd_type: JobCmdType,
}

// subsubcommand!
#[derive(StructOpt, Debug)]
enum JobCmdType {
    Show {
        /// <applications> is expressed as an integer number
        applications: u32,
    },
    List {
        flavor: String,
        dips: u32,
    }
}

fn main() {
//    let command = OptSubCmd::from_args();
//    println!("{:?}", command);

    let opt = Opt::from_args();
    println!("{:?}", opt);
}
