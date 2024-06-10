import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["tier1"], "account transaction table", () => {
  it("correctly references a counterparty", () => {
    cy.visit(`/addresses/accounts/${DEFAULT_ACCOUNT_PK}`);
    cy.get(".loading-placeholder").should("exist");
    cy.get(".loading-placeholder").should("not.exist");
    cy.aliasTableValue(0, "Counterparty", "User Commands", "counterparty");

    for (let i = 0; i <= 9; i++) {
      cy.aliasTableValue(i, "Counterparty", "User Commands", "counterparty");
      cy.get("@counterparty")
        .invoke("text")
        .then((text) => {
          if (text == "Self") {
            expect(true).to.equal(true);
          } else {
            expect(text.length).to.equal(DEFAULT_ACCOUNT_PK.length);
          }
          expect(text).to.not.equal(DEFAULT_ACCOUNT_PK);
        });
    }
  });
});
