import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "table", () => {
  let pages = [
    {
      url: "/blocks",
      section: "Blocks",
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      section: "Block Production",
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      section: "SNARK Jobs",
    },
  ];

  pages.forEach(({ url, section }) =>
    it(`on ${url} includes canonical blocks according to selection`, () => {
      cy.visit(url);
      cy.get("select#canonical-selection").as("menu");
      cy.url().should("not.include", "canonical");

      // should load canonical by default
      cy.contains("section", section)
        .find("table tr:not(:has(th)) span")
        .as("tableRows");
      cy.get("@tableRows").should("not.have.class", "bg-status-failed");
      cy.get("@tableRows").should("have.class", "bg-status-success");

      cy.wait(500);
      cy.get("@menu").select("Non-Canonical");
      cy.wait(500);
      cy.url().should("include", "canonical=false");
      cy.get("@tableRows").should("not.have.class", "bg-status-success");
      cy.get("@tableRows").should("have.class", "bg-status-failed");

      cy.get("@menu").select("Canonical");
      cy.url().should("include", "canonical=true");
      cy.wait(500);
      cy.contains("section", section)
        .find("table tr:not(:has(th)) span")
        .as("tableRows");
      cy.get("@tableRows").should("not.have.class", "bg-status-failed");
      cy.get("@tableRows").should("have.class", "bg-status-success");
    }),
  );
});
