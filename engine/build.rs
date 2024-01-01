use {
    std::{
        env,
        io,
    },
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon_with_id("C:\\Users\\vebsu\\source\\master_mind\\assets\\xmas_gingerbread_christmas_emoji_icon_260224.ico", "my-icon")
            .compile()?;
    }
    Ok(())
}