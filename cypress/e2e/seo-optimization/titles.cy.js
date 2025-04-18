import { FIRST_TXN_HASH, ROMEK_ADDRESS, ROMEK_USERNAME } from "../constants";
describe("meta title", () => {
  let block_hash = "3NLhBh3d4b91DPoJn5hhwRAWmHSAaG8Qz4W5r9FhJBCXLD3WrAt4";
  let pages = [
    {
      url: "/analytics/staker-leaderboard",
      title: "Analytics | Staker Leaderboard",
    },
    {
      url: "/analytics/snarker-leaderboard",
      title: "Analytics | Snarker Leaderboard",
    },
    {
      url: "/addresses/accounts",
      title: "Mina Addresses | Search for Mina accounts on Mina Blockchain",
    },
    {
      url: `/analytics/commands/user`,
      title: `Analytics | User Commands`,
    },
    {
      url: `/analytics/blocks`,
      title: `Analytics | Blocks`,
    },
    {
      url: `/analytics/snarks`,
      title: `Analytics | SNARKs`,
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
      url: "/staking-ledgers?epoch=1",
      title: "Staking Ledger | Epoch 1",
    },
    { url: "/snarks", title: "SNARKs | Search For SNARKs" },
    {
      url: `/commands/${FIRST_TXN_HASH}`,
      title: "Transaction Overview | No Memo",
    },
    {
      url: `/addresses/accounts/${ROMEK_ADDRESS}`,
      title: `Account Overview | ${ROMEK_USERNAME}`,
    },
  ];
  pages.forEach(({ title, url }) =>
    it(`'${title}' exists in <head> for page ${url}`, () => {
      cy.visit(url);
      cy.title().should("equal", title);
    }),
  );
});
