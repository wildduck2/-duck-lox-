## Study Notes: Chapter 4 - A Tree-Walk Interpreter

This part of the book begins the first of two interpreters, **`jlox`**.

* **Language:** Java
* **Goal:** To build a complete interpreter for every feature in the Lox language in under 2,000 lines of simple, clean code.
* **Approach:**
    1.  First, we'll build the main phases front-to-back: **Scanning** (Chapter 4), **Parsing** (Chapter 5-6), and **Evaluating** (Chapter 7).
    2.  After that, we'll add the remaining language features one by one, growing our simple calculator into a full-featured scripting language.

---

## Study Notes: Chapter 4 - Scanning

This chapter builds the **scanner** (also called a **lexer** or **lexical analyzer**).

* **Scanner's Job:** To take the raw source code (a long string of characters) and group it into a series of **tokens** (the "words," "punctuation," and "values" of the language).

### 1. The `Lox.java` Framework

This is the main "shell" of our application.

* **Entrypoint:** The `main` method in the `Lox` class.
* **Two Modes:**
    1.  **File Mode:** `jlox [script.lox]`. This is handled by `runFile(path)`, which reads a file and passes its contents to the `run` method.
    2.  **Interactive Mode (REPL):** `jlox`. This is handled by `runPrompt()`, which gives the user a `>` prompt and executes code one line at a time. REPL stands for **Read, Evaluate, Print, Loop**.
* **Core Logic:** Both modes call `private static void run(String source)`. For now, this method just:
    1.  Creates a `Scanner`.
    2.  Calls `scanner.scanTokens()`.
    3.  Prints the resulting list of tokens.

### 2. Error Handling

* **Separation of Concerns:** Error *detection* (e.g., in the Scanner) is separate from error *reporting* (which is in the main `Lox` class).
* **`Lox.error(line, message)`:** A static helper that reports an error at a specific line.
* **`hadError` Flag:** A `static boolean` in the `Lox` class.
    * If any error is reported, `hadError` is set to `true`.
    * In file mode, if `hadError` is true after running, the application exits with code `65`.
    * In REPL mode, `hadError` is **reset** after each line so a single error doesn't kill the user's session.

### 3. Core Concepts: Lexemes vs. Tokens

* **Lexeme:** The actual raw string of characters from the source code. For example: `var`, `123`, `"`hello`"`, `(`.
* **Token:** The "packaged" object that represents a lexeme. It bundles the raw lexeme with other useful information.

The `Token` class has four fields:
1.  **`type` (`TokenType`):** An enum (e.g., `IDENTIFIER`, `NUMBER`, `STRING`, `LEFT_PAREN`) that categorizes the token.
2.  **`lexeme` (`String`):** The raw text of the lexeme.
3.  **`literal` (`Object`):** The *actual runtime value* of the lexeme. This is `null` for most tokens but holds the `double` for a `NUMBER` or the `String` for a `STRING`.
4.  **`line` (`int`):** The line number where the lexeme appeared, for error reporting.

The `TokenType` enum lists every possible kind of token: single-char punctuation (`LEFT_PAREN`), multi-char operators (`BANG_EQUAL`), literals (`NUMBER`), and keywords (`IF`, `WHILE`, etc.).

### 4. The `Scanner` Class

This class does the main work of scanning.

* **Key Fields:**
    * `source`: The full source code string.
    * `tokens`: A `List<Token>` that we add to as we scan.
    * `start`: An integer marking the *start* of the current lexeme being scanned.
    * `current`: An integer marking the *current character* we are looking at.
    * `line`: The current line number.
* **Main Method: `scanTokens()`**
    * This is the main loop. It runs `while (!isAtEnd())`.
    * Inside the loop, it first sets `start = current` to mark the beginning of a new lexeme.
    * It then calls `scanToken()` to scan and add one token.
    * After the loop, it adds one final `EOF` (End of File) token. This makes the parser simpler later.

### 5. `scanToken()`: The Heart of the Scanner

This is the "big switch statement" that consumes one lexeme and creates one token.

* **Core Helpers:**
    * `advance()`: Consumes the next character (increments `current`) and returns it.
    * `addToken(type)` / `addToken(type, literal)`: Creates a new `Token` using the text from `start` to `current` and adds it to the `tokens` list.
    * `match(expected)`: A "conditional `advance()`." If the *next* character is `expected`, it consumes it and returns `true`. Otherwise, it returns `false`.
    * `peek()`: **Lookahead**. Returns the *current* character (`source.charAt(current)`) **without** consuming it.
    * `peekNext()`: **Two-character lookahead**. Returns the character *after* the current one (`source.charAt(current + 1)`) **without** consuming anything.

* **Scanning Logic (inside `scanToken()`):**
    1.  **Single-Char Tokens:** For `(`, `)`, `{`, `}`, `,`, `.`, `-`, `+`, `;`, `*`, we just call `addToken()`.
    2.  **Multi-Char Operators:** For `!`, `=`, `<`, `>`, we use `match('=')` to check for a second character. This lets us distinguish `!` from `!=`, `==` from `=`, etc.
    3.  **Comments (`/`):** If the character is `/`, we use `match('/')`.
        * If `true` (a `//` comment), we loop and consume characters `while (peek() != '\n' && !isAtEnd())`. The comment is **discarded** (no `addToken()` call).
        * If `false`, it's just a `SLASH` token, so we `addToken(SLASH)`.
    4.  **Whitespace:** ` `, `\r`, `\t` are **ignored**. We just return, and the main `scanTokens()` loop starts over.
    5.  **Newlines:** `\n` is also ignored, but it **increments the `line` counter**.
    6.  **String Literals (`"`):** We loop (`while (peek() != '"')`) and `advance()` until we hit the closing `"`.
        * This supports multi-line strings (it increments `line` on newlines).
        * If we hit the end (`isAtEnd()`), we report an "Unterminated string" error.
        * We call `addToken(STRING, ...)` with the literal *value* (the string *without* the quotes).
    7.  **Number Literals (digits):**
        * We consume all digits (`while (isDigit(peek()))`).
        * We check for a fractional part using `peek()` and `peekNext()` (to see if it's `.` followed by another digit, e.g., `12.34`).
        * If it has a fractional part, we consume it.
        * We call `addToken(NUMBER, Double.parseDouble(...))` with the literal `double` value.
    8.  **Identifiers & Keywords:**
        * If a character is a letter or `_` (`isAlpha()`), we consume all letters, digits, or `_` (`isAlphaNumeric()`).
        * This follows the **Maximal Munch** rule: the scanner consumes the *longest possible* sequence. This is why `orchid` is scanned as one `IDENTIFIER`, not an `OR` keyword.
        * We get the lexeme's text and check it against a `static HashMap<String, TokenType>` of **keywords**.
        * If it's in the map, we `addToken(KEYWORD_TYPE)` (e.g., `AND`, `IF`, `PRINT`).
        * If not, we `addToken(IDENTIFIER)`.
    9.  **Errors:** If `default` is hit (unrecognized character), we call `Lox.error()`.
