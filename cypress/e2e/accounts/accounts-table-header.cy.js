import { MINA_TOKEN_ID } from "../constants";
describe("accounts listing page", () => {
  it(`correctly identifies MINA token`, () => {
    cy.visit(`/addresses/accounts?q-token=${MINA_TOKEN_ID}`);
    cy.contains("MINA Token Accounts");
  });
});
