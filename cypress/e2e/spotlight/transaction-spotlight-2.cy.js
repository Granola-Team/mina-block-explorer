import { FIRST_TXN_HASH } from "../constants";

suite(["@tier2"], `Command ${FIRST_TXN_HASH}`, () => {
  it("is canonical", () => {
    cy.visit(`/commands/user?q-txn-hash=${FIRST_TXN_HASH}`);
    cy.get(".loading-placeholder").should("not.exist");
    cy.clickLinkInTable(0, "Txn Hash", "User Commands");
    cy.aliasTransposedTableRows("Command Spotlight", "canonical");
    cy.get("@canonical").find("td").should("contain", "true");
  });
});
