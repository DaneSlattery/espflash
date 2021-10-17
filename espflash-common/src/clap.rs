use clap::Clap;

#[derive(Clap)]
pub struct ConnectArgs {
    /// Serial port connected to target device
    pub serial: Option<String>,
    /// Baud rate at which to flash target device
    #[clap(long)]
    pub speed: Option<usize>,
}

#[derive(Clap)]
pub struct BuildArgs {
    /// Build the application using the release profile
    #[clap(long)]
    pub release: bool,
    /// Example to build and flash
    #[clap(long)]
    pub example: Option<String>,
    /// Comma delimited list of build features
    #[clap(long, use_delimiter(true))]
    pub features: Option<Vec<String>>,
    /// Image format to flash
    #[clap(long)]
    pub format: Option<String>,
}

#[derive(Clap)]
pub struct FlashArgs {
    /// Load the application to RAM instead of Flash
    #[clap(long)]
    pub ram: bool,
    /// Display the connected board's information (deprecated, use the `board-info` subcommand instead)
    #[clap(long)]
    pub board_info: bool,
    /// Path to a binary (.bin) bootloader file
    #[clap(long)]
    pub bootloader: Option<String>,
    /// Path to a CSV file containing partition table
    #[clap(long)]
    pub partition_table: Option<String>,
    /// Open a serial monitor after flashing
    #[clap(long)]
    pub monitor: bool,
}

/// Save the image to disk instead of flashing to device
#[derive(Clap)]
pub struct SaveImageOpts {
    #[clap(flatten)]
    pub build_args: BuildArgs,
    /// File name to save the generated image to
    pub file: String,
}

/// Display the connected board's information
#[derive(Clap)]
pub struct BoardInfoOpts {
    #[clap(flatten)]
    pub connect_args: ConnectArgs,
}
