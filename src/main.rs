/*	
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 *
 * 
 */

mod jadwal; 
mod ruangkelas;
mod systems;
mod errors;
mod comms;
mod parser;
use std;
//use crate::jadwal::AlokasiJadwalSemester;
use crate::systems::{Tutorctl, Tutor, Matkul, Tipe_Sesi};
use crate::errors::{Error_Type};
//use crate::ruangkelas::RuangKelas;
//use crate::comms::setprompt;
//use crate::parser;

fn main() -> Result<(), Error_Type>
{
/*	
	//let mut aaaa = jadwal::AlokasiJadwalSemester::alokasi_semester(1677384234, 1677989034)?;
	//let mut ruang = ruangkelas::RuangKelas::new(12123, "0", aaaa);
	let mut mytutor = Tutor::new("Wandi-Ka", [Matkul::MATKUL_FISIKA, Matkul::MATKUL_KIMIA].to_vec()).unwrap();
//	let aaa = "matematika".parse::<Matkul>()?;
//	println!("{}", aaa);
	Ok(())

	
	let mut bbbb = Vec::<Tutor>::new();
*/
	let mut aaaa = String::new();
	let mut cds = parser::Database::init();
	loop
	{
		/*
		comms::setprompt(&mut aaaa)?;
		if aaaa.eq("\n") {aaaa.clear(); continue;};
		if aaaa.is_empty() {break;};
		cds.getops(aaaa.as_str());
		let aaa = cds.deciders();
		if aaa.is_err() { if *aaa.as_ref().unwrap_err() == Error_Type::ERR_QUIT {break;}
			else {println!("{}", aaa.unwrap_err().to_string());};
			 };
		cds.cleanup();
		aaaa.clear();
		*/
	}
	//std::ptr::null_mut::<i32>().write(42);
	println!("Bye ^_^");
	Ok(())

}
