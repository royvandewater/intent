# intent

Parses a vitest test file and prints the contents of all the `describe` & `it`
blocks — like a Cucumber feature — so you can read the intent of every test in a
file without the implementation.

## Usage

```sh
intent path/to/file.test.ts
```

Given:

```ts
describe('Calculator', () => {
  it('adds two numbers', () => {
    expect(add(1, 2)).toBe(3)
  })

  describe('division', () => {
    it.skip('throws on divide by zero', () => {
      expect(() => div(1, 0)).toThrow()
    })
  })
})
```

it prints:

```
Calculator
  adds two numbers
  division
    throws on divide by zero
```

Recognizes `describe`, `it`, and `test` blocks (including modifiers like
`.skip` / `.only`), with single-quoted, double-quoted, and backtick titles.
Nesting is reflected by indentation.

## Development

Built with strict TDD. The suite is written with
[cucumber-rs](https://github.com/cucumber-rs/cucumber): Gherkin scenarios live
in `tests/features/`, and their step definitions live in the matching
`tests/*.rs` runners. Run the suite with:

```sh
cargo test
```
