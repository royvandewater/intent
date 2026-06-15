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
