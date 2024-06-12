import {
  DEFAULT_ACCOUNT_PK,
  FIRST_BLOCK_WITH_SNARK_WORK,
  GENESIS_BLOCK_BLOCK_HASH,
} from "../constants";

suite(["@tier1"], "block spotlight user commands", () => {
  let pages = [`/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/commands/user`];
  let columns = ["Hash", "Type", "From", "To", "Fee", "Amount"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("User Commands", columns);
    }),
  );
});

suite(["@tier1"], "staking ledger table", () => {
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
      cy.tableHasOrderedColumns("Staking Ledger - Epoch 1", columns);
    }),
  );
});

suite(["@tier1"], "accounts table", () => {
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

suite(["@tier1"], "transactions table", () => {
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

suite(["@tier1"], "internal commands table", () => {
  let pages = ["/commands/internal"];
  let columns = ["Height", "State Hash", "Recipient", "Fee", "Type", "Age"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Internal Commands", columns);
    }),
  );
});

suite(["@tier1"], "account transactions table", () => {
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

suite(["@tier1"], "account activity transactions", () => {
  let pages = [`/blocks/accounts/${DEFAULT_ACCOUNT_PK}`];
  let columns = ["Hash", "Direction", "Counterparty", "Amount/Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("User Commands", columns, "h2");
    }),
  );
});

suite(["@tier1"], "blocks table", () => {
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

suite(["@tier1"], "snarks table", () => {
  let pages = ["/snarks"];
  let columns = ["Height", "State Hash", "Age", "Prover", "Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("SNARKs", columns);
    }),
  );
});

suite(["@tier1"], "block spotlight snarks table", () => {
  let pages = [`/blocks/${FIRST_BLOCK_WITH_SNARK_WORK}/snark-jobs`];
  let columns = ["State Hash", "Age", "Prover", "Fee"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("SNARK Jobs", columns);
    }),
  );
});

suite(["@tier1"], "internal commands table", () => {
  let pages = [`/blocks/${GENESIS_BLOCK_BLOCK_HASH}/commands/internal`];
  let columns = ["Recipient", "Fee", "Type"];

  pages.forEach((page) =>
    it(`on ${page} includes correct columns`, () => {
      cy.visit(page);
      cy.tableHasOrderedColumns("Internal Commands", columns);
    }),
  );
});
