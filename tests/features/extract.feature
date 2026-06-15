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
