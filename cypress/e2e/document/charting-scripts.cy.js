import { GENESIS_BLOCK_BLOCK_HASH } from "../constants";
describe("<head>", () => {
  let sample_non_charting_pages = ["/", "/blocks"];
  let charting_pages = [`/blocks/${GENESIS_BLOCK_BLOCK_HASH}/analytics`];
  sample_non_charting_pages.forEach((page) =>
    it(`does not contain charting libraries on non-charting page ${page}`, () => {
      cy.visit(page);
      cy.get('head script[src*="echarts"]').should("not.exist");
    }),
  );
  charting_pages.forEach((page) =>
    it(`contains charting libraries on ${page}`, () => {
      cy.visit(page);
      cy.get('head script[src*="echarts"]').should("exist");
    }),
  );
});
