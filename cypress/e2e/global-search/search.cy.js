import { DEFAULT_ACCOUNT_PK, GENESIS_BLOCK_BLOCK_HASH } from "../constants";

suite(["@tier2"], "global search", () => {
  it("has visible placeholder text", () => {
    cy.viewport("iphone-xr");
    cy.visit("/");
    cy.get('input[placeholder="Paste -> Enter -> Explore!"]').should(
      "be.visible",
    );
  });

  let state_hash = "CkpYspKDV9mpSyZLczMYG8kr4CZYAXxXPH3VM9txYwRXdyyaCDfzL";

  let pages = [
    {
      input: 30000,
      expectedUrl: "/blocks?q-height=30000",
    },
    {
      input: "      jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee     ",
      expectedUrl: "/staking-ledgers?epoch=0",
    },
    {
      input: "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
      expectedUrl: "/staking-ledgers?epoch=0",
    },
    {
      input: DEFAULT_ACCOUNT_PK,
      expectedUrl: "/addresses/accounts/" + DEFAULT_ACCOUNT_PK,
    },
    {
      input: GENESIS_BLOCK_BLOCK_HASH,
      expectedUrl: "/blocks/" + GENESIS_BLOCK_BLOCK_HASH,
    },
    {
      input: state_hash,
      expectedUrl: "/commands/" + state_hash,
    },
    {
      input: "1",
      expectedUrl: "/staking-ledgers?epoch=1",
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
      cy.get("@input").should("not.have.value", input);
    }),
  );
});
