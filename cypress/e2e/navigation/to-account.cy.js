import { DEFAULT_ACCOUNT_PK, FIRST_BLOCK_WITH_SNARK_WORK } from "../constants";

suite(["@tier2"], "account page", () => {
  let pages = [
    {
      origin: `/addresses/accounts`,
      column: "Public Key",
      tableHeader: "Accounts",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      column: "Counterparty",
      tableHeader: "User Commands",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/block-production`,
      column: "Coinbase Receiver",
      tableHeader: "Block Production",
    },
    { origin: "/snarks", column: "Prover", tableHeader: "SNARKs" },
    {
      origin: "/staking-ledgers?epoch=1",
      column: "Key",
      tableHeader: "Staking Ledger",
    },
    {
      origin: "/staking-ledgers?epoch=1",
      column: "Delegate",
      tableHeader: "Staking Ledger",
    },
    {
      origin: "/commands/user",
      column: "From",
      tableHeader: "User Commands",
    },
    {
      origin: "/commands/user",
      column: "To",
      tableHeader: "User Commands",
    },
    {
      origin: `/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/commands/user`,
      column: "From",
      tableHeader: "User Commands",
    },
    {
      origin: `/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/commands/user`,
      column: "To",
      tableHeader: "User Commands",
    },
    {
      origin: `/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/snark-jobs`,
      column: "Prover",
      tableHeader: "SNARK Jobs",
    },
    {
      origin: `/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/commands/internal`,
      column: "Recipient",
      tableHeader: "Internal Commands",
    },
  ];

  pages.forEach(
    ({ origin, column, tableHeader, tableHeaderEl = "h1", transposed }) =>
      it(`is navigated to from ${origin} by clicking link in '${column}'`, () => {
        cy.visit(origin);
        if (transposed) {
          cy.clickLinkInTransposedTable(column, tableHeader, tableHeaderEl);
        } else {
          cy.clickLinkInTable(1, column, tableHeader, tableHeaderEl);
        }

        cy.url().should("include", "/accounts/");
      }),
  );
});
