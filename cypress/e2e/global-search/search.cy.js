import {
  GENESIS_ACCOUNT_PK,
  EPOCH_ZERO_STAKING_LEDGER_HASH,
  FIRST_TXN_HASH,
  GENESIS_BLOCK_BLOCK_HASH,
} from "../constants";
suite(["@tier2"], "global search", () => {
  it("has visible placeholder text", () => {
    cy.viewport("iphone-xr");
    cy.visit("/");
    cy.get('input[placeholder="Paste -> Enter -> Explore!"]').should(
      "be.visible",
    );
  });
  let pages = [
    {
      input: 359617,
      expectedUrl: "/blocks?q-height=359617",
    },
    {
      input: `      ${EPOCH_ZERO_STAKING_LEDGER_HASH}     `,
      expectedUrl: "/staking-ledgers?epoch=0",
    },
    {
      input: EPOCH_ZERO_STAKING_LEDGER_HASH,
      expectedUrl: "/staking-ledgers?epoch=0",
    },
    {
      input: GENESIS_ACCOUNT_PK,
      expectedUrl: "/addresses/accounts/" + GENESIS_ACCOUNT_PK,
    },
    {
      input: GENESIS_BLOCK_BLOCK_HASH,
      expectedUrl: "/blocks/" + GENESIS_BLOCK_BLOCK_HASH,
    },
    {
      input: FIRST_TXN_HASH,
      expectedUrl: "/commands/" + FIRST_TXN_HASH,
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
