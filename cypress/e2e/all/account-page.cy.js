import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@tier1"], "account page", () => {
  it(`has all sections`, () => {
    cy.visit(`/addresses/accounts/${DEFAULT_ACCOUNT_PK}`);
    ["User Commands", "SNARK Jobs", "Block Production"].forEach((section) => {
      cy.get("section h1").contains(section);
    });
  });
});
