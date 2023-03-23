
extern crate chrono;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::{vec::Vec, env, fs, str::FromStr, fmt, fmt::{Formatter, Display, Debug}};
use crate::errors::Error_Type;
//use std::time::{SystemTime, UNIX_EPOCH, Duration};


#[derive(Deserialize, Serialize)]
pub enum Matkul {MATKUL_NONE , MATKUL_MATEM,  MATKUL_KIMIA, MATKUL_FISIKA}
#[derive(Deserialize, Serialize)]
pub enum Tipe_Sesi {SESI_PRIVAT, SESI_GRUP, SESI_SMALLCLASS}



#[derive(Deserialize, Serialize)]
pub struct Tutor
{
	id		: u32,
	nama 	: String,
	matkul	: Vec<Matkul>,
	sesi_sesi : Vec<Sesi>,
	banyak_privat : u32,
	banyak_grup	  : u32,
	banyak_smallclass : u32,
}


#[derive(Deserialize, Serialize)]
pub struct Sesi
{
	id	   : u32,
	matkul : Matkul,
	jadwal : Jadwal
}


#[derive(Deserialize, Serialize)]
pub struct Jadwal
{	
	belum : bool,
	unixtime : u64,
	tipe : Tipe_Sesi,
	siswa : String
}

pub trait Tutorctl
{
	fn new_from_file(&self, path :&str) -> Vec<Tutor>;
	fn savetofile(&self, path : &str) -> Error_Type;	
}

impl Matkul
{
	fn to_string(&self) -> String
	{
		let mtkull = 
		match self
		{
			Matkul::MATKUL_NONE => "Tidak ada",
			Matkul::MATKUL_MATEM => "Matematika",
			Matkul::MATKUL_KIMIA => "Kimia",
			Matkul::MATKUL_FISIKA => "Fisika",
			_ => "Something is not right!",
		};
		mtkull.to_string()		
	}
}


impl Display for Matkul
{
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				write!(f, "{}", self.to_string())
		}
}

impl FromStr for Matkul
{
	type Err = Error_Type;
	fn from_str(strmtk : &str) -> Result<Self, Self::Err>
	{
		let mtkull = strmtk.trim().to_lowercase();
		let aaa =
		match mtkull.as_str()
		{
			"matematika" => Matkul::MATKUL_MATEM,
			"kimia" => Matkul::MATKUL_KIMIA,
			"fisika" => Matkul::MATKUL_FISIKA,
			_ => return Err(Error_Type::ERR_CANNOT_PARSE),
		};
		Ok(aaa)
	}
}

/*
pub trait StrToVectorMatkul
{
	fn str_tomatkul_vec(strmtk : &str) -> Result<Vec<Matkul>, Error_Type>;
}


impl StrToVectorMatkul for Vec<Matkul>
{
	fn str_tomatkul_vec(strmtk : &str) -> Result<Vec<Matkul>, Error_Type>
	{
		let mut vecmatkul : Vec<Matkul> = Vec::new();
		let iterator = strmtk.split(",");
		if iterator.clone().collect::<Vec<&str>>()[0].eq("")
		{
			vecmatkul.push(strmtk.parse::<Matkul>()?);
		}
		else
		{
			for a in iterator
			{	
				vecmatkul.push(a.parse::<Matkul>()?);
			}
		}
		Ok(vecmatkul)
	}
}
*/
/*
pub fn str_tomatkul_vec(strmtk : &str) -> Result<Vec<Matkul>, Error_Type>
{
	let mut vecmatkul : Vec<Matkul> = Vec::new();
	let iterator = strmtk.split(",");
	if iterator.clone().collect::<Vec<&str>>()[0].eq("")
	{
		vecmatkul.push(strmtk.parse::<Matkul>()?);
	}
	else
	{
		for a in iterator
		{	
			vecmatkul.push(a.parse::<Matkul>()?);
		}
	}
	Ok(vecmatkul)
}
*/

pub fn parse_vec_from_comma<T>(a: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug 
{
    a.split(",")
         .filter_map(|word| word.parse().ok())
         .collect()
}

impl Tipe_Sesi
{
	fn to_string(&self) -> String
	{
		let sesion =
		match self
		{
			Tipe_Sesi::SESI_GRUP => "Grup",
			Tipe_Sesi::SESI_PRIVAT => "Privat",
			Tipe_Sesi::SESI_SMALLCLASS => "Small Class",
			_ => "None",
		};
		sesion.to_string()
	}
	
}

impl Display for Tipe_Sesi
{
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				write!(f, "{}", self.to_string())
		}
}

impl Tutor
{
	pub fn new(nama : &str, matkul : Vec<Matkul>) -> Result<Tutor, Error_Type>
	{
		if (nama.clone() == "") {return Err(Error_Type::ERR_NAME_IS_EMPTY);}
		let id : u32 = rand::thread_rng().gen_range(0..1<<31);
		Ok(Self{id : id, 
			 matkul : matkul, 
			 nama : nama.to_string(),
			 sesi_sesi : std::vec::Vec::new(),
			 banyak_grup : 0,
			 banyak_privat: 0,
			 banyak_smallclass: 0,
			}
		)
	}
	
	pub fn ins_jadwal(&mut self, matkul : Matkul, tipe : Tipe_Sesi, tanggal : &str, nama_siswa: &str) -> Result<u32, Error_Type>
	{
		let id : u32 = rand::thread_rng().gen_range(0..1<<31);
		let currenttime  = chrono::Local::now().timestamp();
		let unixtime = { match chrono::DateTime::parse_from_str(&format!("{} +0700", tanggal), "%d-%m-%Y %H:%M:%S %z") 
					   {Ok(time) => time.timestamp(), Err(erx) => return Err(Error_Type::ERR_TIME_FORMAT_ERROR)}};
		if (unixtime < currenttime) {return Err(Error_Type::ERR_ADDED_TIME_NOT_REASONABLE)};
		let jadwal = Jadwal {belum : true, tipe : tipe, unixtime : unixtime as u64, siswa: nama_siswa.to_string()};
		self.sesi_sesi.push(Sesi{id : id, matkul : matkul, jadwal : jadwal});
		return Ok(id);
	}
	
	fn getjadwalid(&mut self, sesi_id : u32) -> Result<usize, Error_Type>
	{
		let jadwalindex = {match self.sesi_sesi.iter().position(|x| x.id == sesi_id) 
			{Some(index) => return Ok(index), None => return Err(Error_Type::ERR_SESSION_ID_NOT_FOUND) } };		
	}
	
	pub fn selesai_jadwal(&mut self, sesi_id : u32) -> Error_Type
	{
		let jadwalindex = 
			match self.getjadwalid(sesi_id)
			{
				Ok(index) => index,
				Err(erx) => return erx,				
			};
		if self.sesi_sesi[jadwalindex].jadwal.belum == false { return Error_Type::ERR_SUCCESS; }
		self.sesi_sesi[jadwalindex].jadwal.belum = false;
		match self.sesi_sesi[jadwalindex].jadwal.tipe
		{
			Tipe_Sesi::SESI_GRUP => self.banyak_grup +=1,
			Tipe_Sesi::SESI_PRIVAT => self.banyak_privat +=1,
			Tipe_Sesi::SESI_SMALLCLASS => self.banyak_smallclass +=1,
		}
		return Error_Type::ERR_SUCCESS;
	}
	
	pub fn del_jadwal(&mut self, sesi_id : u32) -> Error_Type
	{
		let jadwalindex = 
			match self.getjadwalid(sesi_id)
			{
				Ok(index) => index,
				Err(erx) => return erx,
			};
		self.sesi_sesi.swap_remove(jadwalindex);
		return Error_Type::ERR_SUCCESS;	
	}
	pub fn return_id(&self) -> u32
	{
		self.id
	}	
	pub fn display_jadwal(&self)
	{
		println!("Id Tutor {}", self.id );
		println!("Nama Tutor {}", self.nama);
		print!("Matkul: ");
		for mtkul in self.matkul.iter()
		{
			print!("{}, ", mtkul.to_string());
		}
		println!(".");
		println!("Banyaknya mengajar privat: {}", self.banyak_privat);
		println!("Banyaknya mengajar grup: {}", self.banyak_grup);
		println!("Banyaknya mengajar small class: {}", self.banyak_smallclass);
		println!("Banyaknya mengajar grup: {}", self.banyak_grup);
		println!("\nJadwal_jadwal yang dilakukan:");
		if (self.sesi_sesi.is_empty() == false)
		{
			for banyaksesi in self.sesi_sesi.iter()
			{
				println!("\t Sesi dengan id: {}", banyaksesi.id);
				println!("\t Jadwal: {}", banyaksesi.jadwal.unixtime);
				println!("\t Matkul: {}", banyaksesi.matkul.to_string());
				println!("\t Tipe: {}", banyaksesi.jadwal.tipe.to_string());
				println!("\t Sudah/Belum: {}", {if banyaksesi.jadwal.belum {"Belum"} else {"Sudah"} });
				println!("\n");
			}
		}
		else
		{
			println!("Belum ada jadwal!");
		}
	}
}



impl Tutorctl for Vec<Tutor>
{
	fn new_from_file(&self, path :&str) -> Vec<Tutor>
	{
		let list_of_tutor = {
			let json_contents = fs::read_to_string(path).expect("cannot open file\n");
			serde_json::from_str::<Vec<Tutor>>(&json_contents).unwrap()
			};
		list_of_tutor
	}
	
	
	fn savetofile(&self, path : &str) -> Error_Type
	{
		match std::fs::write(path, serde_json::to_string_pretty(&self).unwrap())
		{
			Ok(_) => return Error_Type::ERR_SUCCESS,
			Err(_) => return Error_Type::ERR_SESSION_ID_NOT_FOUND,
		}
		return Error_Type::ERR_SESSION_ID_NOT_FOUND;
	}
}
