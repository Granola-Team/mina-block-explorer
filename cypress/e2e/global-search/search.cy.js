import { DEFAULT_ACCOUNT_PK, DEFAULT_CANONICAL_BLOCK_HASH } from "../constants";

suite(["@CI"], "global search", () => {
  let state_hash = "CkpYspKDV9mpSyZLczMYG8kr4CZYAXxXPH3VM9txYwRXdyyaCDfzL";

  let pages = [
    {
      input: DEFAULT_ACCOUNT_PK,
      expectedUrl: "/addresses/accounts/" + DEFAULT_ACCOUNT_PK,
    },
    {
      input: DEFAULT_CANONICAL_BLOCK_HASH,
      expectedUrl: "/blocks/" + DEFAULT_CANONICAL_BLOCK_HASH,
    },
    {
      input: state_hash,
      expectedUrl: "/commands/" + state_hash,
    },
    {
      input: "75",
      expectedUrl: "/staking-ledgers?epoch=75",
    },
  ];

  pages.forEach(({ input, expectedUrl }) =>
    it(`works for input ${input}`, () => {
      cy.visit("/");
      cy.get("input#searchbar").as("input");
      cy.get("@input").parent("form").as("form");

      cy.get("@input").type(input);
      cy.get("@form").submit();

      cy.url().should("include", expectedUrl);
    }),
  );
});
