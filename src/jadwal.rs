use std;
use crate::errors::Error_Type;
use rand::Rng;
extern crate chrono;
use chrono::{DateTime, TimeZone, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct JamPerSesi
{
	id : u32,
	awalsesi: u64,
	akhirsesi: u64,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct AlokasiJadwalSemester
{
	awalsemester : u64,
	akhirsemester : u64,
	lamasemester : u64,
	jampersesi : Vec<JamPerSesi>,
}

impl AlokasiJadwalSemester
{
	pub fn alokasi_semester(awal: &str, akhir : &str) -> Result<Self, Error_Type>
	{
		let currenttime  = chrono::Local::now().timestamp();
		let reswaktuawal =  chrono::DateTime::parse_from_str(&format!("{} +0700", awal), "%d-%m-%Y %H:%M:%S %z");
		let reswaktuakhir = chrono::DateTime::parse_from_str(&format!("{} +0700", akhir), "%d-%m-%Y %H:%M:%S %z");
		if reswaktuawal.is_err() || reswaktuakhir.is_err() 
		{
			println!("tidak dapat memproses hari, format: [hari]-[bulan]-[tahun] [jam]:[menit]:[detik]!");
			return Err(Error_Type::ERR_TIME_FORMAT_ERROR);
		}
		//if *waktuawal.ar_ref.unwrap() < currenttime {return Err(Error_Type::ERR_ADDED_TIME_NOT_REASONABLE)};
		let waktuawal = reswaktuawal.unwrap().timestamp() as u64;
		let waktuakhir = reswaktuakhir.unwrap().timestamp() as u64;
		
		if (waktuakhir - waktuawal ) < 0 {return Err(Error_Type::ERR_WAKTU_SEMESTER_NEGATIVE)}
		
		Ok(Self{awalsemester: waktuawal, akhirsemester: waktuakhir, lamasemester : waktuakhir - waktuawal, jampersesi: std::vec::Vec::new()})
	}
	pub fn alokasi_waktu(&mut self, jamsesi : &str, lamasesi: &str) ->  Result<(), Error_Type>
	{
		let resjamsesi =  chrono::DateTime::parse_from_str(&format!("{} +0700", jamsesi), "%d-%m-%Y %H:%M:%S %z");
		let reslamasesi = lamasesi.parse::<u64>();
		if resjamsesi.is_err() || reslamasesi.is_err()
		{
			println!("tidak dapat memproses hari");
			println!("format hari: [hari]-[bulan]-[tahun] [jam]:[menit]:[detik], format lama sesi: [waktu dalam detik]");
			return Err(Error_Type::ERR_TIME_FORMAT_ERROR);
		}
		let awalsesi = resjamsesi.unwrap().timestamp() as u64;
		let akhirsesi = awalsesi+reslamasesi.unwrap() as u64;
		let lamanya = akhirsesi - awalsesi;
		
		let mut iterator = self.jampersesi.iter();
		let id : u32 = rand::thread_rng().gen_range(0..1<<31);
		if (awalsesi as i64 - self.awalsemester as i64) < 0 {return Err(Error_Type::ERR_AWAL_SESI_TERLALU_JAUH);}
		if (self.akhirsemester as i64 - akhirsesi as i64) < 0 {return Err(Error_Type::ERR_LAMA_SESI_TERLALU_LAMA);}
		if self.lamasemester < lamanya {return Err(Error_Type::ERR_SESI_TERLALU_PANJANG);}
		let default = JamPerSesi{id: id, awalsesi : 0, akhirsesi: 0};

		let mut once = 0;	

		loop
		{
			let a = iterator.next().unwrap_or_else(|| &default );
			let b = iterator.next().unwrap_or_else(|| &default );

			if a.awalsesi == 0 && once == 1
			{
				return Err(Error_Type::ERR_TIDAK_DAPAT_ALOKASI_WAKTU);
			}

			if a.awalsesi == 0 && once == 0
			{
				break;
			}
			
			if a.awalsesi > 0 && a.akhirsesi < awalsesi &&  b.awalsesi == 0 
			{
				if (self.akhirsemester - a.akhirsesi) < 0
				{ return Err(Error_Type::ERR_TIDAK_DAPAT_ALOKASI_WAKTU);}
				break;
			}
     		else {return  Err(Error_Type::ERR_JADWAL_TUMPANG_TINDIH);}
     		
     		if b.awalsesi > 0 && (a.akhirsesi - b.awalsesi) > lamanya
			{
				break;
			}
			
			once = 1;
		}
		self.jampersesi.push(JamPerSesi{id: id, awalsesi : awalsesi, akhirsesi: akhirsesi});
		return Ok(());
	}
	pub fn show_alokasi(&self)
	{
		for jadwalsesi in self.jampersesi.iter()
		{
			let dt_awal = Utc.timestamp_opt(jadwalsesi.awalsesi as i64, 0);
			let dt_akhir = Utc.timestamp_opt(jadwalsesi.awalsesi as i64, 0);
			println!("Awal: {}, Akhir: {}", dt_awal.unwrap().to_string(), dt_akhir.unwrap().to_string());
		};
	}
}



