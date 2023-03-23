use serde_derive::{Deserialize, Serialize};
use std::{string::ToString, fmt, cmp};

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub enum Error_Type {ERR_SUCCESS, 
					ERR_INVALID,
					ERR_NAME_NOT_FOUND, 
					ERR_NAME_IS_EMPTY, 
					ERR_SESSION_ID_NOT_FOUND, 
					ERR_TIME_FORMAT_ERROR, 
					ERR_ADDED_TIME_NOT_REASONABLE,
					ERR_WAKTU_SEMESTER_NEGATIVE,
					ERR_AWAL_SESI_TERLALU_JAUH,
					ERR_LAMA_SESI_TERLALU_LAMA,
					ERR_SESI_TERLALU_PANJANG,
					ERR_TIDAK_DAPAT_ALOKASI_WAKTU,
					ERR_JADWAL_TUMPANG_TINDIH,	
					ERR_CANNOT_PARSE,
					ERR_UNIMPLEMENTED,
					ERR_IO_ERROR,
					ERR_QUIT,	
					}


impl ToString for Error_Type
{
	fn to_string(&self) -> String
	{
		let erx = 
		match self
		{
			
			Error_Type::ERR_SUCCESS => "Success",
			Error_Type::ERR_NAME_IS_EMPTY => "Name is Empty",
			Error_Type::ERR_ADDED_TIME_NOT_REASONABLE => "Added Time is not reasonable",
			Error_Type::ERR_NAME_NOT_FOUND => "Name is not found",
			Error_Type::ERR_SESSION_ID_NOT_FOUND => "Session is not found",
			Error_Type::ERR_TIME_FORMAT_ERROR => "Time format error",
			Error_Type::ERR_WAKTU_SEMESTER_NEGATIVE => "Waktu Semester negatif",
			Error_Type::ERR_AWAL_SESI_TERLALU_JAUH => "awal sesi terlalu jauh",
			Error_Type::ERR_LAMA_SESI_TERLALU_LAMA => "waktu sesi terlalu lama",
			Error_Type::ERR_SESI_TERLALU_PANJANG => "Sesi terlalu panjang",
			Error_Type::ERR_TIDAK_DAPAT_ALOKASI_WAKTU => "tidak dapat mengalokasi waktu",
			Error_Type::ERR_JADWAL_TUMPANG_TINDIH => "Jadwal tumpang tindih",
			Error_Type::ERR_CANNOT_PARSE => "Tidak dapat memproses text",
			Error_Type::ERR_UNIMPLEMENTED => "Tidak diimplementasikan, mohon bersabar",
			Error_Type::ERR_IO_ERROR => "input output error",
			Error_Type::ERR_QUIT => "Keluar",
			Error_Type::ERR_INVALID => "Kesalahan tidak diketahui",
			_ => panic!(),
		};
		erx.to_string()	
	}
}

impl fmt::Debug for Error_Type
{
	
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", self.to_string())
	}
}
/*
impl cmp::PartialEq for Error_Type
{
	fn eq(&self, other : &Self) -> bool
	{
		Error_Type == other
	}
}
*/
