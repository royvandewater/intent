Feature: Extracting intent from a test file

  Scenario: Empty source produces empty output
    Given an empty source
    When I extract the intent
    Then the output is empty
