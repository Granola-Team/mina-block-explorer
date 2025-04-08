import { MINU_TOKEN_ADDRESS, MINU_USERNAME } from "../constants";

suite(["@tier2"], "token spotlight", () => {
  it("is rendered", () => {
    cy.visit(`/tokens/${MINU_TOKEN_ADDRESS}`);
    cy.contains("Token Overview").should("exist");
    cy.contains(`Symbol: ${MINU_USERNAME}`).should("exist");
  });
});
