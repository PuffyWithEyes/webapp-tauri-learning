#[cfg(test)]
mod tests {
	use super::*;
}



#[derive(Clone)]
/// Бинарное поле
pub struct BinField { // Бинарное поле
	/// Массив содержащий биты, все биты выравнены по правой границе, то есть сначала идут незначащие нули а потом сами данные
	pub data: Vec<u8>,
	/// Количество значащих битов, без учёта незначащих битов справа
	pub length: usize,
	/// Количество урезанных справа битов
	pub padding_length: usize,
}

// data_to_read - вектор из которого произвести чтение данных
// bit_offset - индекс бита в массиве с которого начать чтение
// read_length - сколько бит прочесть
// На выходе возвращается бинарное поле содержащее извлечённые данные 
pub fn read_bin_field(data_to_read: &Vec<u8>, bit_offset: usize, read_length: usize) -> BinField {
	const BYTE_LENGTH: usize = 8usize;
	let mut data_buffer = vec![0u8;read_length.div_ceil(BYTE_LENGTH)];

	let length = data_to_read.len();
	let padding_length = (bit_offset + read_length).saturating_sub(length*BYTE_LENGTH);

	let mut byte_index = bit_offset/BYTE_LENGTH;
	if byte_index < length {
		let bytes_to_read = ((bit_offset + read_length - 1usize)/BYTE_LENGTH).clamp(byte_index,length - 1usize);
		
		const bit_mask: u8 = 0xffu8;
		let old_byte_index = byte_index;

		let target_length = data_buffer.len()*BYTE_LENGTH - read_length; // Смещение относительно левой границы
		if bit_offset % BYTE_LENGTH < target_length { // Если смещение левой границы в исходнике меньше сдвигать все биты вправо >>
			let shift = target_length - bit_offset % BYTE_LENGTH;
			let back_shift = BYTE_LENGTH - shift;
			data_buffer[0] = (data_to_read[byte_index] & (bit_mask >> (bit_offset % BYTE_LENGTH))) >> shift;
			
			if 1usize < data_buffer.len() {
				data_buffer[1] = (data_to_read[byte_index] & (bit_mask >> ((bit_offset + read_length) % BYTE_LENGTH))) << back_shift;
				byte_index += 1;
				for byte in byte_index..=bytes_to_read {
					data_buffer[byte - old_byte_index] |= (data_to_read[byte] & (bit_mask << shift)) >> shift;
					if byte - old_byte_index + 1_usize < data_buffer.len() {
						data_buffer[byte - old_byte_index + 1usize] |= (data_to_read[byte] & (bit_mask >> back_shift)) << back_shift;
					}
				}
			}
		} else if bit_offset % BYTE_LENGTH > target_length { // <<
			let shift = bit_offset % BYTE_LENGTH - target_length;
			let back_shift = BYTE_LENGTH - shift;
			data_buffer[0] = (data_to_read[byte_index] & (bit_mask  >> (bit_offset % BYTE_LENGTH))) << shift;
			byte_index += 1usize;
			if byte_index < bytes_to_read {
				data_buffer[0] |= (data_to_read[byte_index] & (bit_mask << back_shift)) >> back_shift;
				for byte in byte_index..=bytes_to_read {
					if byte - old_byte_index < data_buffer.len() {
						data_buffer[byte - old_byte_index] |= (data_to_read[byte] & (bit_mask >> shift)) << shift;
						if byte + 1usize < bytes_to_read {
							data_buffer[byte - old_byte_index] |= (data_to_read[byte + 1usize] & (bit_mask << back_shift)) >> back_shift;
						}
					}
				}
			}

		} else { // -
			data_buffer[0] = data_to_read[byte_index] & (bit_mask >> bit_offset % BYTE_LENGTH);
			byte_index += 1usize;
			for byte in byte_index..=bytes_to_read {
				data_buffer[byte - old_byte_index] = data_to_read[byte];
			}
		}
	}

	BinField {
		data: data_buffer,
		length: read_length,
		padding_length: padding_length,
	}
}

// data_buffer - ссылка на массив в который будет производиться запись
// bit_offset - индекс бита с которого начать запись
// data_to_write - бинарное поле которое необходимо записать
pub fn write_bin_field_into_pos(data_buffer: &mut Vec<u8>, bit_offset: usize, data_to_write: BinField) -> () {
	let length = data_buffer.len();
	let byte_index = bit_offset/8usize;
	if byte_index < length {
		let bytes_to_write = (bit_offset + data_to_write.length - 1usize)/8usize;
		// let bytes_to_read = ((bit_offset + read_length - 1usize)/8usize).clamp(byte_index,length - 1usize);
		
		let source_left_padding = (data_to_write.data.len()*8usize - data_to_write.length) % 8usize;

		let target_left_padding = bit_offset % 8usize;

		if source_left_padding < target_left_padding { // >>
			let shift = target_left_padding - source_left_padding;
			data_buffer[byte_index] |= data_to_write.data[0] >> shift;
			if byte_index + 1usize < length {
				data_buffer[byte_index + 1usize] |= data_to_write.data[0] << (8usize - shift);
				for i in 1..data_to_write.data.len() {
					if byte_index + i < length {
						data_buffer[byte_index + i] |= data_to_write.data[i] >> shift;
						if byte_index + i + 1usize < length {
							data_buffer[byte_index + i + 1usize] |= data_to_write.data[i] << (8usize - shift);
						}
					} else { break };
				}
			}

		} else if source_left_padding > target_left_padding { // <<
			let shift = source_left_padding - target_left_padding;
			data_buffer[byte_index] |= data_to_write.data[0] << shift;
			if 1 < data_to_write.data.len() {               
				data_buffer[byte_index] |= data_to_write.data[1] >> (8usize - shift);
				for i in 1..data_to_write.data.len() {
					if byte_index + i < length {
						data_buffer[byte_index + i] |= data_to_write.data[i] << shift;
						if i + 1usize < data_to_write.data.len() {
							data_buffer[byte_index + i] |= data_to_write.data[i + 1usize] >> (8usize - shift);
						}
					} else { break };
				}
				if bytes_to_write < length {
					data_buffer[bytes_to_write] |= data_to_write.data[data_to_write.data.len() - 1] << shift;
				}
			}

		} else { // -           
			for i in 0..data_to_write.data.len() {
				if byte_index + i < length {
					data_buffer[byte_index + i] |= data_to_write.data[i];
				} else { break };
			}
		}
	}
}

// Отображает бинарное поле в виде набора нулей и единиц, незначащие нули не отображаются,
// усечение справа биты отображаются красным
impl std::fmt::Display for BinField {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let bit_mask = 0x80;
		for bit in (self.data.len()*8usize - self.length)..(self.data.len()*8usize) {
			if bit + self.padding_length < self.data.len()*8usize {
				write!(f,"{}",((bit_mask >> bit % 8usize) & self.data[bit/8usize]) >> (7usize - bit % 8usize))?;
			} else {    
				write!(f, "\x1b[31m{}", ((bit_mask >> bit % 8usize) & self.data[bit/8usize]) >> (7usize - bit % 8usize))?;
			}
		}
		write!(f, "\x1b[37m")
	}
}

// Два бинарных поля рвны только в том случае когда их значения абсолютно совпадают
impl std::cmp::PartialEq for BinField {
	fn eq(&self, other: &Self) -> bool {
		if self.length != other.length || self.padding_length != other.padding_length { return false };
		for i in 0..self.data.len() {
			if self.data[i] != other.data[i] { 
				return false 
			};
		}
		return true;
	}
}

impl BinField {
	pub fn push_bit(&mut self, bit: u8) -> () {
		if self.length  %  8usize > 0usize {
			self.data[0] |= bit << (self.length  %  8usize);
			self.length += 1usize;
		} else {
			self.data.insert(0usize,bit);
			self.length += 1usize;
		}
	}
}

