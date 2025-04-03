import {
  ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
  DEFAULT_ACCOUNT_PK,
  VETAL_BLOCK_STATE_HASH,
} from "../constants";
suite(["@tier2"], "number bubble in tab", () => {
  let tabs = [
    {
      url: `/blocks/${VETAL_BLOCK_STATE_HASH}/commands/user`,
      tab: "User Commands",
      comparison_method: "rows",
    },
    {
      url: `/blocks/${VETAL_BLOCK_STATE_HASH}/snark-jobs`,
      tab: "SNARK Jobs",
      comparison_method: "rows",
    },
    {
      url: `/blocks/${VETAL_BLOCK_STATE_HASH}/commands/internal`,
      tab: "Internal Commands",
      comparison_method: "rows",
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/commands/user`,
      tab: "User Commands",
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/block-production`,
      tab: "Block Production",
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/delegations`,
      tab: "Delegations",
    },
    // {
    //   url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/tokens`,
    //   tab: "Tokens",
    // },
    {
      url: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}/snark-jobs`,
      tab: "SNARK Jobs",
    },
    {
      url: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}/commands/internal`,
      tab: "Internal Commands",
    },
  ];
  tabs.forEach(({ url, tab, comparison_method }) =>
    it(`matches row count on tab '${tab}' at ${url}`, () => {
      cy.visit(url);
      // There are a few graphql resources that load on this page; wait
      // for them all to complete
      cy.intercept("POST", "/graphql").as("graphql");
      cy.wait("@graphql");
      cy.intercept("POST", "/graphql").as("graphql");
      cy.wait("@graphql");
      cy.wait(250);
      cy.contains("a.tab", tab).find(".number-bubble").as("bubble");
      cy.get("@bubble")
        .invoke("text")
        .then((text) => {
          let number = parseInt(text);
          if (comparison_method == "rows") {
            cy.aliasTableRows(tab, "tr");
            cy.get("@tr").should("have.lengthOf", number);
          } else {
            cy.aliasTableMetadata(tab);
            cy.assertTableMetadataCorrect(tab, number, 1);
          }
        });
    }),
  );
});
