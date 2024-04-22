import { DEFAULT_CANONICAL_BLOCK_HASH } from "../constants";

suite(["@CI"], "table of dynamic height", () => {
  Cypress.on("uncaught:exception", (err, runnable) => {
    if (err.message.includes("ResizeObserver")) {
      return false;
    }
  });
  let pages = [
    { url: "/", tableHeading: "Blocks" },
    { url: "/blocks", tableHeading: "Blocks" },
    {
      url: `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/user-commands`,
      tableHeading: "User Commands",
    },
    {
      url: `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/snark-jobs`,
      tableHeading: "SNARK Jobs",
    },
    {
      url: `/blocks/${DEFAULT_CANONICAL_BLOCK_HASH}/internal-commands`,
      tableHeading: "Internal Commands",
    },

    { url: "/commands/user-commands", tableHeading: "User Commands" },

    {
      url: "/commands/internal-commands",
      tableHeading: "Internal Commands",
    },

    { url: "/snarks", tableHeading: "SNARKs" },
    { url: "/stakes", tableHeading: "Current Staking Ledger" },
    { url: "/next-stakes", tableHeading: "Next Staking Ledger" },
  ];

  pages.forEach(({ url, tableHeading }) =>
    it(`on ${url} shows greater than 10 records`, () => {
      cy.viewport(768, 2000);
      cy.visit(url);
      cy.wait(500);
      cy.aliasTablePagination(tableHeading, "pag");
      cy.get("@pag")
        .find("span")
        .first()
        .invoke("text")
        .then((text) => {
          let rowCount = parseInt(extractRows(text));
          cy.log(rowCount, text);
          expect(rowCount).to.be.gt(10);
        });
    }),
  );
});

function extractRows(input) {
  const regex = /Showing 1 to (\d+) of (\d+) records/;
  const match = input.match(regex);
  return match ? match[1] : null;
}
