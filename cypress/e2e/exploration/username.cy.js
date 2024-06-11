import { SNZPOOL_ADDRESS } from "../constants";

suite(["@tier1"], "Username", () => {
  it("is consistent between the staking and accounts pages", () => {
    // staking page
    cy.visit(`/staking-ledgers?q-key=${SNZPOOL_ADDRESS}&epoch=1`);
    cy.wait(500);
    cy.aliasTableRows("Staking Ledger - Epoch 1", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 1);
    cy.aliasTableColumnValue(
      "Staking Ledger - Epoch 1",
      "Username",
      "username",
    );
    cy.get("@username").should("have.text", "SNZPool");

    // accounts listing
    cy.visit(`/addresses/accounts?q-public-key=${SNZPOOL_ADDRESS}`);
    cy.wait(500);
    cy.aliasTableRows("Accounts", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 1);
    cy.aliasTableColumnValue("Accounts", "Username", "username");
    cy.get("@username").should("have.text", "SNZPool");

    // account spotlight
    cy.visit(`/addresses/accounts/${SNZPOOL_ADDRESS}`);
    cy.wait(500);
    cy.get(".spotlight-meta").contains("SNZPool");
  });
});
