pub trait Token {

	/**
	 * Get the text of the token.
	 */
	fn text(&self) -> str;

	/** Get the token type of the token */
	fn token_type(&self) -> isize;

	/** The line number on which the 1st character of this token was matched,
	 *  line=1..n
	 */
	fn line(&self) -> isize;

	/** The index of the first character of this token relative to the
	 *  beginning of the line at which it occurs, 0..n-1
	 */
	fn char_position_in_line(&self) -> isize;

	/** Return the channel this token. Each token can arrive at the parser
	 *  on a different channel, but the parser only "tunes" to a single channel.
	 *  The parser ignores everything not on DEFAULT_CHANNEL.
	 */
	fn channel(&self) -> isize;

	/** An index from 0..n-1 of the token object in the input stream.
	 *  This must be valid in order to print token streams and
	 *  use TokenRewriteStream.
	 *
	 *  Return -1 to indicate that this token was conjured up since
	 *  it doesn't have a valid index.
	 */
	fn token_index(&self) -> isize;

	/** The starting character index of the token
	 *  This method is optional; return -1 if not implemented.
	 */
	fn start_index(&self) -> isize;

	/** The last character index of the token.
	 *  This method is optional; return -1 if not implemented.
	 */
	fn stop_index(&self) -> isize;

}
