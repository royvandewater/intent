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

Output is colored when writing to a terminal — `describe` containers in bold
and `it`/`test` leaves in green, following Cucumber's scheme. Colors are
suppressed when the output is piped or when `NO_COLOR` is set.

## Diffing against `main`

```sh
intent --diff
```

Compares every test file that changed between `main` and `HEAD` (two-dot
`main..HEAD` — a straight tip-to-tip comparison, not against the merge base)
and prints how each file's intent changed: titles added on this branch in
green `+`, titles removed in red `-`, shared titles as plain context.

## Development

Built with strict TDD. The suite is written with
[cucumber-rs](https://github.com/cucumber-rs/cucumber): Gherkin scenarios live
in `tests/features/`, and their step definitions live in the matching
`tests/*.rs` runners. Run the suite with:

```sh
cargo test
```
