pub use error::{Error, Result};

mod error;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
	tagged_status!("[MAIN]", "booting...");


	Err(Error::MainLoopClosed)
}
