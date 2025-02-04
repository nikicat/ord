use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Export {
  #[arg(long, help = "Include addresses in export")]
  include_addresses: bool,
  #[arg(long, help = "Write export to <TSV>")]
  tsv: String,
}

impl Export {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;

    index.update()?;
    index.export(&self.tsv, self.include_addresses)?;

    Ok(Box::new(Empty {}))
  }
}


#[derive(Debug, Parser)]
pub(crate) struct ExportImages {
  #[arg(long, help = "Write files to <DIR>")]
  dir: String,
}

impl ExportImages {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;

    index.update()?;
    index.export_images(&self.dir)?;

    Ok(Box::new(Empty {}))
  }
}
