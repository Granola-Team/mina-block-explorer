import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "transactions table", () => {
  let pages = ["/commands/user-commands"];
  let columns = [
    "Height",
    "Txn Hash",
    "Age",
    "Type",
    "From",
    "To",
    "Nonce",
    "Fee",
    "Amount",
  ];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("User Commands", columns);
    }),
  );
});

suite(["@CI"], "internal commands table", () => {
  let pages = ["/commands/internal-commands"];
  let columns = ["Recipient", "Fee", "Type", "Age"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Internal Commands", columns);
    }),
  );
});

suite([""], "account transactions table", () => {
  let pages = [`/addresses/accounts/${DEFAULT_ACCOUNT_PK}`];
  let columns = [
    "Height",
    "Txn Hash",
    "Nonce",
    "Age",
    "Type",
    "Direction",
    "Counterparty",
    "Amount/Fee",
  ];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.wait(1000);
      cy.tableHasOrderedColumns("User Commands", columns);
    }),
  );
});

suite(["@CI"], "account activity transactions", () => {
  let pages = [`/summary/accounts/${DEFAULT_ACCOUNT_PK}`];
  let columns = ["Hash", "Direction", "Counterparty", "Amount/Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("User Commands", columns, "h2");
    }),
  );
});

suite(["@CI"], "blocks table", () => {
  let pages = ["/blocks", "/summary", "/"];
  let columns = [
    "Height",
    "State Hash",
    "Slot",
    "Age",
    "Block Producer",
    "Coinbase",
    "User Commands",
    "SNARKs",
    "Coinbase Receiver",
  ];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Blocks", columns);
    }),
  );
});

suite(["@CI"], "snarks table", () => {
  let pages = ["/snarks"];
  let columns = ["Height", "State Hash", "Age", "Prover", "Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("SNARKs", columns);
    }),
  );
});

suite(["@CI"], "block spotlight snarks table", () => {
  let pages = [
    "/blocks/3NKLE73AnqCKVit9h3yEZsPbbJBmVfW5WWKA6pNsUjqh3Nm1mKSK/snark-jobs",
  ];
  let columns = ["State Hash", "Age", "Prover", "Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("SNARK Jobs", columns);
    }),
  );
});

suite(["@CI"], "internal commands table", () => {
  let pages = [
    "/blocks/3NKyujsdi2GtWA1XC9KJ6nvXeLAd3DNvYrm1PUGEagj9899s1LMz/internal-commands",
  ];
  let columns = ["Recipient", "Fee", "Type"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Internal Commands", columns);
    }),
  );
});
