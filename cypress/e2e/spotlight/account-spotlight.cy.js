import {
  GENESIS_ACCOUNT_PK,
  TOKEN_ACTIVITY_ONLY_ADDRESS,
  GENESIS_ACCOUNT_PK_ZERO_GENESIS,
  STANDARD_ACCOUNT_PK,
} from "../constants.js";
describe("account spotlight", () => {
  it("displays appropriately for standard accounts", () => {
    cy.visit("/addresses/accounts/" + STANDARD_ACCOUNT_PK);
    cy.contains("Includes 1 MINA account creation fee").should("exist");
  });
  it("displays appropriately for genesis accounst with zero balance", () => {
    cy.visit("/addresses/accounts/" + GENESIS_ACCOUNT_PK_ZERO_GENESIS);
    cy.contains("Includes 1 MINA account creation fee").should("not.exist");
    cy.contains("Includes balance from genesis ledger").should("not.exist");
    cy.contains(
      "This account only has custom tokens and no MINA balance.",
    ).should("not.exist");
  });
  it("displays appropriately for genesis accounts with positive balances", () => {
    cy.visit("/addresses/accounts/" + GENESIS_ACCOUNT_PK);
    cy.contains("Includes balance from genesis ledger").should("exist");
    cy.testSpotlightValue("Genesis Balance", "108,536.109082914MINA");
  });
  it("displays appropriately for token-only accounts", () => {
    cy.visit("/addresses/accounts/" + TOKEN_ACTIVITY_ONLY_ADDRESS);
    cy.contains(
      "This account only has custom tokens and no MINA balance.",
    ).should("exist");
    cy.aliasTableRows("User Commands", "table-rows");
    cy.get("@table-rows").should("have.length.gte", 0);
    cy.assertForEachColumnValue("User Commands", "Type", (text) => {
      expect(text).equal("Zkapp");
    });
  });
  // TODO: uncomment when https://github.com/Granola-Team/mina-indexer/issues/1860 closed and
  // indexer version is updated locally in project
  xit("renders More Details subpage on tokens tab", () => {
    cy.visit(`/addresses/accounts/${TOKEN_ACTIVITY_ONLY_ADDRESS}/tokens`);
    cy.waitUntilTableLoads("Tokens");
    cy.contains("More Details").should("not.exist");
    cy.clickLinkInTable(0, "More", "Tokens");
    cy.contains("More Details").should("exist");
    cy.aliasTransposedTableRows("More Details", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 2);
  });
});
