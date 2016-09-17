#[macro_use]
extern crate clap;


fn vals_to_cmds(submatch: &clap::ArgMatches, cmd: &str, arg: &str) -> String {
    // Raw values from the list
    let values: Vec<&str> = submatch.values_of(arg)
                                    .unwrap()
                                    .collect();
    // Values transformed into Julia commands
    let jlcmds = values.iter()
                       .map(|&x| format!("Pkg.{}(\"{}\")", cmd, &x))
                       .collect::<Vec<_>>();
    // Create a single-line-friendly statement
    return jlcmds.join("; ");
}


fn pkgcmd(matches: &clap::ArgMatches) -> String {
    match matches.subcommand() {
        ("install", Some(cmdargs)) => {
            if cmdargs.is_present("git") {
                return format!("Pkg.clone(\"{}\")", cmdargs.value_of("git").unwrap());
            } else {
                return vals_to_cmds(&cmdargs, "packages", "add");
            }
        },

        ("remove", Some(cmdargs)) => {
            return vals_to_cmds(&cmdargs, "packages", "rm");
        },

        ("update", Some(_)) => {
            return "Pkg.update()".to_string();
        },

        ("list", Some(cmdargs)) => {
            if cmdargs.is_present("packages") {
                return vals_to_cmds(&cmdargs, "packages", "status");
            } else {
                return "Pkg.status()".to_string();
            }
        },

        // Unprovided subcommand is guaranteed by clap to produce a help message
        // so this doesn't need to be handled separately
        _ => unreachable!()
    };
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let clargs = clap::App::from_yaml(yaml).get_matches();

    let cmd = pkgcmd(&clargs);

    let output = std::process::Command::new("julia")
                                       .arg("-e")
                                       .arg(&cmd)
                                       .output()
                                       .expect("Failed to start Julia");

    let code: i32 = output.status.code().unwrap_or(0);

    if output.status.success() {
        // If Julia has nothing to say, assume things are fine
        if output.stdout.is_empty() && output.stderr.is_empty() {
            println!("Commands executed successfully.");
        } else {
            // We're interested in anything and everything Julia wants to say
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    } else {
        println!("Julia commands failed: {}", cmd);
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    std::process::exit(code);
}
