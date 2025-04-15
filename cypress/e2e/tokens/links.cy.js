import { MINU_TOKEN_ADDRESS } from "../constants";

suite(["@tier2"], "token transactions", () => {
  it("links to the user commands page", () => {
    cy.visit("/tokens");
    cy.clickLinkInTable(1, "Transactions", "Tokens");
    cy.url().should("include", `/commands/user?q-token=${MINU_TOKEN_ADDRESS}`);
    cy.aliasTableRows("User Commands", "table-rows");
    cy.get("@table-rows").should("have.length", 1);

    // 1 of 1
    cy.assertTableMetadataCorrect("User Commands", 1, 0);
    cy.assertTableMetadataCorrect("User Commands", 1, 1);
  });
});

suite(["@tier2"], "token holders", () => {
  it("links to the accounts page", () => {
    cy.visit("/tokens");
    cy.clickLinkInTable(1, "Holders", "Tokens");
    cy.url().should(
      "include",
      `/addresses/accounts?q-token=${MINU_TOKEN_ADDRESS}`,
    );
    cy.aliasTableRows("MINU Token Accounts", "table-rows");
    cy.get("@table-rows").should("have.length", 1);
  });
});
