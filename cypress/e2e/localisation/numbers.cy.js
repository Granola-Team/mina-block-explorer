import {
  DEFAULT_LOCALE,
  GENESIS_BLOCK_BLOCK_HASH,
  ROMEK_ADDRESS,
  BLOCK_WITH_ALL_ACTIVITY,
  COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS,
} from "../constants";
import { parseFormattedNumber } from "../helpers";
let pages = [
  {
    page: "/tokens",
    wait: () => {
      cy.intercept("GET", "/summary").as("summaryData");
      cy.wait("@summaryData");
      cy.aliasTableRows("Tokens", "table-rows");
      cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "supply column",
        selector: () => cy.get("@table-rows").first().find("td").eq(1),
        type: "number",
      },
    ],
  },
  {
    page: "/blocks",
    wait: () => {
      cy.intercept("GET", "/summary").as("summaryData");
      cy.wait("@summaryData");
      cy.aliasTableRows("Blocks", "table-rows");
      cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "overview",
        selector: () => cy.get("#blockchainLength"),
        type: "number",
      },
      {
        name: "height column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "slot column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(2);
        },
        type: "number",
      },
    ],
  },
  {
    page: `/blocks/${GENESIS_BLOCK_BLOCK_HASH}/spotlight`,
    tests: [
      {
        name: "coinbase",
        selector: () => {
          cy.aliasTransposedTableRows("Block Spotlight", "spotlight");
          return cy
            .get("@spotlight")
            .contains("Coinbase")
            .siblings("td")
            .first();
        },
        type: "currency",
      },
      {
        name: "SNARK Fees",
        selector: () => {
          cy.aliasTransposedTableRows("Block Spotlight", "spotlight");
          return cy
            .get("@spotlight")
            .contains("SNARK Fees")
            .siblings("td")
            .first();
        },
        type: "currency",
      },
      {
        name: "Transaction Fees",
        selector: () => {
          cy.aliasTransposedTableRows("Block Spotlight", "spotlight");
          return cy
            .get("@spotlight")
            .contains("Transaction Fees")
            .siblings("td")
            .first();
        },
        type: "currency",
      },
    ],
  },
  {
    page: `/blocks/${BLOCK_WITH_ALL_ACTIVITY}/commands/user`,
    wait: () => {
      cy.intercept("GET", "/summary").as("summaryData");
      cy.wait("@summaryData");
      cy.aliasTableRows("User Commands", "table-rows");
    },
    tests: [
      {
        name: "fee",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(6);
        },
        type: "currency",
      },
      {
        name: "amount",
        selector: () => {
          return cy.get("@table-rows").first().find("td").last();
        },
        type: "currency",
      },
    ],
  },
  {
    page: `/blocks/${BLOCK_WITH_ALL_ACTIVITY}/commands/internal`,
    wait: () => {
      cy.aliasTableRows("Internal Commands", "table-rows");
    },
    tests: [
      {
        name: "fee",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(1);
        },
        type: "currency",
      },
    ],
  },
  {
    page: `/blocks/${BLOCK_WITH_ALL_ACTIVITY}/snark-jobs`,
    wait: () => {
      cy.aliasTableRows("SNARK Jobs", "table-rows");
    },
    tests: [
      {
        name: "fee",
        selector: () => {
          return cy.get("@table-rows").first().find("td").last(0);
        },
        type: "currency",
      },
    ],
  },
  //
  {
    page: "/commands/user",
    wait: () => {
      cy.aliasTableRows("User Commands", "table-rows");
      cy.wait(100);
      cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "height column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "nonce column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(7);
        },
        type: "number",
      },
      {
        name: "fee column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(8);
        },
        type: "currency",
      },
      {
        name: "amount column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(9);
        },
        type: "currency",
      },
    ],
  },
  {
    page: "/commands/internal",
    wait: () => {
      cy.aliasTableRows("Internal Commands", "table-rows");
      cy.wait(100);
      cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "height column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "fee column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(3);
        },
        type: "currency",
      },
    ],
  },
  {
    page: "/addresses/accounts",
    wait: () => {
      cy.aliasTableRows("MINA Accounts", "table-rows");
      cy.wait(100);
      cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
      cy.get("th").contains("Balance").find("input").as("input");
      cy.get("@input").type("4000", { delay: 0 });
      cy.wait(1000);
      cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
      cy.aliasTableRows("MINA Accounts", "table-rows");
    },
    tests: [
      {
        name: "balance column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(3);
        },
        type: "currency",
      },
      {
        name: "nonce column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(4);
        },
        type: "number",
      },
    ],
  },
  {
    page: `/addresses/accounts/${COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS}`,
    wait: () => {
      cy.aliasTableRows("User Commands", "uc-table-rows");
      cy.wait(100);
      cy.get(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "user command height column",
        selector: () => {
          return cy.get("@uc-table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "user command nonce column",
        selector: () => {
          return cy.get("@uc-table-rows").first().find("td").eq(2);
        },
        type: "number",
      },
      {
        name: "user command amount column",
        selector: () => {
          return cy
            .get("@uc-table-rows")
            .first()
            .find("td")
            .last()
            .children("span")
            .children("span")
            .first();
        },
        type: "currency",
      },
      {
        name: "user command fee column",
        selector: () => {
          return cy
            .get("@uc-table-rows")
            .first()
            .find("td")
            .last()
            .children("span")
            .children("span")
            .last();
        },
        type: "currency",
      },
    ],
  },
  {
    page: `/addresses/accounts/${COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS}/snark-jobs`,
    wait: () => {
      cy.aliasTableRows("SNARK Jobs", "sj-table-rows");
      cy.wait(100);
      cy.get(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "snark work height column",
        selector: () => {
          return cy.get("@sj-table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "snark work fee column",
        selector: () => {
          return cy.get("@sj-table-rows").first().find("td").last();
        },
        type: "currency",
      },
    ],
  },
  {
    page: `/addresses/accounts/${COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS}/block-production`,
    wait: () => {
      cy.aliasTableRows("Block Production", "bp-table-rows");
      cy.wait(100);
      cy.get(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "block produciton height column",
        selector: () => {
          return cy.get("@bp-table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "block produciton slot column",
        selector: () => {
          return cy.get("@bp-table-rows").first().find("td").eq(2);
        },
        type: "number",
      },
      {
        name: "block produciton coinbase column",
        selector: () => {
          return cy.get("@bp-table-rows").first().find("td").eq(5);
        },
        type: "currency",
      },
    ],
  },
  {
    page: "/snarks",
    wait: () => {
      cy.aliasTableRows("SNARKs", "table-rows");
      cy.wait(100);
      cy.get(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "height column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").first();
        },
        type: "number",
      },
      {
        name: "fee column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").last();
        },
        type: "currency",
      },
    ],
  },
  {
    page: `/staking-ledgers?epoch=0&q-key=${ROMEK_ADDRESS}`,
    wait: () => {
      cy.aliasTableRows("Staking Ledger - Epoch 0", "table-rows");
      cy.get(".loading-placeholder").should("not.exist");
    },
    tests: [
      {
        name: "stake column",
        selector: () => {
          return cy.get("@table-rows").first().find("td").eq(2);
        },
        type: "currency",
      },
    ],
  },
];
pages.forEach(({ tests, page, wait = () => {} }) => {
  describe("number or currency", () => {
    it(`on page ${page} is formatted correctly for '${tests.map((t) => t.name).join("', '")}'`, () => {
      cy.visit(page);
      wait();
      tests.forEach(({ selector, type }) => {
        selector()
          .invoke("text")
          .then((text) => {
            let number = parseFormattedNumber(text.trim());
            expect(number).to.be.a("number");
            let options =
              type === "number"
                ? {}
                : { minimumFractionDigits: 1, maximumFractionDigits: 9 };
            const formatter = new Intl.NumberFormat(DEFAULT_LOCALE, options);
            expect(text).to.contain(formatter.format(number));
          });
      });
    });
  });
});
