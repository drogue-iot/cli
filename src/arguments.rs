use crate::{util, AppId};

use crate::config::Context;
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::convert::AsRef;
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Verbs {
    create,
    delete,
    edit,
    get,
    set,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Set_targets {
    gateway,
    password,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Set_args {
    username,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Resources {
    device,
    devices,
    app,
    apps,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Parameters {
    url,
    id,
    spec,
    config,
    filename,
    context,
    #[strum(serialize = "keep-current")]
    keep_current,
    labels,
    context_name,
    #[strum(serialize = "key-output")]
    key_output,
    #[strum(serialize = "ca-key")]
    ca_key,
    out,
    days,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Other_commands {
    login,
    token,
    version,
    whoami,
    context,
    trust,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Context_subcommands {
    list,
    show,
    #[strum(serialize = "set-active")]
    set_active,
    delete,
    create,
    #[strum(serialize = "set-default-app")]
    set_default_app,
    rename,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Trust_subcommands {
    create,
    add,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Other_flags {
    verbosity,
    cert,
}

pub fn parse_arguments() -> ArgMatches<'static> {
    let resource_id_arg = Arg::with_name(Parameters::id.as_ref())
        .required(true)
        .help("The unique id of the resource.");

    let set_arg = Arg::with_name(Verbs::set.as_ref())
        .required(true)
        .multiple(true)
        .number_of_values(2)
        .value_names(&["device","value"])
        .help("For gateway value is the device id of the gateway, for setting a password credential, value is the password");

    let url_arg = Arg::with_name(Parameters::url.as_ref())
        .required(true)
        .value_name("URL")
        .help("The url of the drogue cloud api endpoint");

    let set_password_username = Arg::with_name(Set_args::username.as_ref())
        .short("u")
        .long("username")
        .required(false)
        .takes_value(true)
        .value_name("username")
        .help("The username associated with the password");

    let app_id_arg = Arg::with_name(Resources::app.as_ref())
        .short("a")
        .long(Resources::app.as_ref())
        .takes_value(true)
        .env("DRG_APP")
        .help("The app owning the device. Can be set with DRG_APP environment variable.");

    let spec_arg = Arg::with_name(Parameters::spec.as_ref())
        .short("s")
        .long(Parameters::spec.as_ref())
        .alias("data")
        .takes_value(true)
        .help("The spec for the resource. --data is deprecated.");

    let file_arg = Arg::with_name(Parameters::filename.as_ref())
        .short("f")
        .long(Parameters::filename.as_ref())
        .takes_value(true)
        .value_name("FILE")
        .conflicts_with(Parameters::spec.as_ref())
        .help("File containing the data to create or update the resource with.")
        .long_help("File containing the data to create or update the resource with. \
            Note: unlike the --spec argument which cover only the spec section of the resource, \
            the file provided with --file must contains the complete resource object, including metadata.");

    let token_arg = Arg::with_name(Other_commands::token.as_ref())
        .short("t")
        .takes_value(true)
        .long(Other_commands::token.as_ref())
        .help("Refresh token for authentication.");

    let config_file_arg = Arg::with_name(Parameters::config.as_ref())
        .long(Parameters::config.as_ref())
        .short("C")
        .takes_value(true)
        .global(true)
        .value_name("FILE")
        .help("Path to the drgconfig file. If not specified, reads $DRGCFG environment variable or defaults to XDG config directory for drg_config.json");

    let verbose = Arg::with_name(Other_flags::verbosity.as_ref())
        .short("v")
        .takes_value(false)
        .multiple(true)
        .global(true)
        .help("Enable verbose output. Multiple occurences increase verbosity.");

    let context_arg = Arg::with_name(Parameters::context.as_ref())
        .long(Parameters::context.as_ref())
        .short("c")
        .takes_value(true)
        .global(true)
        .env("DRG_CONTEXT")
        .help("The name of the context to use. Can be set with DRG_CONTEXT environment variable.");

    let context_id_arg = Arg::with_name(Parameters::context_name.as_ref())
        .conflicts_with(Parameters::context.as_ref())
        .required(true)
        .help("The id of the context");

    let login_keep_current = Arg::with_name(Parameters::keep_current.as_ref())
        .short("k")
        .help("Do not activate the new context.");

    let labels = Arg::with_name(&Parameters::labels.as_ref())
        .required(false)
        .short("l")
        .long(Parameters::labels.as_ref())
        .use_delimiter(true)
        .multiple(true)
        .help("A comma separated list of the label filters to filter the list with.");

    let keyout = Arg::with_name(&Parameters::key_output.as_ref())
        .takes_value(true)
        .required(false)
        .long(Parameters::key_output.as_ref())
        .help("Output file containing the private key. Later to be used to sign device certificates, or device authentication.");

    let device_id_arg = Arg::with_name(&Resources::device.as_ref())
        .short("d")
        .required(true)
        .long(&Resources::device.as_ref())
        .takes_value(true)
        .help("Device id");

    let ca_key = Arg::with_name(&Parameters::ca_key.as_ref())
        .long(&Parameters::ca_key.as_ref())
        .takes_value(true)
        .required(true)
        .help("Private key of the CA i.e application.");

    let cert_out = Arg::with_name(&Parameters::out.as_ref())
        .long(&Parameters::out.as_ref())
        .short("o")
        .takes_value(true)
        .required(false)
        .help("Output device certificate to file.");

    let device_name_subj = Arg::with_name(&Other_flags::cert.as_ref())
        .long(&Other_flags::cert.as_ref())
        .takes_value(false)
        .help("Creates device with the same name as the subject of device certificate.")
        .long_help(
            "X.509 authentication requires that the name of the device should \
            to be equal to the subject of the device's certificate. This flag \
            converts the given device name into the required format.",
        );

    // Default value comes from trust::CERT_VALIDITY_DAYS
    let cert_valid_days = Arg::with_name(&Parameters::days.as_ref())
        .long(&Parameters::days.as_ref())
        .takes_value(true)
        .required(false)
        .help("Number of days the certificate should be valid for. [default: 365]")
        .validator(|n| match n.parse::<u64>() {
            Err(_) => Err(String::from("The value is not an integer")),
            Ok(_) => Ok(()),
        });

    App::new("Drogue Command Line Tool")
        .version(util::VERSION)
        .author("Jb Trystram <jbtrystram@redhat.com>")
        .about("Allows to manage drogue apps and devices in a drogue-cloud instance")
        .arg(config_file_arg)
        .arg(verbose)
        .arg(&context_arg)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(Verbs::create.as_ref())
                .visible_alias("add")
                .about("create a resource in the drogue-cloud registry")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Resources::device.as_ref())
                        .about("create a device.")
                        .arg(&resource_id_arg)
                        .arg(&app_id_arg)
                        .arg(&spec_arg)
                        .arg(&file_arg)
                        .arg(&device_name_subj),
                )
                .subcommand(
                    SubCommand::with_name(Resources::app.as_ref())
                        .about("create an app.")
                        .arg(&resource_id_arg)
                        .arg(&spec_arg)
                        .arg(&file_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name(Verbs::delete.as_ref())
                .visible_alias("remove")
                .about("delete a resource in the drogue-cloud registry")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Resources::device.as_ref())
                        .about("delete a device.")
                        .arg(&resource_id_arg)
                        .arg(&app_id_arg),
                )
                .subcommand(
                    SubCommand::with_name(Resources::app.as_ref())
                        .about("delete an app.")
                        .arg(&resource_id_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name(Verbs::get.as_ref())
                .about("Display one or many resources from the drogue-cloud registry")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Resources::device.as_ref())
                        .about("Retrieve a device spec.")
                        .arg(resource_id_arg.clone().required(false))
                        .arg(&app_id_arg),
                )
                .subcommand(
                    SubCommand::with_name(Resources::app.as_ref())
                        .about("retrieve an app spec.")
                        .arg(resource_id_arg.clone().required(false)),
                )
                // Listing subcommands
                .subcommand(
                    SubCommand::with_name(Resources::apps.as_ref())
                        .about("List all apps.")
                        .arg(&labels)
                        .about("List all apps the user have access to.")
                        .arg(resource_id_arg.clone().required(false)),
                )
                .subcommand(
                    SubCommand::with_name(Resources::devices.as_ref())
                        .arg(&app_id_arg)
                        .arg(&labels)
                        .about("List all devices for an app.")
                        .arg(resource_id_arg.clone().required(false)),
                ),
        )
        .subcommand(
            SubCommand::with_name(Verbs::edit.as_ref())
                .visible_alias("update")
                .about("Update a resource from the drogue-cloud registry")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Resources::device.as_ref())
                        .about("Edit a device spec.")
                        .arg(&resource_id_arg)
                        .arg(&app_id_arg)
                        .arg(&file_arg),
                )
                .subcommand(
                    SubCommand::with_name(Resources::app.as_ref())
                        .about("Edit an app spec.")
                        .arg(&resource_id_arg)
                        .arg(&file_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name(Verbs::set.as_ref())
                .about("Configure apps or devices resources")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Set_targets::gateway.as_ref())
                        .about("Set a gateway for a device.")
                        .arg(&set_arg)
                        .arg(&app_id_arg),
                )
                .subcommand(
                    SubCommand::with_name(Set_targets::password.as_ref())
                        .about("Set a password credentials for a device")
                        .arg(&set_arg)
                        .arg(&app_id_arg)
                        .arg(&set_password_username),
                ),
        )
        .subcommand(
            SubCommand::with_name(Other_commands::version.as_ref())
                .about("Print version information."),
        )
        .subcommand(
            SubCommand::with_name(Other_commands::login.as_ref())
                .arg(&token_arg)
                .about("Log into a drogue cloud installation.")
                .arg(&url_arg)
                .arg(&login_keep_current),
        )
        .subcommand(
            SubCommand::with_name(Other_commands::whoami.as_ref())
                .about("Print cluster adress, version and default app(if any)")
                .arg(
                    token_arg
                        .clone()
                        .takes_value(false)
                        .help("print a valid bearer token for the drogue cloud instance."),
                ),
        )
        .subcommand(
            SubCommand::with_name(Other_commands::context.as_ref())
                .about("Manage contexts in the configuration file.")
                .alias("config")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Context_subcommands::create.as_ref())
                        .setting(AppSettings::Hidden)
                        .help("This subcommand is invalid. To create a new context use drg login."),
                )
                .subcommand(
                    SubCommand::with_name(Context_subcommands::list.as_ref())
                        .about("List existing contexts names in configuration file"),
                )
                .subcommand(
                    SubCommand::with_name(Context_subcommands::show.as_ref())
                        .about("Show full configuration file"),
                )
                .subcommand(
                    SubCommand::with_name("set-active")
                        .about("Set a context as the active context")
                        .arg(&context_id_arg),
                )
                .subcommand(
                    SubCommand::with_name(Context_subcommands::delete.as_ref())
                        .alias("remove")
                        .about("Set a context as the active context")
                        .arg(&context_id_arg),
                )
                .subcommand(
                    SubCommand::with_name("set-default-app")
                        .about("Set a default-app for a context.")
                        .arg(&resource_id_arg),
                )
                .subcommand(
                    SubCommand::with_name(Context_subcommands::rename.as_ref())
                        .about("Rename a context.")
                        .arg(&context_id_arg)
                        .arg(
                            Arg::with_name("new_context_id")
                                .required(true)
                                .help("The new context name"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name(Other_commands::trust.as_ref())
                .about("Manage trust-anchors and device certificates.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(Trust_subcommands::create.as_ref())
                        .about("Create a trust-anchor for an application.")
                        .arg(&app_id_arg)
                        .arg(&keyout)
                        .arg(&cert_valid_days),
                )
                .subcommand(
                    SubCommand::with_name(Trust_subcommands::add.as_ref())
                        .about("Signs device certificate using application's private key.")
                        .arg(&app_id_arg)
                        .arg(&device_id_arg)
                        .arg(&ca_key)
                        .arg(&cert_out)
                        .arg(&keyout)
                        .arg(&cert_valid_days),
                ),
        )
        .get_matches()
}

pub fn get_app_id<'a>(matches: &'a ArgMatches, config: &'a Context) -> Result<AppId> {
    match matches.value_of(Resources::app) {
        Some(a) => Ok(a.to_string()),
        None => config
            .default_app
            .as_ref()
            .map(|v| {
                println!("Using default app \"{}\".", &v);
                v.to_string()
            })
            .ok_or_else(|| {
                anyhow!("Missing app argument and no default app specified in config file.")
            }),
    }
}
