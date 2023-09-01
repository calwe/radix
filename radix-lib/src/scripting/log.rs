use log::{debug, error, info, trace, warn};
use mlua::{Lua, Result};

pub fn trace(_: &Lua, message: String) -> Result<()> {
    trace!("{}", message);
    Ok(())
}

pub fn debug(_: &Lua, message: String) -> Result<()> {
    debug!("{}", message);
    Ok(())
}

pub fn info(_: &Lua, message: String) -> Result<()> {
    info!("{}", message);
    Ok(())
}

pub fn warn(_: &Lua, message: String) -> Result<()> {
    warn!("{}", message);
    Ok(())
}

pub fn error(_: &Lua, message: String) -> Result<()> {
    error!("{}", message);
    Ok(())
}
