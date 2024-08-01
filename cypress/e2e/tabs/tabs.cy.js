import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@tier2"], "tab", () => {
  let tabs = [
    {
      page: "/commands/internal",
      tab: "User Commands",
      expectedUrl: "/commands/user",
    },
    {
      page: "/commands/user",
      tab: "Internal Commands",
      expectedUrl: "/commands/internal",
    },
    {
      page: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/snark-jobs`,
      tab: "User Commands",
      expectedUrl: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/commands/user`,
    },
    {
      page: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      tab: "SNARK Jobs",
      expectedUrl: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/snark-jobs`,
    },
    {
      page: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      tab: "Block Production",
      expectedUrl: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/block-production`,
    },
    {
      page: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      tab: "Internal Commands",
      expectedUrl: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/commands/internal`,
    },
  ];

  tabs.forEach(({ page, tab, expectedUrl }) =>
    it(`'${tab}' links to page ${expectedUrl}`, () => {
      cy.visit(page);
      cy.get("a.active").as("active-tab");
      cy.get("a.inactive").contains(tab).as("target-tab");

      cy.get("@active-tab").should("not.contain", tab);
      cy.get("@target-tab").click();
      cy.url().should("contain", expectedUrl);
      cy.get("@active-tab").should("contain", tab);
    }),
  );
});
