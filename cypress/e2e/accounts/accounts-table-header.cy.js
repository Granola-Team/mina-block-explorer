import { MINA_TOKEN_ID } from "../constants";

suite(["@tier2"], "accounts listing page", () => {
  it(`correctly identifies MINA token`, () => {
    cy.visit(`/addresses/accounts?q-token=${MINA_TOKEN_ID}`);
    cy.contains("MINA Token Accounts");
  });
});
