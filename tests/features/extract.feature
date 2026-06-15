Feature: Extracting intent from a test file

  Scenario: Empty source produces empty output
    Given an empty source
    When I extract the intent
    Then the output is empty

  Scenario: A test block prints its title
    Given the source:
      """
      test('adds two numbers', () => {})
      """
    When I extract the intent
    Then the output is:
      """
      adds two numbers
      """

  Scenario: A single it block prints its title
    Given the source:
      """
      it('adds two numbers', () => {})
      """
    When I extract the intent
    Then the output is:
      """
      adds two numbers
      """

  Scenario: Multiple it blocks print one title per line
    Given the source:
      """
      it('first', () => {})
      it('second', () => {})
      """
    When I extract the intent
    Then the output is:
      """
      first
      second
      """

  Scenario: An it nested in a describe is indented under it
    Given the source:
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    When I extract the intent
    Then the output is:
      """
      Calculator
        adds
      """

  Scenario: Double-quoted titles are supported
    Given the source:
      """
      it("adds two numbers", () => {})
      """
    When I extract the intent
    Then the output is:
      """
      adds two numbers
      """

  Scenario: Backtick titles are supported
    Given the source:
      """
      it(`adds two numbers`, () => {})
      """
    When I extract the intent
    Then the output is:
      """
      adds two numbers
      """

  Scenario: Non-describe or it lines with quotes are ignored
    Given the source:
      """
      describe('Calculator', () => {
        it('adds', () => {
          expect(add(1, 2)).toBe('three')
        })
      })
      """
    When I extract the intent
    Then the output is:
      """
      Calculator
        adds
      """

  Scenario: Colored output styles describe blocks and tests
    Given the source:
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    When I extract the colored intent
    Then "Calculator" is shown as a describe block
    And "adds" is shown as a passing test

  Scenario: test blocks are styled like passing tests
    Given the source:
      """
      test('adds two numbers', () => {})
      """
    When I extract the colored intent
    Then "adds two numbers" is shown as a passing test
