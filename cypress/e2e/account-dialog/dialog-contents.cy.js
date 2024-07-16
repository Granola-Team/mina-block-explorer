import { DEFAULT_ACCOUNT_PK } from "../constants";

// TODO: relegated to tier2 because fails in tier1 for unknown reasons
suite(["@tier2"], "dialog", () => {
  beforeEach(() => {
    cy.visit(`/blocks/accounts/${DEFAULT_ACCOUNT_PK}`);
  });

  it(`has correct sections`, () => {
    ["User Commands", "SNARK Jobs", "Block Production"].forEach((section) => {
      cy.get("section h2").contains(section, {
        timeout: 60000,
      });
    });
  });
});
