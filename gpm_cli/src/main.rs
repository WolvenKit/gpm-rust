use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
mod commands;
use gpm_core::config::GpmConfig;

#[cfg(test)]
mod test;

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("gpm")
        .version("0.1")
        .author("TODO <TODO@users.noreply.github.com>")
        .about("Games Package Manager utility")
        .arg(
            Arg::with_name("store_path")
                .short("s")
                .takes_value(true)
                .help("the path to the store path.")
        )
        .arg(
            Arg::with_name("no_profile_change")
                .help("prevent change to the current profile by the command")
        )
        .subcommand(
            SubCommand::with_name("init")
                .version("0.1")
                .about("creates an mod project in the current directory"),
        )
        .subcommand(
            SubCommand::with_name("package")
                .about("create a redistributable archive of a mod")
                .arg(
                    Arg::with_name("input_dir")
                        .short("i")
                        .takes_value(true)
                        .help("the directory containing the mod to package"),
                )
                .arg(
                    Arg::with_name("output_file")
                        .short("o")
                        .takes_value(true)
                        .required(true)
                        .help("the output file to create"),
                ),
        )
        .subcommand(
            SubCommand::with_name("install_local")
                .about("install a mod for the current profile")
                .arg(
                    Arg::with_name("input")
                        .index(1)
                        .required(true)
                        .help("the path to the content to install. As of now, it should be a .zip file containing a packaged mod")
                )
        )
        .get_matches();

    // load configs
    let mut config = GpmConfig::load_config().unwrap();

    if let Some(custom_store_path) = matches.value_of("store_path") {
        config.store_path = PathBuf::from(custom_store_path);
    };

    //TODO: make it accept a boolean instead
    if matches.is_present("no_profile_change") {
        config.can_change_profile = true;
    };

    match matches.subcommand() {
        ("init", _) => commands::init::init(&config)?,
        ("package", Some(archive_arg)) => {
            commands::package::package(commands::package::PackageParameter {
                input_dir: PathBuf::from(archive_arg.value_of("input_dir").unwrap_or(".")),
                output_file: PathBuf::from(archive_arg.value_of("output_file").unwrap()), //unwrap: output_file is required
            })?;
        }
        ("install_local", Some(install_arg)) => commands::install_local::install_local(
            commands::install_local::InstallLocalParameter {
                input: PathBuf::from(install_arg.value_of("input").unwrap()),
            },
            config,
        )?,
        _ => println!("sub command unknown or unspecified"),
    };

    Ok(())
}
