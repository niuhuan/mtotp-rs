use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "mtotp", name = "mtotp")]
pub enum MtotpCli {
    List(ListArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    Rename(RenameArgs),
    Qr(QrArgs),
}

#[derive(Debug, clap::Args)]
#[command(about = "List registered totp and codes", long_about = None)]
pub(crate) struct ListArgs {}

#[derive(clap::Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "Add new totp", long_about = None)]
pub(crate) struct AddArgs {
    #[arg()]
    pub url_or_key: Option<String>,
}

#[derive(clap::Args)]
#[command(about = "Remove totp", long_about = None)]
pub(crate) struct RemoveArgs {}

#[derive(clap::Args)]
#[command(about = "Rename a totp label", long_about = None)]
pub(crate) struct RenameArgs {}

#[derive(clap::Args)]
#[command(about = "Show qr code of a totp", long_about = None)]
pub(crate) struct QrArgs {}
