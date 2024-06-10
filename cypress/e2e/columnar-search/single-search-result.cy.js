import {
  DEFAULT_ACCOUNT_PK,
  FIRST_INTERNAL_TXN_HASH,
  FIRST_TXN_HASH,
  GENESIS_BLOCK_BLOCK_HASH,
} from "../constants";

import { kebabCase } from "../helpers";

let state_hash = "3NKxUy4mRpuH7MJxFQEobEJbUhPyvDyMEBQywmTRLbWsaHto3nur";

suite(["@tier1"], "search with single result", () => {
  let exact_searches = [
    {
      origin: `/addresses/accounts`,
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Accounts",
      column: "Public Key",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: state_hash,
      tableHeading: "Block Production",
      column: "State Hash",
    },
    {
      origin: "/staking-ledgers?epoch=1",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Staking Ledger",
      column: "Key",
    },
    {
      origin: "/commands/user",
      input: FIRST_TXN_HASH,
      tableHeading: "User Commands",
      column: "Txn Hash",
    },
    {
      origin: "/blocks",
      input: GENESIS_BLOCK_BLOCK_HASH,
      tableHeading: "Blocks",
      column: "State Hash",
    },
    {
      origin: "/commands/internal",
      input: FIRST_INTERNAL_TXN_HASH,
      tableHeading: "Internal Commands",
      column: "State Hash",
    },
  ];

  exact_searches.forEach(({ origin, input, tableHeading, column }) =>
    it(`works on ${origin} page when searching column '${column}'`, () => {
      /*
        Sufficiently "tall" viewport to display many rows per table.
        We want to see that the search bar is filtering results.
      */
      cy.viewport(768, 2000);
      cy.visit(origin);
      cy.wait(500);
      cy.get(".loading-placeholder").should("not.exist");
      let key = "q-" + kebabCase(column);
      let cssSelector = "#" + key;

      // store initial length of table rows
      cy.aliasTableRows(tableHeading, "table-rows");
      cy.get("@table-rows").then(($trs) => {
        const initialLength = $trs.length;

        cy.wait(500);
        cy.get(cssSelector).as("searchinput");
        cy.get("@searchinput").type(input, { delay: 0 });

        // check input
        cy.get("@searchinput").should("have.value", input);
        // check url
        cy.url().should("include", `${key}=${input}`);

        // check table
        cy.get("@table-rows").should("have.length", 1);
        cy.wait(1000);

        cy.go("back");

        // check url
        cy.url().should("not.contain", key);
        // check table
        cy.tableHasMoreThanNRows(tableHeading, initialLength - 1);
      });
    }),
  );
});
