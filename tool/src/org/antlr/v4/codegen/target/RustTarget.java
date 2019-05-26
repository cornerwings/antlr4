package org.antlr.v4.codegen.target;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

import org.antlr.v4.codegen.CodeGenerator;
import org.antlr.v4.codegen.Target;
import org.antlr.v4.codegen.UnicodeEscapes;
import org.antlr.v4.tool.ast.GrammarAST;

public class RustTarget extends Target {

	protected static final String[] rustKeywords = {
		"as", "break", "const", "continue", "crate", "else", "enum",
		"extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
		"match", "mod", "move", "mut", "pub", "ref", "return", "Self",
		"self", "static", "struct", "super", "trait", "true", "type", "unsafe",
		"use", "where", "while", "abstract", "alignof", "become", "box", "do",
		"final", "macro", "offsetof", "override", "priv", "proc", "pure",
		"sizeof", "typeof", "unsized", "virtual", "yield"
	};

	/** Avoid grammar symbols in this set to prevent conflicts in gen'd code. */
	protected final Set<String> badWords = new HashSet<String>();

	public RustTarget(CodeGenerator gen) {
		super(gen, "Rust");
	}

	public String getVersion() {
		return "4.7.1";
	}

	public boolean needsHeader() { return true; }

	public Set<String> getBadWords() {
		if (badWords.isEmpty()) {
			addBadWords();
		}

		return badWords;
	}

	protected void addBadWords() {
		badWords.addAll(Arrays.asList(rustKeywords));
		badWords.add("rule");
		badWords.add("parserRule");
	}

	@Override
	protected void appendUnicodeEscapedCodePoint(int codePoint, StringBuilder sb) {
		// Rust and Swift share the same escaping style.
		UnicodeEscapes.appendSwiftStyleEscapedCodePoint(codePoint, sb);

	}

	@Override
	protected boolean visibleGrammarSymbolCausesIssueInGeneratedCode(GrammarAST idNode) {
		return getBadWords().contains(idNode.getText());
	}

}
