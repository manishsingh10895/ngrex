use exitfailure::ExitFailure;
use structopt::StructOpt;

mod helper;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short = "f", long = "feature")]
    /// Name of the feature to scaffold
    feature: String,

    #[structopt(short = "r", long = "root", parse(from_os_str))]
    /// Root path of the store
    root: Option<std::path::PathBuf>,

    #[structopt(short = "n", long = "new")]
    /// To create new store
    new_store: bool,
}

const STORE_PATH: &str = "./src/app/store";

pub fn get_component_file_path(feature: &str, component: &str) -> String {
    match component {
        "action" => format!(
            "{root}/actions/{feature}.actions.ts",
            root = STORE_PATH,
            feature = feature
        ),
        "effect" => format!("{}/effects/{}.effect.ts", STORE_PATH, feature),
        "selector" => format!("{}/selectors/{}.selectors.ts", STORE_PATH, feature),
        "reducers" => format!("{}/reducers/{}.reducer.ts", STORE_PATH, feature),
        _ => panic!("Invalid component type"),
    }
}

fn scaffold(feature: &str) -> Result<(), ExitFailure> {
    helper::create_store_component(feature, "action")?;
    helper::create_store_component(feature, "effect")?;
    helper::create_store_component(feature, "selector")?;
    helper::create_store_component(feature, "reducer")?;

    return Ok(());
}

fn main() -> Result<(), ExitFailure> {
    println!("Hello, world!");
    let options = Options::from_args();

    if options.new_store {}

    println!("{:?}", options);

    let root = match options.root {
        None => String::from(STORE_PATH),
        Some(val) => val.into_os_string().into_string().unwrap(),
    };

    if std::fs::metadata(&root).is_err() {
        std::fs::create_dir_all(&root)?
    }

    scaffold(&options.feature)?;

    return Ok(());
}
