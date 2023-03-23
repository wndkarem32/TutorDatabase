//mod errors;
use std::net;
use std::io;
use std::io::prelude::*;
use std::io::{Write, BufRead};
use crate::errors::Error_Type;

fn prompt(buffer : &mut String) -> io::Result<()>
{
	let mut wlock = io::stdout().lock();
	wlock.write_all(b"Prompt>")?;
	wlock.flush();
	let mut rlock = io::stdin().lock();
	rlock.read_line(buffer)?;
    Ok(())
}

pub fn setprompt(buffer : &mut String) -> Result<() , Error_Type>
{
	match prompt(buffer)
	{
		Ok(la) => return Ok(()),
		Err(la) => return Err(Error_Type::ERR_IO_ERROR),
	}
}
