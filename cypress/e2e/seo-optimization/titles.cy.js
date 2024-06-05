import { DEFAULT_ACCOUNT_PK, FIRST_TXN_HASH } from "../constants";

suite(["@CI"], "meta title", () => {
  let block_hash = "3NLhBh3d4b91DPoJn5hhwRAWmHSAaG8Qz4W5r9FhJBCXLD3WrAt4";
  let pages = [
    {
      url: "/addresses/accounts",
      title: "Accounts | Search for accounts on Mina Blockchain",
    },
    {
      url: `/analytics/commands/internal`,
      title: `Analytics | Internal Commands`,
    },
    {
      url: `/commands/internal`,
      title: `Transactions | Internal Commands`,
    },
    { url: "/blocks", title: "Blocks | Search for blocks on Mina Blockchain" },
    { url: `/blocks/${block_hash}/`, title: "Block Overview | Spotlight" },
    {
      url: `/blocks/${block_hash}/commands/user`,
      title: "Block Overview | User Commands",
    },
    {
      url: `/blocks/${block_hash}/commands/internal`,
      title: "Block Overview | Internal Commands",
    },
    {
      url: `/blocks/${block_hash}/snark-jobs`,
      title: "Block Overview | SNARK Jobs",
    },
    {
      url: "/broadcast/transaction",
      title: "Offline Broadcasting | Broadcast Signed Transaction",
    },
    {
      url: "/broadcast/delegation",
      title: "Offline Broadcasting | Broadcast Signed Delegation",
    },
    {
      url: "/broadcast/ledger",
      title: "Offline Broadcasting | Broadcast Signed Transaction From Ledger",
    },
    {
      url: "/staking-ledgers?epoch=0",
      title: "Staking Ledger | Epoch 0",
    },
    { url: "/staking-ledgers?epoch=0", title: "Staking Ledger | Epoch 0" },
    { url: "/snarks", title: "SNARKs | Search For SNARKs" },
    {
      url: `/commands/${FIRST_TXN_HASH}`,
      title: "Transaction Overview | No Memo",
    },
    {
      url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      title: `Account Overview | ''`,
    },
  ];

  pages.forEach(({ title, url }) =>
    it(`'${title}' exists in <head> for page ${url}`, () => {
      cy.visit(url);
      cy.title({ timeout: 20000 }).should("equal", title);
    }),
  );
});
