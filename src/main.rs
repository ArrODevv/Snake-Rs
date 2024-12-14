use snake::init;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = core::result::Result<T, E>;

fn main() -> Result<()> {
    let app = init()?;

    println!("{:?}", app);

    Ok(())
}
