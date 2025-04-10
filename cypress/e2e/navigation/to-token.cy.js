import {
  ZK_APP_TXN_HASH,
  NFT_TOKEN_ID,
  TOKEN_ACTIVITY_ONLY_ADDRESS,
} from "../constants";

suite(["@tier2"], "tokens page", () => {
  // Reusable function to verify tokens page navigation
  const verifyTokensPage = () => {
    cy.url().should("contain", `/tokens?q-id=${NFT_TOKEN_ID}`);
    cy.get("th")
      .contains("ID")
      .find("input")
      .should("have.value", NFT_TOKEN_ID);
  };

  it("is navigated to from command spotlight page", () => {
    cy.visit(`/commands/${ZK_APP_TXN_HASH}`);
    cy.waitUntilTableLoads("Accounts Updated");
    cy.clickLinkInTable(5, "Token ID", "Accounts Updated");
    verifyTokensPage();
  });

  it("is navigated to from the account token holding page", () => {
    cy.visit(`/addresses/accounts/${TOKEN_ACTIVITY_ONLY_ADDRESS}/tokens`);
    cy.waitUntilTableLoads("Tokens");
    cy.clickLinkInTable(0, "Token ID", "Tokens");
    verifyTokensPage();
  });
});
