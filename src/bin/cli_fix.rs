use label_fixer::*;
use std::env::args;

fn main() -> Result<(), Error> {
    let Some(path) = args().nth(1) else {
        return Err(Error::Other("must provide a file path"));
    };

    let out_path = fix_label(path, Source::Vinted)?;

    println!("{}", out_path.display());

    Ok(())
}
