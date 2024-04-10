import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "account page", () => {
  it(`has all sections`, () => {
    cy.visit(`/addresses/accounts/${DEFAULT_ACCOUNT_PK}`);
    ["Transactions", "SNARK Jobs", "Block Production"].forEach((section) => {
      cy.get("section h1").contains(section, { timeout: 60000 });
    });
  });
});
