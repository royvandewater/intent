Feature: Running the intent CLI on a test file

  Scenario: Prints the intent of a test file
    Given a test file containing:
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    When I run intent on that file
    Then it exits successfully
    And it prints:
      """
      Calculator
        adds
      """

  Scenario: Printing usage information with --help
    When I run intent with "--help"
    Then it exits successfully
    And the output describes the usage of intent

  Scenario: Resolves a path relative to the git repo root
    Given a git repository containing "src/calc.test.ts":
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    When I run intent on "src/calc.test.ts" from a subdirectory of that repository
    Then it exits successfully
    And it prints:
      """
      Calculator
        adds
      """

  Scenario: Diffing only shows changes introduced by this branch
    Given a git repository whose main branch has "calc.test.ts":
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    And this branch changed "calc.test.ts" to:
      """
      describe('Calculator', () => {
        it('adds', () => {})
        it('subtracts', () => {})
      })
      """
    And main then changed "calc.test.ts" to:
      """
      describe('Calculator', () => {
        it('adds', () => {})
        it('multiplies', () => {})
      })
      """
    When I run intent with "--diff" in that repository
    Then it exits successfully
    And it prints:
      """
      calc.test.ts
        Calculator
          adds
      +   subtracts
      """
