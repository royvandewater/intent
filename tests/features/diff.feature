Feature: Diffing the intent between two versions of a test file

  Scenario: A test added on this branch is shown in green
    Given the source on main:
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    And the source on this branch:
      """
      describe('Calculator', () => {
        it('adds', () => {})
        it('subtracts', () => {})
      })
      """
    When I diff the intent
    Then "subtracts" is marked as added
    And "Calculator" is shown as unchanged
    And "adds" is shown as unchanged

  Scenario: A test removed on this branch is shown in red
    Given the source on main:
      """
      describe('Calculator', () => {
        it('adds', () => {})
        it('subtracts', () => {})
      })
      """
    And the source on this branch:
      """
      describe('Calculator', () => {
        it('adds', () => {})
      })
      """
    When I diff the intent
    Then "subtracts" is marked as removed

  Scenario: Identical intent produces no diff
    Given the source on main:
      """
      it('adds', () => {})
      """
    And the source on this branch:
      """
      it('adds', () => {})
      """
    When I diff the intent
    Then the diff is empty
