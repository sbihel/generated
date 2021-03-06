use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("cloudasset1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200613")
            .about("The cloud asset API manages the history and inventory of cloud resources.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut feeds0 = SubCommand::with_name("feeds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a feed in a parent project/folder/organization to listen to its\nasset updates.");
            feeds0 = feeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an asset feed.");
            feeds0 = feeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details about an asset feed.");
            feeds0 = feeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all asset feeds in a parent project/folder/organization.");
            feeds0 = feeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an asset feed configuration.");
            feeds0 = feeds0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut v_10 = SubCommand::with_name("v_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get_assets_history and export_assets");
        {
            let mcmd = SubCommand::with_name("batch_get_assets_history").about("Batch gets the update history of assets that overlap a time window.\nFor IAM_POLICY content, this API outputs history when the asset and its\nattached IAM POLICY both exist. This can create gaps in the output history.\nOtherwise, this API outputs history with asset in both non-delete or\ndeleted status.\nIf a specified asset does not exist, this API returns an INVALID_ARGUMENT\nerror.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export_assets").about("Exports assets with time and resource types to a given Cloud Storage\nlocation/BigQuery table. For Cloud Storage location destinations, the\noutput format is newline-delimited JSON. Each line represents a\ngoogle.cloud.asset.v1.Asset in the JSON format; for BigQuery table\ndestinations, the output table stores the fields in asset proto as columns.\nThis API implements the google.longrunning.Operation API\n, which allows you to keep track of the export. We recommend intervals of\nat least 2 seconds with exponential retry to poll the export operation\nresult. For regular-size resource parent, the export operation usually\nfinishes within 5 minutes.");
            v_10 = v_10.subcommand(mcmd);
        }
        app = app.subcommand(v_10);
        app = app.subcommand(operations0);
        app = app.subcommand(feeds0);

        Self { app }
    }
}
use google_cloudasset1 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
