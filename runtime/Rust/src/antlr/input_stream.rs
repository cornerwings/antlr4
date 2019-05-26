use antlr::int_stream::IntStream;
use antlr::char_stream::CharStream;
use std::cmp;

const TOKEN_EOF: isize = -1;

struct InputStream {
	data: Vec<char>,
	name: String,
	index: isize,
	size: isize,
}

impl IntStream for InputStream {

	fn consume(&mut self) {
		if (self.index >= self.size) {
			// assert is.LA(1) == TokenEOF
			panic!("cannot consume EOF")
		}
		self.index += 1;
	}

	fn LA(&self, offset: isize) -> isize {
		if (offset == 0) {
			return 0; // nil
		}

		let mut calc_offset = offset;
		if (calc_offset < 0) {
			calc_offset += 1; // e.g., translate LA(-1) to use offset=0
		}

		let pos = self.index + calc_offset - 1;
		if (pos < 0 || pos >= self.size) {
			return TOKEN_EOF;
		}

		return self.data[pos as usize] as isize;
	}

	fn mark(&mut self) -> isize {
		return -1;
	}

	fn release(&mut self, marker: isize) {

	}

	fn index(&self) -> isize {
		return self.index;
	}

	fn seek(&mut self, index: isize) {
		if (index <= self.index) {
			self.index = index;
			return;
		}

		self.index = cmp::min(index, self.size);
	}

	fn size(&self) -> isize {
		return self.size;
	}

	fn source_name(&self) -> &str {
		return self.name.as_str();
	}

}