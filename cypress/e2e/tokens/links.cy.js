import { MINU_SYMBOL, MINU_TOKEN_ADDRESS } from "../constants";

describe("token transactions", () => {
  it("links to the user commands page", () => {
    cy.visit("/tokens");
    cy.clickLinkInTable(1, "Transactions", "Tokens");
    cy.url().should("include", `/commands/user`);
    cy.url().should("include", `q-token=${MINU_TOKEN_ADDRESS}`);
    cy.url().should("include", `q-status=All`);
    cy.aliasTableRows(`User Commands (${MINU_SYMBOL})`, "table-rows");
    cy.get("@table-rows").should("have.length", 1);

    // 1 of 1
    cy.assertTableMetadataCorrect(`User Commands (${MINU_SYMBOL})`, 1, 0);
    cy.assertTableMetadataCorrect(`User Commands (${MINU_SYMBOL})`, 1, 1);
  });
});

describe("token holders", () => {
  it("links to the accounts page", () => {
    cy.visit("/tokens");
    cy.clickLinkInTable(1, "Holders", "Tokens");
    cy.url().should(
      "include",
      `/addresses/accounts?q-token=${MINU_TOKEN_ADDRESS}`,
    );
    cy.aliasTableRows("MINU Token Accounts", "table-rows");
    cy.get("@table-rows").should("have.length", 1);

    // 1 of 1
    cy.assertTableMetadataCorrect("MINU Token Accounts", 1, 0);
    cy.assertTableMetadataCorrect("MINU Token Accounts", 1, 1);
  });
});
