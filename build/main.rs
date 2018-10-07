/*!
Data generator.
*/

use std::{env, fmt, fs, u8};
use std::path::PathBuf;
use std::io::prelude::*;
use std::collections::HashSet;

extern crate csv;

#[path = "../src/schema.rs"]
mod schema;

fn process_data<F: FnMut(csv::StringRecord)>(f: &mut F) {
	// Process the geek1 dataset
	csv::ReaderBuilder::new()
		.delimiter(0x1F)
		.has_headers(true)
		.from_path("build/geek1.csv")
		.unwrap()
		.records()
		.map(Result::unwrap)
		.for_each(&mut *f);
	// Process the geek2 dataset
	csv::ReaderBuilder::new()
		.delimiter(0x1F)
		.has_headers(true)
		.from_path("build/geek2.csv")
		.unwrap()
		.records()
		.map(Result::unwrap)
		.for_each(&mut *f);
}

struct Record<'a> {
	pf: &'a str,
	of: &'a str,
	po: &'a str,
	so: &'a str,
	flds: &'a str,
	o: &'a str,
	proc_: &'a str,
	st: &'a str,
	m: &'a str,
	rl: &'a str,
	x: &'a str,
	mnemonic: &'a str,
	op1: &'a str,
	op2: &'a str,
	op3: &'a str,
	op4: &'a str,
	iext: &'a str,
	grp1: &'a str,
	grp2: &'a str,
	grp3: &'a str,
	tested_f: &'a str,
	modif_f: &'a str,
	def_f: &'a str,
	undef_f: &'a str,
	f_values: &'a str,
	description: &'a str,
}
impl<'a> Record<'a> {
	fn from(record: &'a csv::StringRecord) -> Record<'a> {
		Record {
			pf: record.get(0).unwrap(),
			of: record.get(1).unwrap(),
			po: record.get(2).unwrap(),
			so: record.get(3).unwrap(),
			flds: record.get(4).unwrap(),
			o: record.get(5).unwrap(),
			proc_: record.get(6).unwrap(),
			st: record.get(7).unwrap(),
			m: record.get(8).unwrap(),
			rl: record.get(9).unwrap(),
			x: record.get(10).unwrap(),
			mnemonic: record.get(11).unwrap(),
			op1: record.get(12).unwrap(),
			op2: record.get(13).unwrap(),
			op3: record.get(14).unwrap(),
			op4: record.get(15).unwrap(),
			iext: record.get(16).unwrap(),
			grp1: record.get(17).unwrap(),
			grp2: record.get(18).unwrap(),
			grp3: record.get(19).unwrap(),
			tested_f: record.get(20).unwrap(),
			modif_f: record.get(21).unwrap(),
			def_f: record.get(22).unwrap(),
			undef_f: record.get(23).unwrap(),
			f_values: record.get(24).unwrap(),
			description: record.get(25).unwrap(),
		}
	}
}

fn insert(groups: &mut HashSet<String>, name: Option<&str>) {
	let mut name = match name {
		Some(name) => name,
		None => return,
	};
	if name.len() == 0 {
		name = "Blank";
	}
	for name in name.split_whitespace() {
		groups.insert(name.to_owned());
	}
}
fn print<T: IntoIterator>(out: &mut Write, head: &str, data: T, tail: &str) where T::Item: fmt::Display {
	write!(out, "{}", head).unwrap();
	for item in data {
		writeln!(out, "\t{},", item).unwrap();
	}
	write!(out, "{}", tail).unwrap();
}

fn main() {
	let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

	let mut schema_file = fs::File::create(out_dir.join("schema.rs")).unwrap();
	let mut data_file = fs::File::create(out_dir.join("data.rs")).unwrap();

	let mut groups = HashSet::new();
	let mut iext = HashSet::new();

	writeln!(data_file, "pub static OPCODES_TABLE: [Opcode; 0] = [").unwrap();

	process_data(&mut |record| {
		// Read groups
		insert(&mut groups, record.get(17));
		insert(&mut groups, record.get(18));
		insert(&mut groups, record.get(19));
		// Read iext
		insert(&mut iext, record.get(16));

		// Write the instruction database
		let record = Record::from(&record);
		if record.po.len() > 0 && record.grp1.len() > 0 {
			let pf = u8::from_str_radix(record.pf, 16).unwrap_or(0);
			let of = u8::from_str_radix(record.of, 16).unwrap_or(0);
			let po = u8::from_str_radix(record.po, 16).unwrap();
			let so = u8::from_str_radix(record.so, 16).unwrap_or(0);
			writeln!(data_file, "\tOpcode /* {:02x}: {} */ {{", po, record.mnemonic).unwrap();
			writeln!(data_file, "\t\tbytes: OpcodeBytes {{ prefix: {:#04x}, of: {:#04x}, po: {:#04x}, so: {:#04x}, mask: 0b11111111, flags: OpcodeFlags(0b1_00) }},",
				pf, of, po, so,
			).unwrap();
			writeln!(data_file, "\t}},").unwrap();
		}
	});

	writeln!(data_file, "];").unwrap();

	print(&mut schema_file, "pub enum Group {\n", &groups, "}\n");
	print(&mut schema_file, "pub enum ExtGroup {\n", &iext, "}\n");
}
