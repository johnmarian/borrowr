Feature: Main menu

  Scenario: App launches and exits
    Given the app is running
    Then the main menu is displayed
    When the user selects Quit
    Then the app exits successfully
