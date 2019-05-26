pub trait IntStream {

	fn consume(&mut self);

	fn LA(&self, offset: isize) -> isize;

	fn mark(&mut self) -> isize;

	fn release(&mut self, marker: isize);

	fn index(&self) -> isize;

	fn seek(&mut self, index: isize);

	fn size(&self) -> isize;

	fn source_name(&self) -> &str;

}