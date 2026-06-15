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
