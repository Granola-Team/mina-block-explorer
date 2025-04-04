import {
  DEFAULT_ACCOUNT_PK,
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_BLOCK_WITH_SNARK_WORK,
} from "../constants";
suite(["@tier2"], "account page", () => {
  let pages = [
    // TODO: Enable when there is some data to display
    // {
    //   origin: `/analytics/snarker-leaderboard`,
    //   column: "Public Key",
    //   tableHeader: "Snarker Leaderboard",
    // },
    // TODO: no staking data to display ATM
    {
      origin: `/analytics/staker-leaderboard`,
      column: "Public Key",
      tableHeader: "Staker Leaderboard",
    },
    {
      origin: `/addresses/accounts`,
      column: "Public Key",
      tableHeader: "MINA Accounts",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      column: "Counterparty",
      tableHeader: "User Commands",
    },
    {
      origin: `/addresses/accounts/${FIRST_BLOCK_PRODUCER_ADDRESS}/block-production`,
      column: "Coinbase Receiver",
      tableHeader: "Block Production",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/delegations`,
      column: "Public Key",
      tableHeader: "Delegations",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/delegations`,
      column: "Username",
      tableHeader: "Delegations",
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
        cy.waitUntilTableLoads(tableHeader);
        if (transposed) {
          cy.clickLinkInTransposedTable(column, tableHeader, tableHeaderEl);
        } else {
          cy.clickLinkInTable(0, column, tableHeader, tableHeaderEl);
        }
        cy.url().should("include", "/accounts/");
        // to avoid errors
        cy.wait(150);
      }),
  );
});
