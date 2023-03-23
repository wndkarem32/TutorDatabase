use crate::systems::{Tutor, parse_vec_from_comma};
use crate::jadwal::{AlokasiJadwalSemester};
use crate::ruangkelas::RuangKelas;
use crate::errors::Error_Type;
use std::{str::FromStr, fs};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum Operation {
	OPS_NOOP,
	OPS_ADD_TUTOR,
	OPS_ADD_JADWAL,
	OPS_DEL_TUTOR,
	OPS_DEL_JADWAL,
	OPS_SHOW_JADWAL,
	OPS_PILIH_TABEL,
	OPS_QUIT,
	OPS_SHOW_TUTOR,
	OPS_INIT_JADWAL,
	OPS_ADD_RUANGAN,
	OPS_SAVE_TO_FILE,
	OPS_LOAD_TO_FILE,
	}

#[derive(Deserialize, Serialize)]
pub struct Database
{
	op	: Operation,
	args : String,
	listtutor :  Vec<Tutor>,
	lsjadwal : Option<AlokasiJadwalSemester>,
	lskelas	: Vec<RuangKelas>,
}

#[derive(Deserialize, Serialize)]
struct FileDatabase
{
	listtutor :  Vec<Tutor>,
	lsjadwal : Option<AlokasiJadwalSemester>,
	lskelas	: Vec<RuangKelas>,
}

impl FromStr for Operation
{
	type Err = Error_Type;
	fn from_str(parsestr : &str) -> Result<Self, Self::Err>
	{
		let a = 
		match parsestr.to_lowercase().as_str()
		{
			"pilihtabel" => Operation::OPS_PILIH_TABEL,
			"addtutor" => Operation::OPS_ADD_TUTOR,
			"deltutor" => Operation::OPS_DEL_TUTOR,
			"showtutor" => Operation::OPS_SHOW_TUTOR,
			"initjadwal" => Operation::OPS_INIT_JADWAL,
			"addjadwal" => Operation::OPS_ADD_JADWAL,
			"deljadwal" => Operation::OPS_DEL_JADWAL,
			"showjadwal" => Operation::OPS_SHOW_JADWAL,
			"addruangan" => Operation::OPS_ADD_RUANGAN,
			"save" => Operation::OPS_SAVE_TO_FILE,
			"load" => Operation::OPS_LOAD_TO_FILE,
			"quit" => Operation::OPS_QUIT,
			_ => return Err(Error_Type::ERR_UNIMPLEMENTED),
		};
		Ok(a)
	}
}

impl Database
{
	pub fn init() -> Self
	{
		Self{op : Operation::OPS_NOOP, args : String::new(), listtutor : Vec::new(), lsjadwal : None, lskelas : Vec::new()}
	}
/*
	pub fn getops(&mut self, argmt: &str) -> Result<(), Error_Type>
	{
		//println!("{}", argmt.clone());
		let binding = argmt.clone().to_lowercase();
		let args = binding.trim().split_once(":").ok_or_else(|| Error_Type::ERR_CANNOT_PARSE)?;	
		self.op = args.0.parse::<Operation>()?;
		self.args.push_str(args.1);
		Ok(())
	}

	pub fn deciders(&mut self) -> Result<(), Error_Type>
	{	
		let mut vecofargs = Vec::<(&str , &str)>::new();
		for als in self.args.as_str().split("&")
		{
			let argspair : (&str , &str) = als.clone().split_once("=").unwrap_or_else(|| (als, ""));
			vecofargs.push(argspair);
		}
		match self.op
		{
			Operation::OPS_ADD_TUTOR => 
			{
				let mut tutorname : Option<&str> = None;
				let mut matkul : Option<&str> = None;
				for args in vecofargs.iter()
				{
					match args.0
					{
						"nametutor" => 
						{
							tutorname = Some(args.1);
						},
						"matkul" => 
						{
							matkul = Some(args.1);
						},
						_ => {/*return Err(Error_Type::ERR_IO_ERROR)*/},
					};
				}
				let buftutor = Tutor::new(tutorname.ok_or_else(|| {println!("Error: nama tutor tidak ada"); Error_Type::ERR_CANNOT_PARSE})?,
				parse_vec_from_comma(matkul.ok_or_else(|| {println!("Error: nama matkul tidak ada"); Error_Type::ERR_CANNOT_PARSE})?))?;
				println!("Success, tutor id :{}", buftutor.return_id());
				self.listtutor.push(buftutor);
			},
			Operation::OPS_DEL_TUTOR =>
			{
				let mut tutorid : Option<&str> = None;
				for args in vecofargs.iter()
				{
					match args.0
					{
						"tutorid" => 
						{
							tutorid = Some(args.1);
						},
						_ => {/*return Err(Error_Type::ERR_UNIMPLEMENTED)*/},
					};
				};
				let id = tutorid.ok_or_else(|| Error_Type::ERR_CANNOT_PARSE)?.parse::<u32>().or_else(|x| {println!("failed to parse {}", x); Err(Error_Type::ERR_CANNOT_PARSE)})?;
				let index = self.listtutor.iter().position(|x| x.return_id() == id).ok_or(Error_Type::ERR_NAME_NOT_FOUND)?;
				let tutor = self.listtutor.swap_remove(index);
				println!("successfully deleted");
				tutor.display_jadwal();
			},
			Operation::OPS_SHOW_TUTOR =>
			{
				for args in vecofargs.iter()
				{
					match args.0
					{
						"tutorid" => 
						{
							let id = args.1.parse::<u32>().or_else(|x| {println!("failed to parse {}", x); Err(Error_Type::ERR_CANNOT_PARSE)})?;
							let tutor = self.listtutor.iter().find(|&x| x.return_id() == id).ok_or(Error_Type::ERR_NAME_NOT_FOUND)?;
							tutor.display_jadwal();
						},
						"showall" =>
						{
							let iterator = self.listtutor.iter();
							for abds in iterator
							{
								abds.display_jadwal();
							};
						},
						_ => {/*return Err(Error_Type::ERR_CANNOT_PARSE)*/},
					};
				};
			},
			Operation::OPS_INIT_JADWAL =>
			{
				for args in vecofargs.iter()
				{
					let rentangjadwal : Option<&str> = None;
					match args.0
					{
						"rentangjadwal" =>
						{
							if self.lsjadwal.is_some() {println!("jadwal telah diinisialisasi"); return Err(Error_Type::ERR_INVALID);}
							let prsrentang = args.1.split_once(",").ok_or_else(|| {print!("Format: initjadwal");
								print!(":rentangjadwal=[hari]-[bulan]-[tahun] [jam]:[menit]:[detik],");
								println!("[hari]-[bulan]-[tahun] [jam]:[menit]:[detik]!"); Error_Type::ERR_CANNOT_PARSE})?;
							let listofjadwal = AlokasiJadwalSemester::alokasi_semester(prsrentang.0, prsrentang.1)?;
							self.lsjadwal = Some(listofjadwal);
							println!("sukses menginisialisasi rentang jadwal");
						},
						_ => {return Err(Error_Type::ERR_CANNOT_PARSE)}, 
					}
				}
			},
			Operation::OPS_ADD_RUANGAN =>
			{
				if self.lsjadwal.is_none() {println!("jadwal belum diinilisasi"); return Err(Error_Type::ERR_INVALID);}
				let mut namaruang: Option<&str> = None;
				for args in vecofargs.iter()
				{
					match args.0 
					{
						"namaruang" => namaruang = Some(args.1),				
						_ => {println!("argumen tidak dikenali"); return Err(Error_Type::ERR_INVALID);}
					};
					
				};
				if namaruang.is_none() {return Err(Error_Type::ERR_INVALID);}
				let ruangan = RuangKelas::new(namaruang.unwrap(), self.lsjadwal.clone().unwrap());
				self.lskelas.push(ruangan);
			},
			Operation::OPS_ADD_JADWAL =>
			{
				if self.lsjadwal.is_none() {println!("jadwal belum diinilisasi"); return Err(Error_Type::ERR_INVALID);}
				let mut idtutor : Option<&str> = None;
				let mut tanggal	: Option<&str> = None;
				let mut lamanya : Option<&str> = None;
				let mut ruangan : Option<&str> = None;
				let mut searchidtutor : Option<u32> = None;
				//let mut searchidruangan : Option<u64> = None;
				for args in vecofargs.iter()
				{
					match args.0 
					{
						"idtutor" => idtutor = Some(args.1),
						"tanggal" => tanggal = Some(args.1),
						"lamanya" => lamanya = Some(args.1),
						"ruangan" => ruangan = Some(args.1),
						_ => {println!("argument tidak dikenali"); return Err(Error_Type::ERR_INVALID);}
					};
				};
				/* search for tutor*/
				for args in self.listtutor.iter_mut()
				{
					if args.return_id() == idtutor.unwrap().parse::<u32>().or_else(|xa| Err(Error_Type::ERR_INVALID))?
					{
						searchidtutor = Some(args.return_id());
						break;
					}
				};
				/* search for ruangan */
				for args in self.lskelas.iter_mut()
				{
					if args.showid() == ruangan.unwrap().parse::<u32>().or_else(|xa| Err(Error_Type::ERR_INVALID))?
					{
						args.addjadwal(tanggal.unwrap(), lamanya.unwrap(), searchidtutor.unwrap())?;
						break;
					}
				};
			},
			Operation::OPS_SHOW_JADWAL =>
			{
				if self.lsjadwal.is_none() {println!("jadwal belum diinilisasi"); return Err(Error_Type::ERR_INVALID);}; 
				self.lsjadwal.clone().unwrap().show_alokasi();
			},
			Operation::OPS_SAVE_TO_FILE =>
			{
				let mut file : Option<&str> = None;
				for args in vecofargs.iter()
				{
					match args.0 
					{
						"file" => file = Some(args.1),
						_ => {println!("Format: \"save:file=<path>\""); return Err(Error_Type::ERR_CANNOT_PARSE);}
					}
				}
				if file.is_none() {println!("file tidak diisi!"); return Err(Error_Type::ERR_INVALID);};
				self.savetofile(file.unwrap());
			},
			Operation::OPS_LOAD_TO_FILE =>
			{
				let mut file : Option<&str> = None;
				for args in vecofargs.iter()
				{
					match args.0 
					{
						"file" => file = Some(args.1),
						_ => {println!("Format: \"load:file=<path>\""); return Err(Error_Type::ERR_CANNOT_PARSE);}
					}
				}
				if file.is_none() {println!("file tidak diisi!"); return Err(Error_Type::ERR_INVALID);};
				self.loadtofile(file.unwrap());				
			},	
			Operation::OPS_QUIT => {return Err(Error_Type::ERR_QUIT)},
			_ => {return Err(Error_Type::ERR_UNIMPLEMENTED)},
		};
		self.args.clear();
		self.op = Operation::OPS_NOOP;
		Ok(())
	}

	pub fn cleanup(&mut self)
	{
		self.args.clear();
		self.op = Operation::OPS_NOOP;		
	}
*/
	pub fn savetofile(&self, path : &str) -> Result<(), Error_Type>
	{
		match std::fs::write(path, serde_json::to_string_pretty(self).unwrap())
		{
			Ok(_) => return Ok(()),
			Err(_) => return Err(Error_Type::ERR_IO_ERROR),
		}
		Err(Error_Type::ERR_IO_ERROR)
	}
	pub fn loadtofile(&mut self, path : &str) -> Result<(), Error_Type>
	{
		let buffer : Database = {
			let json_buffer = fs::read_to_string(path).expect("cannot open file\n");
			serde_json::from_str::<Database>(&json_buffer).unwrap()
			};
		self.listtutor = buffer.listtutor;
		self.lsjadwal = buffer.lsjadwal;
		self.lskelas = buffer.lskelas;
		Ok(())
	}
}

fn getops(argmt: &str) -> Result<(Operation, String), Error_Type>
{
	let binding = argmt.clone().to_lowercase();
	let args = binding.trim().split_once(":").ok_or_else(|| Error_Type::ERR_CANNOT_PARSE)?;	
	let op = args.0.parse::<Operation>()?;
	Ok((op, args.1.to_string()))
}

fn deciders(database: &mut Database, args : &str) -> Result<(), Error_Type>
{	
	let mut vecofargs = Vec::<(&str , &str)>::new();
	let ops = getops(args)?;
	
	for als in ops.1.as_str().split("&")
	{
		let argspair : (&str , &str) = als.clone().split_once("=").unwrap_or_else(|| (als, ""));
		vecofargs.push(argspair);
	}
	match ops.0
	{
		Operation::OPS_ADD_TUTOR => 
		{
			let mut tutorname : Option<&str> = None;
			let mut matkul : Option<&str> = None;
			for args in vecofargs.iter()
			{
				match args.0
				{
					"nametutor" => 
					{
						tutorname = Some(args.1);
					},
					"matkul" => 
					{
						matkul = Some(args.1);
					},
					_ => {/*return Err(Error_Type::ERR_IO_ERROR)*/},
				};
			}
			let buftutor = Tutor::new(tutorname.ok_or_else(|| {println!("Error: nama tutor tidak ada"); Error_Type::ERR_CANNOT_PARSE})?,
			parse_vec_from_comma(matkul.ok_or_else(|| {println!("Error: nama matkul tidak ada"); Error_Type::ERR_CANNOT_PARSE})?))?;
			println!("Success, tutor id :{}", buftutor.return_id());
			database.listtutor.push(buftutor);
		},
		Operation::OPS_DEL_TUTOR =>
		{
			let mut tutorid : Option<&str> = None;
			for args in vecofargs.iter()
			{
				match args.0
				{
					"tutorid" => 
					{
						tutorid = Some(args.1);
					},
					_ => {/*return Err(Error_Type::ERR_UNIMPLEMENTED)*/},
				};
			};
			let id = tutorid.ok_or_else(|| Error_Type::ERR_CANNOT_PARSE)?.parse::<u32>().or_else(|x| {println!("failed to parse {}", x); Err(Error_Type::ERR_CANNOT_PARSE)})?;
			let index = database.listtutor.iter().position(|x| x.return_id() == id).ok_or(Error_Type::ERR_NAME_NOT_FOUND)?;
			let tutor = database.listtutor.swap_remove(index);
			println!("successfully deleted");
			tutor.display_jadwal();
		},
		Operation::OPS_SHOW_TUTOR =>
		{
			for args in vecofargs.iter()
			{
				match args.0
				{
					"tutorid" => 
					{
						let id = args.1.parse::<u32>().or_else(|x| {println!("failed to parse {}", x); Err(Error_Type::ERR_CANNOT_PARSE)})?;
						let tutor = database.listtutor.iter().find(|&x| x.return_id() == id).ok_or(Error_Type::ERR_NAME_NOT_FOUND)?;
						tutor.display_jadwal();
					},
					"showall" =>
					{
						let iterator = database.listtutor.iter();
						for abds in iterator
						{
							abds.display_jadwal();
						};
					},
					_ => {/*return Err(Error_Type::ERR_CANNOT_PARSE)*/},
				};
			};
		},
		Operation::OPS_INIT_JADWAL =>
		{
			for args in vecofargs.iter()
			{
				let rentangjadwal : Option<&str> = None;
				match args.0
				{
					"rentangjadwal" =>
					{
						if database.lsjadwal.is_some() {println!("jadwal telah diinisialisasi"); return Err(Error_Type::ERR_INVALID);}
						let prsrentang = args.1.split_once(",").ok_or_else(|| {print!("Format: initjadwal");
							print!(":rentangjadwal=[hari]-[bulan]-[tahun] [jam]:[menit]:[detik],");
							println!("[hari]-[bulan]-[tahun] [jam]:[menit]:[detik]!"); Error_Type::ERR_CANNOT_PARSE})?;
						let listofjadwal = AlokasiJadwalSemester::alokasi_semester(prsrentang.0, prsrentang.1)?;
						database.lsjadwal = Some(listofjadwal);
						println!("sukses menginisialisasi rentang jadwal");
					},
					_ => {return Err(Error_Type::ERR_CANNOT_PARSE)}, 
				}
			}
		},
		Operation::OPS_ADD_RUANGAN =>
		{
			if database.lsjadwal.is_none() {println!("jadwal belum diinilisasi"); return Err(Error_Type::ERR_INVALID);}
			let mut namaruang: Option<&str> = None;
			for args in vecofargs.iter()
			{
				match args.0 
				{
					"namaruang" => namaruang = Some(args.1),				
					_ => {println!("argumen tidak dikenali"); return Err(Error_Type::ERR_INVALID);}
				};
				
			};
			if namaruang.is_none() {return Err(Error_Type::ERR_INVALID);}
			let ruangan = RuangKelas::new(namaruang.unwrap(), database.lsjadwal.clone().unwrap());
			database.lskelas.push(ruangan);
		},
		Operation::OPS_ADD_JADWAL =>
		{
			if database.lsjadwal.is_none() {println!("jadwal belum diinilisasi"); return Err(Error_Type::ERR_INVALID);}
			let mut idtutor : Option<&str> = None;
			let mut tanggal	: Option<&str> = None;
			let mut lamanya : Option<&str> = None;
			let mut ruangan : Option<&str> = None;
			let mut searchidtutor : Option<u32> = None;
			//let mut searchidruangan : Option<u64> = None;
			for args in vecofargs.iter()
			{
				match args.0 
				{
					"idtutor" => idtutor = Some(args.1),
					"tanggal" => tanggal = Some(args.1),
					"lamanya" => lamanya = Some(args.1),
					"ruangan" => ruangan = Some(args.1),
					_ => {println!("argument tidak dikenali"); return Err(Error_Type::ERR_INVALID);}
				};
			};
			/* search for tutor*/
			for args in database.listtutor.iter_mut()
			{
				if args.return_id() == idtutor.unwrap().parse::<u32>().or_else(|xa| Err(Error_Type::ERR_INVALID))?
				{
					searchidtutor = Some(args.return_id());
					break;
				}
			};
			/* search for ruangan */
			for args in database.lskelas.iter_mut()
			{
				if args.showid() == ruangan.unwrap().parse::<u32>().or_else(|xa| Err(Error_Type::ERR_INVALID))?
				{
					args.addjadwal(tanggal.unwrap(), lamanya.unwrap(), searchidtutor.unwrap())?;
					break;
				}
			};
		},
		Operation::OPS_SHOW_JADWAL =>
		{
			if database.lsjadwal.is_none() {println!("jadwal belum diinilisasi"); return Err(Error_Type::ERR_INVALID);}; 
			database.lsjadwal.clone().unwrap().show_alokasi();
		},
		Operation::OPS_SAVE_TO_FILE =>
		{
			let mut file : Option<&str> = None;
			for args in vecofargs.iter()
			{
				match args.0 
				{
					"file" => file = Some(args.1),
					_ => {println!("Format: \"save:file=<path>\""); return Err(Error_Type::ERR_CANNOT_PARSE);}
				}
			}
			if file.is_none() {println!("file tidak diisi!"); return Err(Error_Type::ERR_INVALID);};
			database.savetofile(file.unwrap());
		},
		Operation::OPS_LOAD_TO_FILE =>
		{
			let mut file : Option<&str> = None;
			for args in vecofargs.iter()
			{
				match args.0 
				{
					"file" => file = Some(args.1),
					_ => {println!("Format: \"load:file=<path>\""); return Err(Error_Type::ERR_CANNOT_PARSE);}
				}
			}
			if file.is_none() {println!("file tidak diisi!"); return Err(Error_Type::ERR_INVALID);};
		//	database = Database::loadtofile(file.unwrap()).unwrap();	
			database.loadtofile(file.unwrap()).unwrap();	
		},	
		Operation::OPS_QUIT => {return Err(Error_Type::ERR_QUIT)},
		_ => {return Err(Error_Type::ERR_UNIMPLEMENTED)},
	};
	//database.args.clear();
	//database.op = Operation::OPS_NOOP;
	Ok(())
}

pub fn looping() -> Result<(), Error_Type>
{
	
	
}

