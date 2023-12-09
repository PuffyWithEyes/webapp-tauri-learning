#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use bin_read::*;



struct FrequencyOfField {
	field: BinField,
	frequency: usize,
}

impl FrequencyOfField {
	fn new(field: BinField) -> Self {
		Self {
			frequency: 1usize,
			field: field,
		}
	}
}

fn file_vec_into_bin_vec(file: &Vec<u8>, bin_length: usize) -> Vec<BinField> {
	let mut bin_vec : Vec<BinField> = Vec::with_capacity((file.len()*8usize).div_ceil(bin_length)); 
	for i in 0..bin_vec.capacity() {
		bin_vec.push(read_bin_field(file, i*bin_length,bin_length));
	}
	bin_vec
}

fn create_frequency_table (data: &Vec<BinField>) -> Vec<FrequencyOfField> {
	let mut frequency_table: Vec<FrequencyOfField> = Vec::new();
	for value in data {
		let mut should_be_added_to_dictionary = true;
		for cell in &mut frequency_table {
			if *value == (*cell).field {
				should_be_added_to_dictionary = false;
				cell.frequency += 1usize;
				break;
			}
		}
		
		if should_be_added_to_dictionary {
			frequency_table.push(FrequencyOfField::new((*value).clone()));
		}
	}

	return frequency_table;
}

fn merge_indexes_between_vec(vec: &mut Vec<(usize,Vec<usize>,BinField)>, x: usize, y: usize) -> () {
	let mut b = vec[y].1.clone();
	vec[x].1.append(&mut b);
}

fn create_huffman_code (frequency_table: &Vec<FrequencyOfField>) -> Vec<(usize,Vec<usize>,BinField)> {
	let mut frequency_seq = vec![(0usize,Vec::new(),BinField{padding_length: 0usize,length: 0usize,data: Vec::new()});frequency_table.len()];
	let mut sum = 0usize;
	for i in 0..frequency_table.len() {
		frequency_seq[i].0 = frequency_table[i].frequency;
		frequency_seq[i].1.push(i);
		sum += frequency_seq[i].0;
	}
	if frequency_table.len() - 1 == 0 {
		frequency_seq[0].2.push_bit(1);
		// println!("{} ", frequency_seq[0]);
	}
	for _ in 0..(frequency_table.len() - 1) {
		let mut min1 = (sum,0usize);
		let mut min2 = (sum,0usize);
		for n in 0..frequency_table.len() {
			if min2.0 >= frequency_seq[n].0 {
				if min1.0 >= frequency_seq[n].0 {
					min2.0 = min1.0;
					min2.1 = min1.1;
					min1.0 = frequency_seq[n].0;
					min1.1 = n;
				} else {
					min2.0 = frequency_seq[n].0;
					min2.1 = n;
				}
			}
		}
		frequency_seq[min1.1].0 = sum;
		frequency_seq[min2.1].0 = min1.0 + min2.0;
		for n in 0..frequency_seq[min1.1].1.len() {
			let index = frequency_seq[min1.1].1[n];
			frequency_seq[index].2.push_bit(0u8);
		}
		frequency_seq[min1.1].1.shrink_to(0usize);
		for n in 0..frequency_seq[min2.1].1.len() {
			let index = frequency_seq[min2.1].1[n];
			frequency_seq[index].2.push_bit(1u8);
		}
		merge_indexes_between_vec(&mut frequency_seq,min2.1,min1.1);
	}
	frequency_seq
}

// fn get_pure_shannon_entropy(frequency_table: &Vec<FrequencyOfField>) -> f64 {
// 	let mut i = 0.0f64;
// 	let mut sum = 0usize;
// 	for index in 0..frequency_table.len() {
// 		sum += frequency_table[index].frequency;
// 	}
// 	for index in 0..frequency_table.len() {
// 		i += (frequency_table[index].frequency as f64)*f64::log2((frequency_table[index].frequency as f64)/(sum as f64));
// 	}
// 	-i/(sum as f64)
// }

fn get_shannon_entropy(frequency_table: &Vec<FrequencyOfField>) -> f64 {
	let mut i = 0.0f64;
	let mut sum = 0usize;
	for index in 0..frequency_table.len() {
		sum += frequency_table[index].frequency;
	}
	for index in 0..frequency_table.len() {
		i += (frequency_table[index].frequency as f64)*f64::log2((frequency_table[index].frequency as f64)/(sum as f64));
	}
	-i/(sum as f64)
}

fn get_b_entropy(frequency_table: &Vec<FrequencyOfField>) -> f64 {
	let mut b = 0.0f64;
	let mut sum = 0usize;
	for i in 0..frequency_table.len() {
		sum += frequency_table[i].frequency;
	}
	for i in 0..frequency_table.len() {
		let mut some_coefficient = 0.0f64;
		for j in 0..frequency_table.len() {
			some_coefficient += (1.0f64 - ((frequency_table[j].frequency as f64 - frequency_table[i].frequency as f64)/(sum as f64)).abs())*(frequency_table[j].frequency as f64)/(sum as f64);
		}
		b += (frequency_table[i].frequency as f64)*some_coefficient.log2();
	}
	-b/(sum as f64)
}


fn get_pure_compression_coeffiсient(huffman_table: &Vec<(usize, Vec<usize>, BinField)>,frequency_table: &Vec<FrequencyOfField>, size: usize) -> f64 {
	let mut compressed_size = 0usize;
	for i in 0..huffman_table.len() {
		compressed_size += huffman_table[i].2.length*frequency_table[i].frequency;
	}
	(size as f64)/(compressed_size as f64)
}

fn get_compression_coeffiсient(huffman_table: &Vec<(usize,Vec<usize>,BinField)>,frequency_table: &Vec<FrequencyOfField>, size: usize) -> f64 {
	(size as f64)/(get_output_size(huffman_table,frequency_table) as f64)
}

fn _get_max_size_of_bin_field(huffman_table: &Vec<(usize,Vec<usize>,BinField)>) -> usize {
	let mut max_length = 0usize;
	for i in 0..huffman_table.len() {
		if max_length < huffman_table[i].2.length {
			max_length = huffman_table[i].2.length;
		}
	}
	max_length
}

fn u16_into_bin_field(x: u16) -> BinField {
	let mut bin_field = BinField {
		padding_length: 0usize,
		length: 0usize,
		data: vec![0u8;2usize],
	};
	bin_field.data[0] = (x >> 8usize) as u8;
	bin_field.data[1] = x as u8;
	bin_field
}

fn get_output_size(huffman_table: &Vec<(usize,Vec<usize>,BinField)>,frequency_table: &Vec<FrequencyOfField>) -> usize {
	let mut compressed_size = 0usize;
	for i in 0..huffman_table.len() {
		compressed_size += huffman_table[i].2.length*(frequency_table[i].frequency + 1usize);
		// if frequency_table[0].field.length == 72 {
		// 	println!("длина {}", huffman_table[i].2.length);
		// 	for elem in frequency_table {
		// 		println!("{}", elem.field);
		// 	}
		// }
	}
	compressed_size = compressed_size + 16usize + frequency_table.len()*(frequency_table[0].field.length + 16usize);
	compressed_size
}

fn encode_file_with_given_length(file: &Vec<u8>,length: usize) -> Vec<u8> {
	let seq_of_bin_fields = file_vec_into_bin_vec(&file,length);
	let frequency_table = create_frequency_table(&seq_of_bin_fields);
	let huffman_table = create_huffman_code(&frequency_table);
	let compressed_size = get_output_size(&huffman_table,&frequency_table);
	// if length == 72 {
	// 	println!("{},{},{}",frequency_table.len(),huffman_table.len(),compressed_size);
	// }
	let mut output_file = vec![0u8;compressed_size.div_ceil(8usize)];
	output_file[0] = (frequency_table[0].field.length >> 8usize) as u8;
	output_file[1] = frequency_table[0].field.length as u8;
	let mut bit_index = 16usize;
	for i in 0..frequency_table.len() {
		write_bin_field_into_pos(&mut output_file,bit_index,frequency_table[i].field.clone());
		bit_index += frequency_table[0].field.length;
		write_bin_field_into_pos(
			&mut output_file,
			bit_index,
			u16_into_bin_field(huffman_table[i].2.length as u16)
			);
		bit_index += 16usize;
		write_bin_field_into_pos(
			&mut output_file,
			bit_index,
			huffman_table[i].2.clone()
		);
		bit_index += huffman_table[i].2.length;
	}
	for i in 0..seq_of_bin_fields.len() {
		write_bin_field_into_pos(
			&mut output_file,
			bit_index,
			huffman_table[get_index_of_bin_field_in_frequency_table(&frequency_table,&seq_of_bin_fields[i])].2.clone()
		);
		bit_index += huffman_table[get_index_of_bin_field_in_frequency_table(&frequency_table,&seq_of_bin_fields[i])].2.length;
	}
	output_file
}

// Коэффициент сжатия со словарём
// Коэффициент сжатия без словаря
// б энтропия без словаря
// Шенноновская энтропия
fn get_efficency_coefficients(file: &Vec<u8>,length: usize) -> (f64,f64,f64,f64) {
	let seq_of_bin_fields = file_vec_into_bin_vec(&file,length);
	let frequency_table = create_frequency_table(&seq_of_bin_fields);
	let huffman_table = create_huffman_code(&frequency_table);
	if length == 72 {
		println!("{}", file.len());
	}
	(
		get_compression_coeffiсient(&huffman_table,&frequency_table,file.len()*8usize),
		get_pure_compression_coeffiсient(&huffman_table,&frequency_table,file.len()*8usize),
		get_b_entropy(&frequency_table),
		get_shannon_entropy(&frequency_table)

	)
}

fn get_index_of_bin_field_in_frequency_table(frequency_table: &Vec<FrequencyOfField>, field: &BinField) -> usize {
	for i in 0..frequency_table.len() {
		if *field == frequency_table[i].field {
			return i;
		}
	}
	panic!("По какой-то причине поле не было найдено");
}


#[tauri::command]
fn choose_file() -> String {
    use tauri_api::dialog::{
        select,
        Response,
    };
    
    return match select(None::<&str>, Some(std::path::Path::new("~"))).unwrap() {
        Response::Okay(filename) => filename,
        _ => String::from("None"),
    }
}


#[tauri::command]
fn choose_dir() -> String {
    use tauri_api::dialog::{
        pick_folder,
        Response,
    };

    return match pick_folder(Some(std::path::Path::new("~"))).unwrap() {
        Response::Okay(filename) => filename,
        _ => String::from("None"),
    }
}


#[derive(serde::Serialize)]
struct Coords {
    compression_dict: f64,
    compression: f64,
    b_entropy: f64,
    shen_entropy: f64,
}


impl Coords {
    fn new(
        _compression_dict: f64,
        _compression: f64,
        _b_entropy: f64,
        _shen_entropy: f64,
    ) -> Self {
        Self {
            compression_dict: _compression_dict,
            compression: _compression,
            b_entropy: _b_entropy,
            shen_entropy: _shen_entropy,
        }
    }
}


#[tauri::command(rename_all = "snake_case")]
fn calc(
	file_path: &str,
	dir_path: &str,
	length: usize,
	full_length: usize
) -> Coords {
    let file = fs::read(file_path).unwrap();
    let data = encode_file_with_given_length(&file, length);
	
	if length == full_length {
		let mut output_path = String::from(dir_path);
		output_path.push_str("/output.txt");
		fs::write(output_path, &data).unwrap();
	}
    
    let coords = get_efficency_coefficients(&file, length);
	if length == full_length {
		println!("{:?}", coords);
	}
    let coords = Coords::new(coords.0, coords.1, coords.2, coords.3);
	
    coords
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
	tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![choose_file, choose_dir, calc])
        .run(tauri::generate_context!())?;

    Ok(())
}

