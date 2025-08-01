export default function findLeftMostFeature(currSubLine) {
  /**
   * Find the left most feature of a line.
   * @param {*} line : string - the current line we are looking at.
   * @returns null - if there was no feature on the line.
   * @returns object with the [start, end) position in the original line of the match and jsx of the parsed element.
   */
  const parsedOptions = parsers.map((parser) => parser.tryParse(currSubLine));
  let earliest = null;
  for (const option of parsedOptions) {
    if (option === null) {
      continue;
    }

    if (earliest === null || option.start < earliest.start) {
      earliest = option;
    }
  }
  return earliest;
}

/**
 * An abstract class representing a parsing interface for inline items.
 */
export class parser {
  constructor(regex) {
    this.regex = regex;
  }

  /**
   * Try to parse for the class's feature at the start of the given line; return null if the line could not be parsed.
   * @param {*} line - string : the current line to parse.
   * @returns null if the feature could not be parsed on the line.
   * @returns object with the [start, end) position in the original line of the match and jsx of the parsed element.
   */
  tryParse(line) {
    line; // just to turn off linter yelling
    return null;
  }
}

class link extends parser {
  constructor() {
    super(/\[([^\]+]+)\]\(([^)]+)\)/);
  }

  /**
   * For a given line, try to find a link at the start.
   * A link looks like [desc](url).
   */
  tryParse(line) {
    const attempt = line.match(this.regex);
    if (attempt === null) {
      return attempt;
    }

    const description = attempt[1];
    const url = attempt[2];

    return {
      jsx: (
        <a className="text-blue-400 underline" href={url} target="_blank">
          {description}
        </a>
      ),
      start: attempt.index,
      end: attempt.index + attempt[0].length,
    };
  }
}

class code extends parser {
  constructor() {
    super(/`([^`]+)`/);
  }

  /**
   * For a given line, try to find an inline code snippet at the start.
   * Inline code looks like `print()` this - two backtics with non-backtics inside.
   */
  tryParse(line) {
    const attempt = line.match(this.regex);

    if (attempt === null) {
      return attempt;
    }

    const contents = attempt[1];

    return {
      jsx: <code>{contents}</code>,
      start: attempt.index,
      end: attempt.index + attempt[0].length,
    };
  }
}

class bold extends parser {
  constructor() {
    // okay to use non-posix ?: since this is supported by most browsers as per mdm.
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Regular_expressions/Non-capturing_group
    super(/\*{2}((?:[^*]|\\\*)+)\*{2}/);
  }

  /**
   * For a given line, try to find a piece of bold text.
   * The contents should replaced all escaped asterisks with just normal ones.
   * Bold text looks like **this** where an asterisk can be escaped by a \ i.e. \*.
   */
  tryParse(line) {
    const attempt = line.match(this.regex);

    if (attempt === null) {
      return attempt;
    }

    const contents = attempt[1].replace("\\*", "*");
    return {
      jsx: <b>{contents}</b>,
      start: attempt.index,
      end: attempt.index + attempt[0].length,
    };
  }
}

export const parsers = [new link(), new code(), new bold()];
