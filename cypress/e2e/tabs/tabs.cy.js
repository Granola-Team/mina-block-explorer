suite(["@tier1"], "tab", () => {
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
