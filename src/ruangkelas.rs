//mod jadwal;
use crate::jadwal::AlokasiJadwalSemester;
use crate::errors::Error_Type;
use crate::systems::Tutor;
use serde_derive::{Deserialize, Serialize};
use rand::Rng;

#[derive(Deserialize, Serialize)]
pub struct RuangKelas
{
	ruangid: u32,
	ruangan: String,
	jampersesi : AlokasiJadwalSemester,
	tutor : Option<u32>,
}

impl RuangKelas
{
	pub fn new(ruangan : &str, alokasijadwal : AlokasiJadwalSemester) -> RuangKelas
	{
		let ruangid : u32 = rand::thread_rng().gen_range(0..1<<31);
		Self{ruangid : ruangid, ruangan : ruangan.to_string(), jampersesi : alokasijadwal, tutor: None}
		
	}
	pub fn addjadwal(&mut self, jamsesi : &str, lamasesi: &str, tutorid : u32) -> Result<(), Error_Type>
	{
		self.jampersesi.alokasi_waktu(jamsesi, lamasesi)?;
		self.tutor = Some(tutorid);
		Ok(())
	}
	pub fn showid(&self) -> u32
	{
		self.ruangid
	}
}

