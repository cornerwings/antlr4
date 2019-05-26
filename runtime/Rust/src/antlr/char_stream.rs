use antlr::int_stream::IntStream;
use antlr::misc::interval::Interval;

pub trait CharStream: IntStream {

	fn text(&self, interval: Interval) -> str;

}

