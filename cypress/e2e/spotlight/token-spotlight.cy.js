import {
  MINU_TOKEN_ADDRESS,
  MINU_USERNAME,
  STANDARD_ACCOUNT_PK,
} from "../constants";

suite(["@tier2"], "token holding", () => {
  it("is rendered", () => {
    cy.visit(
      `/addresses/accounts/${STANDARD_ACCOUNT_PK}/tokens/${MINU_TOKEN_ADDRESS}`,
    );
    cy.contains("Token Holding").should("exist");
    cy.contains(`Symbol: ${MINU_USERNAME}`).should("exist");

    cy.testSpotlightValue("Public Key", STANDARD_ACCOUNT_PK);
    cy.testSpotlightValue("Balance", "100,000,000,000,000");
    cy.testSpotlightValue("Nonce", "0");
    cy.testSpotlightValue("Delegate", STANDARD_ACCOUNT_PK);
    cy.testSpotlightValue("Zkapp URI", "None");
    cy.testSpotlightValue("Ver. Hash Key", "None");

    cy.aliasTransposedTableRows("Other Zkapp Details", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 2);
  });
});
