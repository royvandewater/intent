Feature: Restricting --diff to named files

  Scenario: No filters shows every changed path
    Given the changed path "tests/calculator.test.ts"
    And no filename filters
    Then the path is shown

  Scenario: A matching filter shows the path
    Given the changed path "tests/calculator.test.ts"
    And the filename filter "tests/calculator.test.ts"
    Then the path is shown

  Scenario: A non-matching filter hides the path
    Given the changed path "tests/calculator.test.ts"
    And the filename filter "tests/parser.test.ts"
    Then the path is hidden

  Scenario: A basename filter matches a nested path
    Given the changed path "tests/calculator.test.ts"
    And the filename filter "calculator.test.ts"
    Then the path is shown
