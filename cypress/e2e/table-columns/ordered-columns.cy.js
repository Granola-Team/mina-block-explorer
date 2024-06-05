import {
  DEFAULT_ACCOUNT_PK,
  FIRST_BLOCK_WITH_SNARK_WORK,
  GENESIS_BLOCK_BLOCK_HASH,
} from "../constants";

suite(["@CI"], "staking ledger table", () => {
  let pages = ["/staking-ledgers?epoch=1"];
  let columns = [
    "Key",
    "Username",
    "Stake",
    "Total Stake %",
    "Block Win %",
    "Delegate",
    "Delegators",
  ];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Staking Ledger - Epoch 0", columns);
    }),
  );
});

suite(["@CI"], "accounts table", () => {
  let pages = ["/addresses/accounts"];
  let columns = [
    "Public Key",
    "Username",
    "Balance",
    "Nonce",
    "Delegate",
    "Time Locked",
  ];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Accounts", columns);
    }),
  );
});

suite(["@CI"], "transactions table", () => {
  let pages = ["/commands/user"];
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
  let pages = ["/commands/internal"];
  let columns = ["Height", "State Hash", "Recipient", "Fee", "Type", "Age"];

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
  let pages = [`/blocks/accounts/${DEFAULT_ACCOUNT_PK}`];
  let columns = ["Hash", "Direction", "Counterparty", "Amount/Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("User Commands", columns, "h2");
    }),
  );
});

suite(["@CI"], "blocks table", () => {
  let pages = ["/blocks"];
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
  let pages = [`/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/snark-jobs`];
  let columns = ["State Hash", "Age", "Prover", "Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("SNARK Jobs", columns);
    }),
  );
});

suite(["@CI"], "internal commands table", () => {
  let pages = [`/blocks/${GENESIS_BLOCK_BLOCK_HASH}/commands/internal`];
  let columns = ["Recipient", "Fee", "Type"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Internal Commands", columns);
    }),
  );
});
