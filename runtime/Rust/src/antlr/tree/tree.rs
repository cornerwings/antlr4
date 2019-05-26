use antlr::misc::interval::Interval;

pub trait Tree<T> {

	fn parent(&self) -> &Tree<T>;

	fn child(&self, i: isize) -> &Tree<T>;

	fn payload(&self) -> T;

	fn child_count(&self) -> isize;
}

pub trait SyntaxTree<T>: Tree<T> {

	fn source_interval(&self) -> Interval;

}