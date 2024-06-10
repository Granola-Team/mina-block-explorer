import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["tier1"], "Account spotlight", () => {
  let expected_fields = ["Balance", "Delegate"];

  it("displays complete information", () => {
    cy.visit(`/addresses/accounts/${DEFAULT_ACCOUNT_PK}`);
    cy.testSpotlight("Account Spotlight", DEFAULT_ACCOUNT_PK, expected_fields);
  });
});
