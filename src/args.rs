use clap::Parser;

#[derive(Parser)]
pub struct Piperviewer {
    pub infile: Option<String>,

    #[arg(short, long)]
    pub outfile: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue, env="PV_SILENT")]
    pub silent: bool,
}

impl Piperviewer {
    pub fn new() -> Self {
        let cli = Piperviewer::parse();
        // let file_name = match cli.infile {
        //     Some(file_name) => file_name,
        //     None => String::default(),
        // };
        // let output = match cli.outfile {
        //     Some(file_output) => file_output,
        //     None => String::default()
        // };
        let file_name = Piperviewer::check_arg(cli.infile.as_deref());
        let file_output = Piperviewer::check_arg(cli.outfile.as_deref());
        Self {
            infile: Some(file_name),
            outfile: Some(file_output),
            silent: cli.silent
        }
    }

    fn check_arg(arg: Option<&str>) -> String {
        match arg {
            Some(i) => i.to_owned(),
            None => String::default()
        }
    }
}

impl Default for Piperviewer {
    fn default() -> Self {
        Self {
            infile: None,
            outfile: None,
            silent: false
        }
    }
}