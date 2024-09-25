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
      expected_row_count: 2,
    },
    {
      url: `/blocks/${VETAL_BLOCK_STATE_HASH}/snark-jobs`,
      tab: "SNARK Jobs",
      expected_row_count: 3,
    },
    {
      url: `/blocks/${VETAL_BLOCK_STATE_HASH}/commands/internal`,
      tab: "Internal Commands",
      expected_row_count: 2,
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/commands/user`,
      tab: "User Commands",
      expected_row_count: 29,
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/block-production`,
      tab: "Block Production",
      expected_row_count: 25,
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/delegations`,
      tab: "Delegations",
      expected_row_count: 5,
    },
    {
      url: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}/snark-jobs`,
      tab: "SNARK Jobs",
      expected_row_count: 25,
    },
    {
      url: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}/commands/internal`,
      tab: "Internal Commands",
      expected_row_count: 2,
    },
  ];

  tabs.forEach(({ url, tab, expected_row_count }) =>
    it(`matches row count on tab '${tab}' at ${url}`, () => {
      cy.visit(url);

      cy.contains("a.tab", tab).find(".number-bubble").as("bubble");
      cy.wait(100);
      cy.get("@bubble").should("not.equal", 0);

      cy.get("@bubble")
        .invoke("text")
        .then(($value) => {
          let num = parseInt($value);
          // tabs may dipslay total count from indexer and may be greater than rendered amount
          expect(num).to.be.at.least(expected_row_count);
        });

      cy.aliasTableRows(tab, "tr");
      cy.get("@tr").should("have.lengthOf", expected_row_count);
    }),
  );
});
